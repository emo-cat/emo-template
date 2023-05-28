use thiserror::Error;

#[allow(dead_code)]
pub type Result<T> = ::std::result::Result<T, EmoError>;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum EmoError {
    #[error("some error: '{0}'")]
    SomeError(String),
}
