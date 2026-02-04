use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum SubsnifferError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("DNS resolution failed for {0}")]
    ResolutionFailed(String),

    #[error("Wordlist not found: {0}")]
    WordlistNotFound(String),
}
