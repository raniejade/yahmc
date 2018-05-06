#[derive(Debug, Fail)]
pub enum ResourceError {
    #[fail(display = "Resource not found: {}", _0)]
    ResourceNotFound(String),
    #[fail(display = "Internal error: {}", _0)]
    Internal(String),
}

pub type Result<T> = ::std::result::Result<T, ResourceError>;
