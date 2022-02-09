use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrepError {
    #[error("Glob pattern error")]
    GlobPatternError(#[from] glob::PatternError),

    #[error("Regex pattern error")]
    RegexPatternError(#[from] regex::Error),

    #[error("I/O pattern error")]
    IoError(#[from] std::io::Error),

}