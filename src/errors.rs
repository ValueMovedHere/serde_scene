use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum SceneError {
    #[error("No such file or directory: path {path} does not exist")]
    NotFound { path: String },
    #[error("Failed to parse JSON scene file")]
    ParseError,
}
