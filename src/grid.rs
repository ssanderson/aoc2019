use std::cmp::{max, min};
use std::collections::HashMap;

pub type Coord = (i64, i64);

pub trait GridElem: Default + Copy + Into<char> {}

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
        for j in (bounds.ymin..=(bounds.ymax + 1)).rev() {
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
