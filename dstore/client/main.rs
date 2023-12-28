use anyhow::Result;
use bytes::BytesMut;
use dstore::pb::pb;
use prost::Message;
use std::io::Write;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<()> {
    send_cmd().await
}

fn encode_get_cmd() -> Vec<u8> {
    let cmd = pb::CommondRequest {
        request_data: Some(pb::commond_request::RequestData::Get(pb::Get {
            key: "test-key".to_string(),
        })),
    };

    // let mut buf = [0, 4096];
    let bs = cmd.encode_to_vec();
    bs
}

fn encode_set_cmd() -> Vec<u8> {
    let cmd = pb::CommondRequest {
        request_data: Some(pb::commond_request::RequestData::Set(pb::Set {
            kv: Some(pb::Kv {
                key: "test-key".to_string(),
                value: "test-value".to_string(),
            }),
        })),
    };

    // let mut buf = [0, 4096];
    let bs = cmd.encode_to_vec();
    bs
}

async fn send_cmd() -> Result<()> {
    let addr = "127.0.0.1:8989";
    loop {
        print!(">>>");
        std::io::stdout().flush().unwrap();

        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            std::result::Result::Ok(_) => {
                let req = parse_cmd(buf);

                let mut stream = TcpStream::connect(addr).await?;
                let cmd = req.encode_to_vec();
                stream.write_all(&cmd).await.unwrap();
                println!("done write cmd: {:?}", cmd);

                let mut resp = BytesMut::with_capacity(4096);
                stream.read_buf(&mut resp).await.unwrap();
                println!("read response: {:?}", String::from_utf8(resp.to_vec()));
            }
            Err(e) => {
                println!("read line err: {:?}", e);
            }
        }
    }
}

fn parse_cmd(cmd: String) -> pb::CommondRequest {
    pb::CommondRequest {
        request_data: Some(pb::commond_request::RequestData::Get(pb::Get {
            key: "test-key".to_string(),
        })),
    }
}
