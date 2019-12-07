use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode),
    Input,
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    EqualTo(ParameterMode, ParameterMode),
    Exit,
    Value(i64),
}

#[derive(Debug, Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn from_i64(i: i64) -> Option<ParameterMode> {
        match i {
            0 => Some(Self::Position),
            1 => Some(Self::Immediate),
            _ => None,
        }
    }
}

fn hundreds_digit(i: i64) -> i64 {
    (i % 1000) / 100
}

fn thousands_digit(i: i64) -> i64 {
    (i % 10000) / 1000
}

mod tests {
    #[test]
    fn test_hundreds_digit() {
        assert_eq!(super::hundreds_digit(5), 0);
        assert_eq!(super::hundreds_digit(100), 1);
        assert_eq!(super::hundreds_digit(200), 2);
        assert_eq!(super::hundreds_digit(302), 3);
        assert_eq!(super::hundreds_digit(403), 4);
        assert_eq!(super::hundreds_digit(1504), 5);
    }

    #[test]
    fn test_thousands_digit() {
        assert_eq!(super::thousands_digit(5), 0);
        assert_eq!(super::thousands_digit(100), 0);
        assert_eq!(super::thousands_digit(312), 0);
        assert_eq!(super::thousands_digit(1504), 1);
        assert_eq!(super::thousands_digit(2504), 2);
    }
}

fn first_parameter_mode(i: i64) -> Option<ParameterMode> {
    ParameterMode::from_i64(hundreds_digit(i))
}

fn second_parameter_mode(i: i64) -> Option<ParameterMode> {
    ParameterMode::from_i64(thousands_digit(i))
}

fn from_i64(i: i64) -> Option<Op> {
    if i > 3000 {
        return None;
    }

    let result = match i % 100 {
        1 => Op::Add(first_parameter_mode(i)?, second_parameter_mode(i)?),
        2 => Op::Mul(first_parameter_mode(i)?, second_parameter_mode(i)?),
        3 => Op::Input,
        4 => Op::Output(first_parameter_mode(i)?),
        5 => Op::JumpIfTrue(first_parameter_mode(i)?, second_parameter_mode(i)?),
        6 => Op::JumpIfFalse(first_parameter_mode(i)?, second_parameter_mode(i)?),
        7 => Op::LessThan(first_parameter_mode(i)?, second_parameter_mode(i)?),
        8 => Op::EqualTo(first_parameter_mode(i)?, second_parameter_mode(i)?),
        99 => Op::Exit,
        _ => Op::Value(i),
    };

    Some(result)
}

#[derive(Debug)]
pub enum ExecuteError {
    OutOfBounds(usize),
    BadOp { code: i64, pos: usize },
    InputError,
    OutputError,
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Bring enum variant constructors into scope.
use self::ExecuteError::*;

/// Result type for program executions.
pub type ExecuteResult<T> = Result<T, ExecuteError>;

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

pub trait IO {
    fn input(&mut self) -> Option<i64>;
    fn output(&mut self, value: i64) -> Option<()>;
}

struct NoIO;

impl IO for NoIO {
    fn input(&mut self) -> Option<i64> {
        None
    }

    fn output(&mut self, _value: i64) -> Option<()> {
        None
    }
}

pub struct StaticIO {
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl StaticIO {
    pub fn new(mut inputs: Vec<i64>) -> StaticIO {
        // Reverse so that inputs are popped in the order they were passed.
        inputs.reverse();
        StaticIO {
            inputs,
            outputs: vec![],
        }
    }

    pub fn outputs(self) -> Vec<i64> {
        self.outputs
    }
}

impl IO for StaticIO {
    fn input(&mut self) -> Option<i64> {
        self.inputs.pop()
    }

    fn output(&mut self, value: i64) -> Option<()> {
        self.outputs.push(value);
        Some(())
    }
}

/// An IntCode program.
pub struct Program {
    code: Vec<i64>,
}

impl Program {
    /// Construct a Program from a vector of i64s.
    pub fn new(code: Vec<i64>) -> Program {
        Program { code: code }
    }

    pub fn run<T: IO>(&self, io: &mut T) -> ExecuteResult<Vec<i64>> {
        Execution::new(self.code.clone()).simulate(io)
    }

    /// Run the program on set of inputs.
    pub fn run_problem2(&self, noun: i64, verb: i64, output_index: usize) -> ExecuteResult<i64> {
        if self.code.len() <= output_index {
            return ExecuteResult::Err(OutOfBounds(output_index));
        }

        let mut code_copy = self.code.clone();
        code_copy[1] = noun;
        code_copy[2] = verb;

        let final_state = Execution::new(code_copy).simulate(&mut NoIO)?;

        Ok(final_state[output_index])
    }
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Result<Vec<i64>, ParseError> = s
            .trim_end()
            .split(",")
            .map(|s| s.parse::<i64>().map_err(|_| ParseError(s.into())))
            .collect();

        Ok(Program::new(parsed?))
    }
}

/// A single program execution.
struct Execution {
    state: Vec<i64>,
}

impl Execution {
    pub fn new(state: Vec<i64>) -> Execution {
        Execution { state: state }
    }

    pub fn simulate<T: IO>(mut self, io: &mut T) -> ExecuteResult<Vec<i64>> {
        let mut pos: usize = 0;
        loop {
            let code = self.state[pos];
            let op = from_i64(code);
            match op {
                Some(Op::Add(lhs_mode, rhs_mode)) => {
                    self.do_binop(pos, lhs_mode, rhs_mode, |x, y| x + y);
                    pos += 4;
                }
                Some(Op::Mul(lhs_mode, rhs_mode)) => {
                    self.do_binop(pos, lhs_mode, rhs_mode, |x, y| x * y);
                    pos += 4;
                }
                Some(Op::LessThan(lhs_mode, rhs_mode)) => {
                    self.do_binop(pos, lhs_mode, rhs_mode, |x, y| (x < y) as i64);
                    pos += 4;
                }
                Some(Op::EqualTo(lhs_mode, rhs_mode)) => {
                    self.do_binop(pos, lhs_mode, rhs_mode, |x, y| (x == y) as i64);
                    pos += 4;
                }
                Some(Op::JumpIfTrue(test_mode, target_mode)) => {
                    let test = self.do_read(self.state[pos + 1], test_mode);
                    if test != 0 {
                        pos = self.do_read(self.state[pos + 2], target_mode) as usize;
                    } else {
                        pos += 3;
                    }
                }
                Some(Op::JumpIfFalse(test_mode, target_mode)) => {
                    let test = self.do_read(self.state[pos + 1], test_mode);
                    if test == 0 {
                        pos = self.do_read(self.state[pos + 2], target_mode) as usize;
                    } else {
                        pos += 3;
                    }
                }
                Some(Op::Input) => {
                    match io.input() {
                        Some(value) => {
                            let dest = self.state[pos + 1] as usize;
                            self.state[dest] = value
                        }
                        None => return ExecuteResult::Err(InputError),
                    }
                    pos += 2;
                }
                Some(Op::Output(mode)) => {
                    let value = self.do_read(self.state[pos + 1], mode);
                    match io.output(value) {
                        Some(()) => {}
                        None => return ExecuteResult::Err(OutputError),
                    }
                    pos += 2;
                }
                Some(Op::Exit) => {
                    break;
                }
                Some(Op::Value(_)) | None => {
                    return ExecuteResult::Err(BadOp { code, pos });
                }
            }
        }
        Ok(self.state)
    }

    fn do_binop<F>(&mut self, pos: usize, lhs_mode: ParameterMode, rhs_mode: ParameterMode, f: F)
    where
        F: FnOnce(i64, i64) -> i64,
    {
        let lhs = self.do_read(self.state[pos + 1], lhs_mode);
        let rhs = self.do_read(self.state[pos + 2], rhs_mode);
        let dest = self.state[pos + 3] as usize;

        self.state[dest] = f(lhs, rhs);
    }

    fn do_read(&self, param: i64, mode: ParameterMode) -> i64 {
        match mode {
            ParameterMode::Position => self.state[param as usize],
            ParameterMode::Immediate => param,
        }
    }
}
