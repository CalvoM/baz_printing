use thiserror::Error;

#[derive(Error, Debug)]
pub enum LPDPClientError {
    #[error("{0}")]
    UnreachableServer(String),
    #[error("{0}")]
    NotAcknowledged(String),
    #[error("{0}")]
    FailedWrite(String),
    #[error("{0}")]
    FailedRead(String),
    #[error("{0}")]
    SystemDetailsError(String),
    #[error("{0}")]
    FileReadError(String),
    #[error("{0}")]
    WriteTimeoutError(String),
    #[error("{0}")]
    ReadTimeoutError(String),
}
