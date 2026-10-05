use anyhow;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum SceneError {
    #[error("no such file or directory")]
    #[from(std::io::Error)]
    NotFound,
    #[error("failed to parse JSON scene file")]
    #[from(serde_json::Error)]
    ParseError,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
