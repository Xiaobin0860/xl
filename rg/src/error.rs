use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrepError {
    #[error("Glob error")]
    GlobError(#[from] glob::PatternError),
    #[error("Regex error")]
    RegexError(#[from] regex::Error),
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
}
