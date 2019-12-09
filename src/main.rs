use std::env;

mod intcode;
mod utils;

mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem7;
mod problem8;

fn main() -> utils::ProblemResult<()> {
    let args: Vec<String> = env::args().collect();
    let num_args = args.len() - 1;
    match num_args {
        1 => match args[1].as_ref() {
            "1" => problem1::run(),
            "2" => problem2::run(),
            "3" => problem3::run(),
            "4" => problem4::run(),
            "5" => problem5::run(),
            "6" => problem6::run(),
            "7" => problem7::run(),
            "8" => problem8::run(),
            _ => utils::bail(&format!("Unknown problem: {}", args[1])),
        },
        _ => utils::bail(&format!("Expected 1 arg, got {}", num_args)),
    }
}
