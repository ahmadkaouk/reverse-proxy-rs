use thiserror::Error;
use tonic::{Code, Status};

/// A `Result` alias where the `Err` case is `reqwest::Error`.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown error")]
    Unknown,
    #[error("http client error")]
    ReqwestError(#[from] reqwest::Error),
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        let code = match value {
            Error::Unknown => Code::Unknown,
            Error::ReqwestError(_) => Code::Aborted,
        };
        Status::new(code, String::new())
    }
}
