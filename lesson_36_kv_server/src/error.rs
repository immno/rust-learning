use thiserror::Error;

use crate::Value;

#[derive(Error, Debug)]
pub enum KvError {
    #[error("Not found for table: {0}, key: {1}")]
    NotFound(String, String),

    #[error("Cannot parse command: `{0}`")]
    InvalidCommand(String),

    #[error("Cannot convert value: `{:0}` to {1}")]
    ConvertError(Value, &'static str),

    #[error("Cannot process command: {0} with table: {1}, key: {2}. Error: {}")]
    StorageError(&'static str, String, String, String),

    #[error("Failed to encode protobuf message")]
    EncodeError(#[from] prost::EncodeError),

    #[error("Failed to decode protobuf message")]
    DecodeError(#[from] prost::DecodeError),

    #[error("Failed to access sled db")]
    SledError(#[from] sled::Error),

    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("Frame is larger than max size")]
    FrameError,

    #[error("Internal error: {0}")]
    Internal(String),
}