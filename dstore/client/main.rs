use anyhow::Result;
use bytes::BytesMut;
use dstore::pb as storepb;
use dstore::{errors::CmdError, pb::pb, validator::CmdValidtor};
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

    match args[0] {
        storepb::GET_CMD => {
            let res = pb::Get::validate(args.clone());
            if res.is_some() {
                return Err(res.unwrap());
            }

            Ok(pb::CommondRequest {
                request_data: Some(pb::commond_request::RequestData::Get(pb::Get::from(args))),
            })
        }
        storepb::SET_CMD => {
            let res = pb::Set::validate(args.clone());
            if res.is_some() {
                return Err(res.unwrap());
            }

            Ok(pb::CommondRequest {
                request_data: Some(pb::commond_request::RequestData::Set(pb::Set::from(args))),
            })
        }
        storepb::HGET_CMD => {
            let res = pb::HGet::validate(args.clone());
            if res.is_some() {
                return Err(res.unwrap());
            }

            Ok(pb::CommondRequest {
                request_data: Some(pb::commond_request::RequestData::Hget(pb::HGet::from(args))),
            })
        }
        storepb::HSET_CMD => {
            let res = pb::HSet::validate(args.clone());
            if res.is_some() {
                return Err(res.unwrap());
            }

            Ok(pb::CommondRequest {
                request_data: Some(pb::commond_request::RequestData::Hset(pb::HSet::from(args))),
            })
        }
        storepb::SADD_CMD => {
            let res = pb::SAdd::validate(args.clone());
            if res.is_some() {
                return Err(res.unwrap());
            }

            Ok(pb::CommondRequest {
                request_data: Some(pb::commond_request::RequestData::Sadd(pb::SAdd::from(args))),
            })
        }
        storepb::SMEMBERS_CMD => {
            let res = pb::SMembers::validate(args.clone());
            if res.is_some() {
                return Err(res.unwrap());
            }

            Ok(pb::CommondRequest {
                request_data: Some(pb::commond_request::RequestData::Smembers(
                    pb::SMembers::from(args),
                )),
            })
        }

        storepb::LPUSH => Ok(pb::CommondRequest {
            request_data: Some(pb::commond_request::RequestData::Lpush(pb::LPush::from(
                args,
            ))),
        }),
        storepb::LPOP => Ok(pb::CommondRequest {
            request_data: Some(pb::commond_request::RequestData::Lpop(pb::LPop::from(args))),
        }),
        storepb::LRange => Ok(pb::CommondRequest {
            request_data: Some(pb::commond_request::RequestData::Lrange(pb::LRange::from(
                args,
            ))),
        }),
        _ => Err(CmdError::UnknownCmd(args[0].to_string())),
    }
}
