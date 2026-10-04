use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum SceneError {
    #[error("no such file or directory: path {path} does not exist")]
    NotFound { path: String },
    #[error("failed to parse JSON scene file")]
    ParseError,
}
