use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid connection string passed.")]
    InvalidConnectionString(),
    #[error("Connection Failed: {0}")]
    ConnectionFailed(String),
}
