use anyhow::Result;
use bytes::BytesMut;
use dstore::pb as storepb;
use dstore::{errors::CmdError, pb::pb};
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

async fn send_cmd() -> Result<()> {
    let addr = "127.0.0.1:8989";
    loop {
        print!(">>>");
        std::io::stdout().flush().unwrap();

        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            std::result::Result::Ok(_) => {
                let req_res = parse_cmd(buf);
                match req_res {
                    Ok(req) => {
                        let mut stream = TcpStream::connect(addr).await?;
                        let cmd = req.encode_to_vec();
                        stream.write_all(&cmd).await.unwrap();
                        println!("done write cmd: {:?}", cmd);

                        let mut resp = BytesMut::with_capacity(4096);
                        stream.read_buf(&mut resp).await.unwrap();

                        let cmd_resp = pb::CommandResponse::decode(&mut resp);
                        match cmd_resp {
                            Ok(v) => {
                                println!("{}", v);
                            }
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("parse cmd err: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("read line err: {:?}", e);
            }
        }
    }
}

fn parse_cmd(cmd: String) -> Result<pb::CommondRequest, CmdError> {
    let args = Vec::from_iter(cmd.split(" ").filter(|x| x.trim() != "").map(|x| x.trim()));
    if args.len() < 1 {
        return Err(CmdError::InvalidCmdParameter(cmd));
    }

    let valid_err = storepb::validate(cmd.as_str(), args.clone());
    match valid_err {
        Some(e) => Err(e),
        None => match storepb::build_cmd_request(cmd.as_str(), args) {
            Some(req) => Ok(req),
            None => Err(CmdError::UnknownCmd(cmd)),
        },
    }
}
