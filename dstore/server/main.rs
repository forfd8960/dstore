use anyhow::Result;
use bytes::Bytes;
use prost::Message;
use tokio::{io::AsyncWriteExt, net::TcpListener};

use dstore::{
    pb::pb::{self as dspb, CommandResponse},
    service::StoreServer,
};

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

        tokio::spawn(async move {
            let mut buf = [0; 4096];
            match tcp_stream.try_read(&mut buf) {
                Ok(0) => {}
                Ok(n) => {
                    println!("read {} bytes", n);
                    let data = Bytes::copy_from_slice(&buf[0..n]);
                    let req_data = dspb::CommondRequest::decode(data);
                    match req_data {
                        Ok(req) => {
                            println!("decode cmd: {:?}", req);

                            let resp = server_clone.dispatch(req.request_data.unwrap());
                            match resp {
                                Ok(res) => {
                                    let res_bs = res.encode_to_vec();
                                    let _ = tcp_stream.write_all(res_bs.as_slice()).await;
                                }
                                Err(kv_error) => {
                                    let resp = CommandResponse {
                                        status: 400,
                                        message: kv_error.to_string(),
                                        pairs: vec![],
                                        values: vec![],
                                    };
                                    let _ =
                                        tcp_stream.write_all(resp.encode_to_vec().as_slice()).await;
                                }
                            }
                        }
                        Err(e) => {
                            println!("failed to decode request: {}", e);
                        }
                    }
                }
                Err(ref e) => {
                    println!("read data err: {}", e);
                }
            }
        });
    }
}
