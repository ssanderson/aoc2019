/// --- Day 1: The Tyranny of the Rocket Equation ---

/// Santa has become stranded at the edge of the Solar System while delivering
/// presents to other planets! To accurately calculate his position in space,
/// safely align his warp drive, and return to Earth in time to save Christmas,
/// he needs you to bring him measurements from fifty stars.

/// Collect stars by solving puzzles. Two puzzles will be made available on each
/// day in the Advent calendar; the second puzzle is unlocked when you complete
/// the first. Each puzzle grants one star. Good luck!

/// The Elves quickly load you into a spacecraft and prepare to launch.

/// At the first Go / No Go poll, every Elf is Go until the Fuel
/// Counter-Upper. They haven't determined the amount of fuel required yet.

/// Fuel required to launch a given module is based on its mass. Specifically,
/// to find the fuel required for a module, take its mass, divide by three,
/// round down, and subtract 2.

/// For example:

/// For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
/// For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
/// For a mass of 1969, the fuel required is 654.
/// For a mass of 100756, the fuel required is 33583.

/// The Fuel Counter-Upper needs to know the total fuel requirement. To find it,
/// individually calculate the fuel needed for the mass of each module (your
/// puzzle input), then add together all the fuel values.

/// What is the sum of the fuel requirements for all of the modules on your spacecraft?
use std::error;
use std::fs;
use std::path::Path;

// A result with a boxed error type. We use a box here to allow us to represent
// multiple different possible error types.
type BoxedErrorResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn run() {
    let here = Path::new(file!()).parent().unwrap();
    let input_path = here.join("problem1_input.txt");

    let masses = match read_masses(&input_path) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to read masses. \nError was: {}", e);
            return;
        }
    };

    let fuels = masses.into_iter().map(fuel_for_mass);
    let total_fuel: u64 = fuels.sum();

    println!("Problem 1:");
    println!("==========");
    println!("Total Fuel Required: {}", total_fuel);
}

fn fuel_for_mass(mass: u64) -> u64 {
    (mass / 3) - 2
}

fn read_masses(path: &Path) -> BoxedErrorResult<Vec<u64>> {
    let file_content = fs::read_to_string(path)?;

    let masses: Result<Vec<u64>, _> = file_content
        .lines()
        .map(|line| line.parse::<u64>())
        .collect();

    // Convert parse error into dynamic error.
    masses.map_err(|e| e.into())
}
