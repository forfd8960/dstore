use crate::errors::CmdError;
use crate::pb::pb;

pub trait CmdValidtor {
    fn validate(cmds: Vec<&str>) -> Option<CmdError>;
}

impl CmdValidtor for pb::Get {
    fn validate(cmds: Vec<&str>) -> Option<CmdError> {
        if cmds.len() < 2 {
            return Some(CmdError::InvalidGetCmd(
                "not enough parameter for Get".to_string(),
            ));
        }

        None
    }
}

impl CmdValidtor for pb::Set {
    fn validate(cmds: Vec<&str>) -> Option<CmdError> {
        if cmds.len() < 3 {
            return Some(CmdError::InvalidSetCmd(
                "not enough parameter for Set".to_string(),
            ));
        }

        None
    }
}

impl CmdValidtor for pb::HGet {
    fn validate(cmds: Vec<&str>) -> Option<CmdError> {
        if cmds.len() < 2 {
            return Some(CmdError::InvalidHGetCmd(
                "not enough parameter for HGet".to_string(),
            ));
        }

        None
    }
}

impl CmdValidtor for pb::HSet {
    fn validate(cmds: Vec<&str>) -> Option<CmdError> {
        if cmds.len() < 4 {
            return Some(CmdError::InvalidHSetCmd(
                "not enough parameter for HSet".to_string(),
            ));
        }

        if (cmds.len() - 4) % 2 != 0 {
            return Some(CmdError::InvalidHSetCmd(
                "invalid parameter for HSet".to_string(),
            ));
        }

        None
    }
}

impl CmdValidtor for pb::SAdd {
    fn validate(cmds: Vec<&str>) -> Option<CmdError> {
        if cmds.len() < 3 {
            return Some(CmdError::InvalidHSetCmd(
                "not enough parameter for SAdd".to_string(),
            ));
        }

        None
    }
}
