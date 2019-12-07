use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

// A result with a boxed error type. We use a box here to allow us to represent
// multiple different possible error types.
pub type BoxedErrorResult<T> = std::result::Result<T, Box<dyn Error>>;

pub type ProblemResult<T> = BoxedErrorResult<T>;

#[derive(Debug)]
pub struct SimpleError {
    msg: String,
}

impl SimpleError {
    pub fn new(msg: &str) -> SimpleError {
        SimpleError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for SimpleError {}

pub fn bail(msg: &str) -> ProblemResult<()> {
    Err(SimpleError::new(msg).into())
}

pub trait ProblemInput
where
    Self: Sized,
{
    fn for_problem(n: u64) -> ProblemResult<Self>;
}

impl<T: FromStr> ProblemInput for T
where
    <T as std::str::FromStr>::Err: 'static + std::error::Error,
{
    fn for_problem(n: u64) -> ProblemResult<T> {
        let here = Path::new(file!()).parent().unwrap();
        let input_path = here.join(format!("inputs/problem{}_input.txt", n));
        let file_content = fs::read_to_string(input_path)?;

        file_content.parse::<T>().map_err(|e| e.into())
    }
}
