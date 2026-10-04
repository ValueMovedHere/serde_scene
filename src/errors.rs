use anyhow;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum SceneError {
    #[error("no such file or directory: path {path} does not exist")]
    #[from(std::io::Error)]
    NotFound { path: String },
    #[error("failed to parse JSON scene file")]
    ParseError,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
