use std::error::Error;
// A result with a boxed error type. We use a box here to allow us to represent
// multiple different possible error types.
pub type BoxedErrorResult<T> = std::result::Result<T, Box<dyn Error>>;
