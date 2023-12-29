use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum KvError {
    #[error("Not found for key: {0}")]
    NotFound(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum ParseCmdError {
    #[error("Invalid get cmd: {0}")]
    InvalidGetCmd(String),

    #[error("Invalid set cmd: {0}")]
    InvalidSetCmd(String),

    #[error("Invalid hset cmd: {0}")]
    InvalidHSetCmd(String),

    #[error("Invalid hget cmd: {0}")]
    InvalidHGetCmd(String),
}
