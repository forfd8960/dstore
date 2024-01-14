use crate::errors::CmdError;

pub mod pb;

use crate::pb::pb as dspb;
use crate::validator::CmdValidtor;

pub const GET_CMD: &str = "Get";
pub const SET_CMD: &str = "Set";
pub const HGET_CMD: &str = "HGet";
pub const HSET_CMD: &str = "HSet";
pub const SADD_CMD: &str = "SAdd";
pub const SMEMBERS_CMD: &str = "SMembers";
pub const LPUSH: &str = "LPush";
pub const LPOP: &str = "LPop";
pub const LRANGE: &str = "LRange";

impl std::fmt::Display for pb::CommandResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.pairs.len() > 0 {
            for kv in &self.pairs {
                let _ = writeln!(f, "{}: {}", kv.key, kv.value);
            }
            return write!(f, "{}", "");
        }

        if self.values.len() > 0 {
            for value in &self.values {
                let _ = writeln!(f, "{}", value.val);
            }
            return write!(f, "{}", "");
        }

        write!(f, "{}", self.message)
    }
}

pub trait CmdRequestBuild {
    fn build(cmds: Vec<&str>) -> Option<dspb::CommondRequest>;
}

pub fn validate(cmd: &str, args: Vec<&str>) -> Option<CmdError> {
    match cmd {
        GET_CMD => dspb::Get::validate(args),
        SET_CMD => dspb::Set::validate(args),
        HGET_CMD => dspb::HGet::validate(args),
        HSET_CMD => dspb::HSet::validate(args),
        SADD_CMD => dspb::SAdd::validate(args),
        SMEMBERS_CMD => dspb::SMembers::validate(args),
        LPUSH => dspb::LPush::validate(args),
        LPOP => dspb::LPop::validate(args),
        LRANGE => dspb::LRange::validate(args),
        _ => Some(CmdError::UnknownCmd(cmd.to_string())),
    }
}

pub fn build_cmd_request(cmd: &str, args: Vec<&str>) -> Option<dspb::CommondRequest> {
    match cmd {
        GET_CMD => dspb::Get::build(args),
        SET_CMD => dspb::Set::build(args),
        HGET_CMD => dspb::HGet::build(args),
        HSET_CMD => dspb::HSet::build(args),
        SADD_CMD => dspb::SAdd::build(args),
        SMEMBERS_CMD => dspb::SMembers::build(args),
        LPUSH => dspb::LPush::build(args),
        LPOP => dspb::LPop::build(args),
        LRANGE => dspb::LRange::build(args),
        _ => None,
    }
}

impl CmdRequestBuild for dspb::Get {
    fn build(cmds: Vec<&str>) -> Option<dspb::CommondRequest> {
        Some(dspb::CommondRequest {
            request_data: Some(dspb::commond_request::RequestData::Get(dspb::Get::from(
                cmds,
            ))),
        })
    }
}

impl CmdRequestBuild for dspb::Set {
    fn build(cmds: Vec<&str>) -> Option<dspb::CommondRequest> {
        Some(dspb::CommondRequest {
            request_data: Some(dspb::commond_request::RequestData::Set(dspb::Set::from(
                cmds,
            ))),
        })
    }
}

impl CmdRequestBuild for dspb::SAdd {
    fn build(cmds: Vec<&str>) -> Option<dspb::CommondRequest> {
        Some(dspb::CommondRequest {
            request_data: Some(dspb::commond_request::RequestData::Sadd(dspb::SAdd::from(
                cmds,
            ))),
        })
    }
}

impl CmdRequestBuild for dspb::SMembers {
    fn build(cmds: Vec<&str>) -> Option<dspb::CommondRequest> {
        Some(dspb::CommondRequest {
            request_data: Some(dspb::commond_request::RequestData::Smembers(
                dspb::SMembers::from(cmds),
            )),
        })
    }
}

impl CmdRequestBuild for dspb::HGet {
    fn build(cmds: Vec<&str>) -> Option<dspb::CommondRequest> {
        Some(dspb::CommondRequest {
            request_data: Some(dspb::commond_request::RequestData::Hget(dspb::HGet::from(
                cmds,
            ))),
        })
    }
}

impl CmdRequestBuild for dspb::HSet {
    fn build(cmds: Vec<&str>) -> Option<dspb::CommondRequest> {
        Some(dspb::CommondRequest {
            request_data: Some(dspb::commond_request::RequestData::Hset(dspb::HSet::from(
                cmds,
            ))),
        })
    }
}

impl CmdRequestBuild for dspb::LPush {
    fn build(cmds: Vec<&str>) -> Option<dspb::CommondRequest> {
        Some(dspb::CommondRequest {
            request_data: Some(dspb::commond_request::RequestData::Lpush(
                dspb::LPush::from(cmds),
            )),
        })
    }
}

impl CmdRequestBuild for dspb::LPop {
    fn build(cmds: Vec<&str>) -> Option<dspb::CommondRequest> {
        Some(dspb::CommondRequest {
            request_data: Some(dspb::commond_request::RequestData::Lpop(dspb::LPop::from(
                cmds,
            ))),
        })
    }
}

impl CmdRequestBuild for dspb::LRange {
    fn build(cmds: Vec<&str>) -> Option<dspb::CommondRequest> {
        Some(dspb::CommondRequest {
            request_data: Some(dspb::commond_request::RequestData::Lrange(
                dspb::LRange::from(cmds),
            )),
        })
    }
}
