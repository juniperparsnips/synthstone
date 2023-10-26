use thiserror::Error;

pub type CookBookResult<T> = Result<T, CookBookError>;

#[derive(Debug, Error)]
pub enum CookBookError {
    #[error("Unknown error: {0}")]
    Unknown(String),
}
