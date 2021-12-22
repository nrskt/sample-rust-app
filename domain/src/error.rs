use thiserror::Error;

pub type DomainResult<T> = Result<T, DomainError>;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("story point takes 1,2,3,5,7")]
    InvalidStoryPoint,
}
