use std::cmp::{max, min};
use std::collections::HashMap;

pub type Coord = (i64, i64);

pub trait GridElem: Default + Copy + Into<char> {}

impl<T: Default + Copy + Into<char>> GridElem for T {}

#[derive(Debug)]
pub struct Grid<T: GridElem> {
    cells: HashMap<Coord, T>,
}

impl<T: GridElem> Grid<T> {
    pub fn new(cells: HashMap<Coord, T>) -> Grid<T> {
        Grid { cells }
    }

    pub fn empty() -> Grid<T> {
        Self::new(HashMap::new())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Coord, &T)> {
        self.cells.iter()
    }

    pub fn neighbors(&self, c: &Coord) -> impl Iterator<Item = (Coord, T)> {
        let mut vec = Vec::with_capacity(4);
        for direction in DIRECTIONS.iter() {
            let coord = *c + *direction;
            vec.push((coord, self.get(&coord)));
        }
        vec.into_iter()
    }

    pub fn get(&self, coord: &Coord) -> T {
        *self.cells.get(coord).unwrap_or(&Default::default())
    }

    pub fn set(&mut self, coord: Coord, value: T) {
        self.cells.insert(coord, value);
    }

    fn bounds(&self) -> GridBounds {
        let ((xmin, xmax), (ymin, ymax)) =
            self.cells
                .keys()
                .fold(((0, 0), (0, 0)), |((xmin, xmax), (ymin, ymax)), &(x, y)| {
                    ((min(x, xmin), max(x, xmax)), (min(y, ymin), max(y, ymax)))
                });

        GridBounds {
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }

    pub fn render(&self) -> String {
        let bounds = self.bounds();

        let mut out = String::new();
        for j in bounds.ymin..=(bounds.ymax + 1) {
            for i in bounds.xmin..=(bounds.xmax + 1) {
                out.push(self.get(&(i, j)).into());
            }
            out.push('\n');
        }
        out
    }

    pub fn initialized_count(&self) -> usize {
        self.cells.len()
    }
}

struct GridBounds {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Into<i64> for Direction {
    fn into(self) -> i64 {
        self as i64
    }
}

impl std::ops::Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, dir: Direction) -> Coord {
        match dir {
            Direction::North => (self.0, self.1 - 1),
            Direction::West => (self.0 - 1, self.1),
            Direction::South => (self.0, self.1 + 1),
            Direction::East => (self.0 + 1, self.1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Turn {
    CW,
    CCW,
}

impl Turn {
    pub fn apply(self, direction: Direction) -> Direction {
        match (direction, self) {
            (Direction::North, Turn::CW) => Direction::East,
            (Direction::East, Turn::CW) => Direction::South,
            (Direction::South, Turn::CW) => Direction::West,
            (Direction::West, Turn::CW) => Direction::North,

            (Direction::North, Turn::CCW) => Direction::West,
            (Direction::East, Turn::CCW) => Direction::North,
            (Direction::South, Turn::CCW) => Direction::East,
            (Direction::West, Turn::CCW) => Direction::South,
        }
    }
}
