use anyhow;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum SceneError {
    #[error("no such file or directory")]
    #[from(std::io::Error)]
    NotFound {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse JSON scene file")]
    #[from(serde_json::Error)]
    ParseError {
        file_path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
