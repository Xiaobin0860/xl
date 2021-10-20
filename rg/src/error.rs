use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrepError {
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
}
