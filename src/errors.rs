use thiserror::Error;

#[derive(Clone, Copy, Debug, Error)]
pub enum SceneError {
    #[error("Scene file not found")]
    NotFound,
    #[error("Failed to parse scene JSON file")]
    ParseError,
}
