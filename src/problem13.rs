/*!1

--- Day 13: Care Package ---

As you ponder the solitude of space and the ever-increasing three-hour
roundtrip for messages between you and Earth, you notice that the Space Mail
Indicator Light is blinking. To help keep you sane, the Elves have sent you a
care package.

It's a new game for the ship's arcade cabinet! Unfortunately, the arcade is all
the way on the other end of the ship. Surely, it won't be hard to build your
own - the care package even comes with schematics.

The arcade cabinet runs Intcode software like the game the Elves sent (your
puzzle input). It has a primitive screen capable of drawing square tiles on a
grid. The software draws tiles to the screen with output instructions: every
three output instructions specify the x position (distance from the left), y
position (distance from the top), and tile id. The tile id is interpreted as
follows:

0 is an empty tile. No game object appears in this tile.
1 is a wall tile. Walls are indestructible barriers.
2 is a block tile. Blocks can be broken by the ball.
3 is a horizontal paddle tile. The paddle is indestructible.
4 is a ball tile. The ball moves diagonally and bounces off objects.

For example, a sequence of output values like 1,2,3,6,5,4 would draw a
horizontal paddle tile (1 tile from the left and 2 tiles from the top) and a
ball tile (6 tiles from the left and 5 tiles from the top).

Start the game. How many block tiles are on the screen when the game exits?

--- Part Two ---

The game didn't run because you didn't put in any quarters. Unfortunately, you
did not bring any quarters. Memory address 0 represents the number of quarters
that have been inserted; set it to 2 to play for free.

The arcade cabinet has a joystick that can move left and right. The software
reads the position of the joystick with input instructions:

If the joystick is in the neutral position, provide 0.
If the joystick is tilted to the left, provide -1.
If the joystick is tilted to the right, provide 1.

The arcade cabinet also has a segment display capable of showing a single
number that represents the player's current score. When three output
instructions specify X=-1, Y=0, the third output instruction is not a tile; the
value instead specifies the new score to show in the segment display. For
example, a sequence of output values like -1,0,12345 would show 12345 as the
player's current score.

Beat the game by breaking all the blocks. What is your score after the last
block is broken?

 */

use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::intcode::{Program, IO};
use crate::utils::{ProblemInput, ProblemResult};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

struct BadTile(i64);

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '\u{2591}',  // White
            Tile::Block => '\u{2588}', // Black
            Tile::Paddle => '-',
            Tile::Ball => 'O',
        }
    }
}

impl TryFrom<i64> for Tile {
    type Error = BadTile;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            i => Err(BadTile(i)),
        }
    }
}

type Coord = (usize, usize);

#[derive(Debug)]
enum IOState {
    Empty,
    One(i64),
    Two(i64, i64),
}

#[derive(Debug)]
enum Joystick {
    Left = -1,
    Neutral = 0,
    Right = 1,
}

#[derive(Debug)]
struct Game {
    screen: HashMap<Coord, Tile>,
    state: IOState,
    pub score: Option<i64>,
    ball: Option<Coord>,
    paddle: Option<Coord>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            screen: HashMap::new(),
            state: IOState::Empty,
            score: None,
            ball: None,
            paddle: None,
        }
    }

    #[allow(dead_code)]
    pub fn paint_screen(&self) {
        use std::cmp::{max, min};

        let ((xmin, xmax), (ymin, ymax)) =
            self.screen
                .keys()
                .fold(((0, 0), (0, 0)), |((xmin, xmax), (ymin, ymax)), &(x, y)| {
                    ((min(x, xmin), max(x, xmax)), (min(y, ymin), max(y, ymax)))
                });

        let mut out = String::new();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                let tile = *self.screen.get(&(x, y)).unwrap_or(&Tile::Empty);
                out.push(tile.into())
            }
            out.push('\n');
        }
        print!("{}[2J", 27 as char); // Clear screen.
        println!("{}", out);
        println!("Score: {}", self.score.unwrap_or(0));
    }

    pub fn count(&self, tile: Tile) -> usize {
        self.screen.values().filter(|&&x| x == tile).count()
    }
}

impl IO for Game {
    fn input(&mut self) -> Option<i64> {
        self.paint_screen();
        match (self.ball, self.paddle) {
            (Some(ball), Some(paddle)) => {
                match ball.0.cmp(&paddle.0) {
                    Ordering::Less => Some(Joystick::Left as i64),
                    Ordering::Equal => Some(Joystick::Neutral as i64),
                    Ordering::Greater => Some(Joystick::Right as i64),
                }
            }
            _ => None,
        }
    }

    fn output(&mut self, value: i64) -> Option<()> {
        match self.state {
            IOState::Empty => {
                self.state = IOState::One(value);
            }
            IOState::One(prev) => {
                self.state = IOState::Two(prev, value);
            }
            IOState::Two(-1, 0) => {
                self.score = Some(value);
                self.state = IOState::Empty;
            }
            IOState::Two(first, second) => {
                let tile = Tile::try_from(value).ok()?;
                let coord = (usize::try_from(first).ok()?, usize::try_from(second).ok()?);

                self.screen.insert(coord, tile);

                // Remember locations of ball and paddle for AI.
                if tile == Tile::Ball {
                    self.ball = Some(coord);
                }
                if tile == Tile::Paddle {
                    self.paddle = Some(coord);
                }

                self.state = IOState::Empty;
            }
        }
        Some(())
    }
}

pub fn run() -> ProblemResult<()> {
    let program = Program::for_problem(13)?;

    // Part 1
    {
        let mut game = Game::new();
        program.run(&mut game)?;
        println!("Number of block tiles: {}", game.count(Tile::Block));
    }

    // Part 2
    {
        let mut program = program; // Don't need to be mutable until here.
        program.set_address(0, 2);

        let mut game = Game::new();
        program.run(&mut game)?;

        println!("Final Score: {}", game.score.ok_or("Error running game.")?);
    }

    Ok(())
}
