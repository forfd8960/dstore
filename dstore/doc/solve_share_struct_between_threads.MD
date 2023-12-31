## Before

define a struct and used in tokio::spawn task:

- service/mod.rs

```rust

pub struct StoreServer {
    data_store: Arc<dyn Storage>,
}

impl Clone for StoreServer {
    fn clone(&self) -> Self {
        Self {
            data_store: Arc::clone(&self.data_store),
        }
    }
}
```

- server/main.rs

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let addr: &str = "127.0.0.1:8989";
    let listener = TcpListener::bind(addr).await?;
    println!("start listen on: {}", addr);

    let store_server = StoreServer::new();

    loop {
        let (mut tcp_stream, addr) = listener.accept().await?;
        println!("client: {} connected", addr);

        let _ = tcp_stream.readable().await;
        let server_clone = store_server.clone();

        ...
        tokio::spawn(async move {
            ...
             let resp = server_clone.dispatch(req.request_data.unwrap());
             ...
        })

```

Then get the error: `trait Sync, Send not implemented for dyn Storage + 'static`.

## How to solve it

refactor the StoreServer with

init the ServerInner with: `let data_store: Box<dyn Storage + Send + Sync> = Box::new(MemTable::new(100));`

```rust
pub struct StoreServer {
    server_inner: Arc<ServerInner>,
}

struct ServerInner {
    data_store: Box<dyn Storage + Send + Sync>,
}

impl StoreServer {
    pub fn new() -> Self {
        let data_store: Box<dyn Storage + Send + Sync> = Box::new(MemTable::new(100));
        Self {
            server_inner: Arc::new(ServerInner {
                data_store: data_store,
            }),
        }
    }
    ...
}

impl Clone for StoreServer {
    fn clone(&self) -> Self {
        Self {
            server_inner: Arc::clone(&self.server_inner),
        }
    }
}
```

server/main.rs no need to change.

## Why it solved the issue

asked in devv.ai: `rust tokio trait Sync and Send is not implemented for dyn MyTrait`

```rust
# The error message you're seeing indicates that the future cannot be sent between threads safely, and it's related to the Send and Sync traits not being implemented for dyn MyTrait. Let's break down the reasons behind this and discuss the solution.

# The error message you're encountering is due to the fact that when you cast something as dyn MyTrait, you're removing all the original type information and replacing it with the type dyn MyTrait. This means that you lose the auto-implemented Send and Sync traits on MyStruct. The tokio::spawn function requires Send, and this issue isn't inherent to async programming; it's because tokio::spawn will run the future on its thread pool, possibly sending it to another thread.

# To address this issue, you can use the Box type to add additional trait bounds to the dyn MyTrait object. By boxing the object and adding the Send and Sync trait bounds, you can resolve the issue. Here's an example of how to modify your code to add the necessary trait bounds:


let object: Box<dyn MyTrait + Send + Sync> = Box::new(MyStruct::new());

# By using Box<dyn MyTrait + Send + Sync>, you ensure that the dyn MyTrait object is both Send and Sync, which satisfies the requirements of tokio::spawn.
```
