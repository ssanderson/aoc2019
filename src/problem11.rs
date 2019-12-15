/*!

--- Day 11: Space Police ---

On the way to Jupiter, you're pulled over by the Space Police.

"Attention, unmarked spacecraft! You are in violation of Space Law! All
spacecraft must have a clearly visible registration identifier! You have 24
hours to comply or be sent to Space Jail!"

Not wanting to be sent to Space Jail, you radio back to the Elves on Earth for
help. Although it takes almost three hours for their reply signal to reach you,
they send instructions for how to power up the emergency hull painting robot
and even provide a small Intcode program (your puzzle input) that will cause it
to paint your ship appropriately.

There's just one problem: you don't have an emergency hull painting robot.

You'll need to build a new emergency hull painting robot. The robot needs to be
able to move around on the grid of square panels on the side of your ship,
detect the color of its current panel, and paint its current panel black or
white. (All of the panels are currently black.)

The Intcode program will serve as the brain of the robot. The program uses
input instructions to access the robot's camera: provide 0 if the robot is over
a black panel or 1 if the robot is over a white panel. Then, the program will
output two values:

First, it will output a value indicating the color to paint the panel the robot
is over: 0 means to paint the panel black, and 1 means to paint the panel
white.

Second, it will output a value indicating the direction the robot should turn:
0 means it should turn left 90 degrees, and 1 means it should turn right 90
degrees.

After the robot turns, it should always move forward exactly one panel. The
robot starts facing up.

The robot will continue running for a while like this and halt when it is
finished drawing. Do not restart the Intcode computer inside the robot during
this process.

For example, suppose the robot is about to start running. Drawing black panels
as ., white panels as #, and the robot pointing the direction it is facing (< ^
> v), the initial state and region near the robot looks like this:

.....
.....
..^..
.....
.....

The panel under the robot (not visible here because a ^ is shown instead) is
also black, and so any input instructions at this point should be provided
0. Suppose the robot eventually outputs 1 (paint white) and then 0 (turn
left). After taking these actions and moving forward one panel, the region now
looks like this:

.....
.....
.<#..
.....
.....

Input instructions should still be provided 0. Next, the robot might output 0
(paint black) and then 0 (turn left):

.....
.....
..#..
.v...
.....

After more outputs (1,0, 1,0):

.....
.....
..^..
.##..
.....

The robot is now back where it started, but because it is now on a white panel,
input instructions should be provided 1. After several more outputs (0,1, 1,0,
1,0), the area looks like this:

.....
..<#.
...#.
.##..
.....

Before you deploy the robot, you should probably have an estimate of the area
it will cover: specifically, you need to know the number of panels it paints at
least once, regardless of color. In the example above, the robot painted 6
panels at least once. (It painted its starting panel twice, but that panel is
still only counted once; it also never painted the panel it ended on.)

Build a new emergency hull painting robot and run the Intcode program on
it. How many panels does it paint at least once?

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---

You're not sure what it's trying to paint, but it's definitely not a
registration identifier. The Space Police are getting impatient.

Checking your external ship cameras again, you notice a white panel marked
"emergency hull painting robot starting panel". The rest of the panels are
still black, but it looks like the robot was expecting to start on a white
panel, not a black one.

Based on the Space Law Space Brochure that the Space Police attached to one of
your windows, a valid registration identifier is always eight capital
letters. After starting the robot on a single white panel instead, what
registration identifier does it paint on your hull?

*/

use std::cmp::{max, min};
use std::collections::HashMap;
use std::convert::Into;

use crate::intcode::{Program, IO};
use crate::utils::{ProblemInput, ProblemResult};

type Coord = (i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Black,
    White,
}

impl Into<char> for Color {
    fn into(self) -> char {
        match self {
            Color::Black => '\u{2588}',
            Color::White => '\u{2591}',
        }
    }
}

impl Into<i64> for Color {
    fn into(self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl From<i64> for Color {
    fn from(i: i64) -> Color {
        match i {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Invalid color: {}", i),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    CW,
    CCW,
}

impl From<i64> for Turn {
    fn from(i: i64) -> Turn {
        match i {
            0 => Turn::CCW,
            1 => Turn::CW,
            _ => panic!("Invalid turn direction: {}", i),
        }
    }
}

#[derive(Debug)]
struct Robot {
    pub panels: HashMap<Coord, Color>,
    position: Coord,
    direction: Direction,
}

impl Robot {
    fn new(start_color: Color) -> Robot {
        let mut panels = HashMap::new();

        if start_color == Color::White {
            panels.insert((0, 0), Color::White);
        }

        Robot {
            panels,
            position: (0, 0),
            direction: Direction::Up,
        }
    }

    fn current_color(&self) -> Color {
        match self.panels.get(&self.position) {
            Some(&color) => color,
            None => Color::Black,
        }
    }

    fn paint(&mut self, color: Color) {
        self.panels.insert(self.position, color);
    }

    fn change_direction(&mut self, turn: Turn) {
        self.direction = match (self.direction, turn) {
            (Direction::Up, Turn::CW) => Direction::Right,
            (Direction::Right, Turn::CW) => Direction::Down,
            (Direction::Down, Turn::CW) => Direction::Left,
            (Direction::Left, Turn::CW) => Direction::Up,

            (Direction::Up, Turn::CCW) => Direction::Left,
            (Direction::Right, Turn::CCW) => Direction::Up,
            (Direction::Down, Turn::CCW) => Direction::Right,
            (Direction::Left, Turn::CCW) => Direction::Down,
        }
    }

    fn move_forward(&mut self) {
        let (x, y) = self.position;
        self.position = match self.direction {
            Direction::Up => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y - 1),
            Direction::Left => (x - 1, y),
        }
    }
}

#[derive(Debug)]
enum OutputState {
    ExpectingColor,
    ExpectingDirection,
}

#[derive(Debug)]
struct RobotIO<'a> {
    robot: &'a mut Robot,
    state: OutputState,
}

impl<'a> RobotIO<'a> {
    fn new(robot: &'a mut Robot) -> RobotIO {
        RobotIO {
            robot,
            state: OutputState::ExpectingColor,
        }
    }
}

impl<'a> IO for RobotIO<'a> {
    fn input(&mut self) -> Option<i64> {
        Some(self.robot.current_color().into())
    }

    fn output(&mut self, value: i64) -> Option<()> {
        self.state = match self.state {
            OutputState::ExpectingColor => {
                self.robot.paint(Color::from(value));
                OutputState::ExpectingDirection
            }
            OutputState::ExpectingDirection => {
                self.robot.change_direction(Turn::from(value));
                self.robot.move_forward();
                OutputState::ExpectingColor
            }
        };

        Some(())
    }
}

fn render(panels: &HashMap<Coord, Color>) -> String {
    let ((xmin, xmax), (ymin, ymax)) =
        panels
            .keys()
            .fold(((0, 0), (0, 0)), |((xmin, xmax), (ymin, ymax)), &(x, y)| {
                ((min(x, xmin), max(x, xmax)), (min(y, ymin), max(y, ymax)))
            });

    let mut out = String::new();
    for j in (ymin..=(ymax + 1)).rev() {
        for i in xmin..=(xmax + 1) {
            let color: Color = *panels.get(&(i, j)).unwrap_or(&Color::Black);
            out.push(color.into());
        }
        out.push('\n');
    }
    out
}

pub fn run() -> ProblemResult<()> {
    let program = Program::for_problem(11)?;

    // Part 1
    {
        let mut robot = Robot::new(Color::Black);
        let mut io = RobotIO::new(&mut robot);
        program.run(&mut io)?;
        println!("Number of painted locations: {}", robot.panels.len());
    }

    // Part 2
    {
        let mut robot = Robot::new(Color::White);
        let mut io = RobotIO::new(&mut robot);
        program.run(&mut io)?;
        println!("Number of painted locations: {}", robot.panels.len());
        println!("Label:\n{}", render(&robot.panels));
    }

    Ok(())
}
