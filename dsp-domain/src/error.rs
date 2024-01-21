use thiserror::Error;

/// Type alias for `Result` with default error `DspDomainError`.
///
/// Can be used like `std::result::Result` as well.
pub type Result<T, E = DspDomainError> = std::result::Result<T, E>;

/// This error is raised when a domain entity or value fails creation
/// at runtime.
#[derive(Debug, Error)]
pub enum DspDomainError {
    #[error("Error creating value object: `{0}`")]
    CreateValueObject(String),
    #[error("Error creating domain object.")]
    CreateDomainObject(String),
}
