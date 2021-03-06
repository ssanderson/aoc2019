use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    EqualTo(ParameterMode, ParameterMode, ParameterMode),
    AdjustRelativeBase(ParameterMode),
    Exit,
}

#[derive(Debug, Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn from_i64(i: i64) -> Option<ParameterMode> {
        match i {
            0 => Some(Self::Position),
            1 => Some(Self::Immediate),
            2 => Some(Self::Relative),
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

fn ten_thousands_digit(i: i64) -> i64 {
    (i % 100000) / 10000
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

fn third_parameter_mode(i: i64) -> Option<ParameterMode> {
    ParameterMode::from_i64(ten_thousands_digit(i))
}

fn from_i64(i: i64) -> Option<Op> {
    let result = match i % 100 {
        1 => Op::Add(
            first_parameter_mode(i)?,
            second_parameter_mode(i)?,
            third_parameter_mode(i)?,
        ),
        2 => Op::Mul(
            first_parameter_mode(i)?,
            second_parameter_mode(i)?,
            third_parameter_mode(i)?,
        ),
        3 => Op::Input(first_parameter_mode(i)?),
        4 => Op::Output(first_parameter_mode(i)?),
        5 => Op::JumpIfTrue(first_parameter_mode(i)?, second_parameter_mode(i)?),
        6 => Op::JumpIfFalse(first_parameter_mode(i)?, second_parameter_mode(i)?),
        7 => Op::LessThan(
            first_parameter_mode(i)?,
            second_parameter_mode(i)?,
            third_parameter_mode(i)?,
        ),
        8 => Op::EqualTo(
            first_parameter_mode(i)?,
            second_parameter_mode(i)?,
            third_parameter_mode(i)?,
        ),
        9 => Op::AdjustRelativeBase(first_parameter_mode(i)?),
        99 => Op::Exit,
        _ => return None,
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

impl Error for ExecuteError {}

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

pub trait IO: fmt::Debug {
    fn input(&mut self) -> Option<i64>;
    fn output(&mut self, value: i64) -> Option<()>;
}

#[derive(Debug)]
struct NoIO;

impl IO for NoIO {
    fn input(&mut self) -> Option<i64> {
        None
    }

    fn output(&mut self, _value: i64) -> Option<()> {
        None
    }
}

#[derive(Debug)]
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

    /// Run a single instance of the program to completion.
    pub fn run<T: IO>(&self, io: &mut T) -> ExecuteResult<()> {
        Execution::new(self.code.clone(), io).run_to_completion()?;
        Ok(())
    }

    /// Run multiple instances of the program until all programs have halted.
    /// Whenever a program performs output, switch between programs.
    pub fn run_concurrently<T: IO>(&self, ios: &mut Vec<T>) -> ExecuteResult<()> {
        use std::collections::VecDeque;

        let mut run_queue: VecDeque<Execution<T>> = ios
            .iter_mut()
            .map(|io| Execution::new(self.code.clone(), io))
            .collect();

        while let Some(mut execution) = run_queue.pop_front() {
            loop {
                match execution.step()? {
                    ExecState::Running => {}
                    ExecState::DidOutput => {
                        run_queue.push_back(execution);
                        break;
                    }
                    ExecState::Halted => {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn set_address(&mut self, addr: usize, value: i64) {
        self.code[addr] = value;
    }

    /// Run the program on set of inputs.
    pub fn run_problem2(&self, noun: i64, verb: i64, output_index: usize) -> ExecuteResult<i64> {
        if self.code.len() <= output_index {
            return ExecuteResult::Err(OutOfBounds(output_index));
        }

        let mut code_copy = self.code.clone();
        code_copy[1] = noun;
        code_copy[2] = verb;

        let final_state = Execution::new(code_copy, &mut NoIO).run_to_completion()?;

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
struct Execution<'a, T: IO> {
    io: &'a mut T,
    state: Vec<i64>,
    pos: usize,
    relative_base: i64,
}

enum ExecState {
    Running,
    DidOutput,
    Halted,
}

impl<'a, T: IO> Execution<'a, T> {
    pub fn new(state: Vec<i64>, io: &'a mut T) -> Execution<'a, T> {
        Execution {
            state,
            io,
            pos: 0,
            relative_base: 0,
        }
    }

    pub fn run_to_completion(mut self) -> ExecuteResult<Vec<i64>> {
        loop {
            match self.step()? {
                ExecState::Halted => {
                    break;
                }
                _ => {}
            }
        }
        Ok(self.state)
    }

    pub fn step(&mut self) -> ExecuteResult<ExecState> {
        let code = self.state[self.pos];
        let op = from_i64(code);
        match op {
            Some(Op::Add(lhs_mode, rhs_mode, dest_mode)) => {
                self.do_binop(lhs_mode, rhs_mode, dest_mode, |x, y| x + y)?;
                self.pos += 4;
            }
            Some(Op::Mul(lhs_mode, rhs_mode, dest_mode)) => {
                self.do_binop(lhs_mode, rhs_mode, dest_mode, |x, y| x * y)?;
                self.pos += 4;
            }
            Some(Op::LessThan(lhs_mode, rhs_mode, dest_mode)) => {
                self.do_binop(lhs_mode, rhs_mode, dest_mode, |x, y| (x < y) as i64)?;
                self.pos += 4;
            }
            Some(Op::EqualTo(lhs_mode, rhs_mode, dest_mode)) => {
                self.do_binop(lhs_mode, rhs_mode, dest_mode, |x, y| (x == y) as i64)?;
                self.pos += 4;
            }
            Some(Op::JumpIfTrue(test_mode, target_mode)) => {
                let test = self.do_read(self.pos + 1, test_mode)?;
                if test != 0 {
                    self.pos = self.do_read(self.pos + 2, target_mode)? as usize;
                } else {
                    self.pos += 3;
                }
            }
            Some(Op::JumpIfFalse(test_mode, target_mode)) => {
                let test = self.do_read(self.pos + 1, test_mode)?;
                if test == 0 {
                    self.pos = self.do_read(self.pos + 2, target_mode)? as usize;
                } else {
                    self.pos += 3;
                }
            }
            Some(Op::Input(mode)) => {
                match self.io.input() {
                    Some(value) => {
                        self.do_write(self.pos + 1, mode, value)?;
                    }
                    None => return Err(InputError),
                }
                self.pos += 2;
            }
            Some(Op::Output(mode)) => {
                let value = self.do_read(self.pos + 1, mode)?;
                match self.io.output(value) {
                    Some(()) => {}
                    None => return Err(OutputError),
                }
                self.pos += 2;
                return Ok(ExecState::DidOutput);
            }
            Some(Op::Exit) => {
                return Ok(ExecState::Halted);
            }
            Some(Op::AdjustRelativeBase(mode)) => {
                let value = self.do_read(self.pos + 1, mode)?;
                self.relative_base += value;
                self.pos += 2;
            }
            None => {
                return ExecuteResult::Err(BadOp {
                    code,
                    pos: self.pos,
                });
            }
        }
        Ok(ExecState::Running)
    }

    fn do_binop<F>(
        &mut self,
        lhs_mode: ParameterMode,
        rhs_mode: ParameterMode,
        dest_mode: ParameterMode,
        f: F,
    ) -> ExecuteResult<()>
    where
        F: FnOnce(i64, i64) -> i64,
    {
        let lhs = self.do_read(self.pos + 1, lhs_mode)?;
        let rhs = self.do_read(self.pos + 2, rhs_mode)?;
        self.do_write(self.pos + 3, dest_mode, f(lhs, rhs))?;

        Ok(())
    }

    fn do_read(&mut self, pos: usize, mode: ParameterMode) -> ExecuteResult<i64> {
        let param = self.state[pos];
        match mode {
            ParameterMode::Position | ParameterMode::Relative => {
                Ok(self.read_raw(self.as_address(param, mode)?))
            }
            ParameterMode::Immediate => Ok(param),
        }
    }

    fn do_write(&mut self, pos: usize, mode: ParameterMode, value: i64) -> ExecuteResult<()> {
        let param = self.state[pos];
        let dest = self.as_address(param, mode)?;

        self.write_raw(dest, value);

        Ok(())
    }

    fn read_raw(&mut self, ix: usize) -> i64 {
        self.ensure_capacity(ix);
        self.state[ix]
    }

    fn write_raw(&mut self, ix: usize, value: i64) {
        self.ensure_capacity(ix);
        self.state[ix] = value;
    }

    fn as_address(&self, param: i64, mode: ParameterMode) -> ExecuteResult<usize> {
        match mode {
            ParameterMode::Position => Ok(param as usize),
            ParameterMode::Relative => Ok((param + self.relative_base) as usize),
            ParameterMode::Immediate => panic!("Can't take address of immediate"),
        }
    }

    fn ensure_capacity(&mut self, ix: usize) {
        if ix >= self.state.len() {
            self.state.resize(ix + 1, 0);
        }
    }
}
