/// --- Day 3: Crossed Wires ---

/// The gravity assist was successful, and you're well on your way to the Venus
/// refuelling station. During the rush back on Earth, the fuel management
/// system wasn't completely installed, so that's next on the priority list.

/// Opening the front panel reveals a jumble of wires. Specifically, two wires
/// are connected to a central port and extend outward on a grid. You trace the
/// path each wire takes as it leaves the central port, one wire per line of
/// text (your puzzle input).

/// The wires twist and turn, but the two wires occasionally cross paths. To
/// fix the circuit, you need to find the intersection point closest to the
/// central port. Because the wires are on a grid, use the Manhattan distance
/// for this measurement. While the wires do technically cross right at the
/// central port where they both start, this point does not count, nor does a
/// wire count as crossing with itself.

/// For example, if the first wire's path is R8,U5,L5,D3, then starting from
/// the central port (o), it goes right 8, up 5, left 5, and finally down 3:

/// ...........
/// ...........
/// ...........
/// ....+----+.
/// ....|....|.
/// ....|....|.
/// ....|....|.
/// .........|.
/// .o-------+.
/// ...........

/// Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down
/// 4, and left 4:

/// ...........
/// .+-----+...
/// .|.....|...
/// .|..+--X-+.
/// .|..|..|.|.
/// .|.-X--+.|.
/// .|..|....|.
/// .|.......|.
/// .o-------+.
/// ...........

/// These wires cross at two locations (marked X), but the lower-left one is
/// closer to the central port: its distance is 3 + 3 = 6.

/// Here are a few more examples:

/// R75,D30,R83,U83,L12,D49,R71,U7,L72
/// U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
/// R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
/// U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135

/// What is the Manhattan distance from the central port to the closest
/// intersection?

/// --- Part Two ---

/// It turns out that this circuit is very timing-sensitive; you actually need
/// to minimize the signal delay.

/// To do this, calculate the number of steps each wire takes to reach each
/// intersection; choose the intersection where the sum of both wires' steps is
/// lowest. If a wire visits a position on the grid multiple times, use the
/// steps value from the first time it visits that position when calculating
/// the total value of a specific intersection.

/// The number of steps a wire takes is the total number of grid squares the
/// wire has entered to get to that location, including the intersection being
/// considered. Again consider the example from above:

/// ...........
/// .+-----+...
/// .|.....|...
/// .|..+--X-+.
/// .|..|..|.|.
/// .|.-X--+.|.
/// .|..|....|.
/// .|.......|.
/// .o-------+.
/// ...........

/// In the above example, the intersection closest to the central port is
/// reached after 8+5+5+2 = 20 steps by the first wire and 7+6+4+3 = 20 steps
/// by the second wire for a total of 20+20 = 40 steps.

/// However, the top-right intersection is better: the first wire takes only
/// 8+5+2 = 15 and the second wire takes only 7+6+2 = 15, a total of 15+15 = 30
/// steps.

/// Here are the best steps for the extra examples from above:

/// R75,D30,R83,U83,L12,D49,R71,U7,L72
/// U62,R66,U55,R34,D71,R55,D58,R83 = 610 steps
/// R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
/// U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = 410 steps
/// What is the fewest combined steps the wires must take to reach an intersection?
use std::fs;
use std::path::Path;

use crate::utils::BoxedErrorResult;

mod wire {
    use std::collections::HashSet;
    use std::error::Error;
    use std::fmt;
    use std::str::FromStr;

    #[derive(Debug, Clone, Copy)]
    enum Segment {
        Up(u64),
        Down(u64),
        Left(u64),
        Right(u64),
    }

    #[derive(Debug)]
    pub enum ParseError {
        EmptySegment,
        BadPrefix(char),
        BadInt(String),
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Error for ParseError {}

    impl FromStr for Segment {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parse_length = || -> Result<u64, ParseError> {
                let suffix = &s[1..];
                let result = suffix.parse::<u64>();
                result.map_err(|_| ParseError::BadInt(suffix.into()))
            };

            let parsed = match s.chars().nth(0) {
                Some('U') => Segment::Up(parse_length()?),
                Some('D') => Segment::Down(parse_length()?),
                Some('L') => Segment::Left(parse_length()?),
                Some('R') => Segment::Right(parse_length()?),
                Some(c) => return Err(ParseError::BadPrefix(c)),
                None => {
                    return Err(ParseError::EmptySegment);
                }
            };

            Ok(parsed)
        }
    }

    #[derive(Debug)]
    pub struct Wire {
        segments: Vec<Segment>,
    }

    impl Wire {
        pub fn intersect(&self, other: &Wire) -> HashSet<Point> {
            self.trace().intersection(&other.trace()).cloned().collect()
        }

        fn trace(&self) -> HashSet<Point> {
            let mut points: HashSet<Point> = HashSet::new();
            let mut pos = Point::new(0, 0);

            for segment in &self.segments {
                let new_points = pos.points_along_segment(*segment);
                for point in new_points.iter() {
                    points.insert(*point);
                }
                pos = *new_points.last().unwrap();
            }

            points
        }
    }

    impl FromStr for Wire {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let segments: Result<Vec<Segment>, ParseError> =
                s.split(",").map(|part| part.parse::<Segment>()).collect();

            Ok(Wire {
                segments: segments?,
            })
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Point {
        x: i64,
        y: i64,
    }

    impl Point {
        pub fn new(x: i64, y: i64) -> Point {
            Self { x, y }
        }

        pub fn manhattan_distance_from_origin(&self) -> u64 {
            (self.x.abs() + self.y.abs()) as u64
        }

        fn points_along_segment(&self, segment: Segment) -> Vec<Point> {
            let mut out = vec![];

            match segment {
                Segment::Up(n) => {
                    for i in 1..n + 1 {
                        out.push(Point {
                            x: self.x,
                            y: self.y + i as i64,
                        });
                    }
                }
                Segment::Down(n) => {
                    for i in 1..n + 1 {
                        out.push(Point {
                            x: self.x,
                            y: self.y - i as i64,
                        });
                    }
                }
                Segment::Left(n) => {
                    for i in 1..n + 1 {
                        out.push(Point {
                            x: self.x - i as i64,
                            y: self.y,
                        });
                    }
                }
                Segment::Right(n) => {
                    for i in 1..n + 1 {
                        out.push(Point {
                            x: self.x + i as i64,
                            y: self.y,
                        });
                    }
                }
            }

            out
        }
    }
}

use wire::{ParseError, Point, Wire};

pub fn run() {
    let here = Path::new(file!()).parent().unwrap();
    let input_path = here.join("problem3_input.txt");

    match read_wires(&input_path) {
        Ok((first, second)) => {
            let intersection = first.intersect(&second);
            let closest_to_origin: &Point = intersection
                .iter()
                .min_by_key(|p| p.manhattan_distance_from_origin())
                .expect("Lines do not intersect!");

            println!("Part 1");
            println!("------");
            println!("Closest point to origin is {:?}.", closest_to_origin);
            println!(
                "Distance is {:?}.",
                closest_to_origin.manhattan_distance_from_origin()
            );
        }
        Err(e) => {
            println!("Failed to read input.\nError was: {}", e);
            return;
        }
    }
}

fn read_wires(path: &Path) -> BoxedErrorResult<(Wire, Wire)> {
    let file_content = fs::read_to_string(path)?;
    let lines: Result<Vec<Wire>, ParseError> = file_content
        .lines()
        .map(|line| line.parse::<Wire>())
        .collect();

    let mut lines = lines?;

    // TODO: Clean up these.
    if lines.len() != 2 {
        panic!("Expected 2 lines.");
    } else {
        Ok((lines.pop().unwrap(), lines.pop().unwrap()))
    }
}
