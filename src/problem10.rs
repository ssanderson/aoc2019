/*! -- Day 10: Monitoring Station ---

You fly into the asteroid belt and reach the Ceres monitoring station. The
Elves here have an emergency: they're having trouble tracking all of the
asteroids and can't be sure they're safe.

The Elves would like to build a new monitoring station in a nearby area of
space; they hand you a map of all of the asteroids in that region (your puzzle
input).

The map indicates whether each position is empty (.) or contains an asteroid
(#). The asteroids are much smaller than they appear on the map, and every
asteroid is exactly in the center of its marked position. The asteroids can be
described with X,Y coordinates where X is the distance from the left edge and Y
is the distance from the top edge (so the top-left corner is 0,0 and the
position immediately to its right is 1,0).

Your job is to figure out which asteroid would be the best place to build a new
monitoring station. A monitoring station can detect any asteroid to which it
has direct line of sight - that is, there cannot be another asteroid exactly
between them. This line of sight can be at any angle, not just lines aligned to
the grid or diagonally. The best location is the asteroid that can detect the
largest number of other asteroids.

For example, consider the following map:
.#..#
.....
#####
....#
...##

The best location for a new monitoring station on this map is the highlighted
asteroid at 3,4 because it can detect 8 asteroids, more than any other
location. (The only asteroid it cannot detect is the one at 1,0; its view of
this asteroid is blocked by the asteroid at 2,2.) All other asteroids are worse
locations; they can detect 7 or fewer other asteroids. Here is the number of
other asteroids a monitoring station on each asteroid could detect:

.7..7
.....
67775
....7
...87

Here is an asteroid (#) and some examples of the ways its line of sight might
be blocked. If there were another asteroid at the location of a capital letter,
the locations marked with the corresponding lowercase letter would be blocked
and could not be detected:

#.........
...A......
...B..a...
.EDCG....a
..F.c.b...
.....c....
..efd.c.gb
.......c..
....f...c.
...e..d..c

Here are some larger examples:

Best is 5,8 with 33 other asteroids detected:

......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####

Best is 1,2 with 35 other asteroids detected:

#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.

Best is 6,3 with 41 other asteroids detected:

.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..

Best is 11,13 with 210 other asteroids detected:

.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##

Find the best location for a new monitoring station. How many other asteroids
can be detected from that location?

--- Part Two ---

Once you give them the coordinates, the Elves quickly deploy an Instant
Monitoring Station to the location and discover the worst: there are simply too
many asteroids.

The only solution is complete vaporization by giant laser.

Fortunately, in addition to an asteroid scanner, the new monitoring station
also comes equipped with a giant rotating laser perfect for vaporizing
asteroids. The laser starts by pointing up and always rotates clockwise,
vaporizing any asteroid it hits.

If multiple asteroids are exactly in line with the station, the laser only has
enough power to vaporize one of them before continuing its rotation. In other
words, the same asteroids that can be detected can be vaporized, but if
vaporizing one asteroid makes another one detectable, the newly-detected
asteroid won't be vaporized until the laser has returned to the same position
by rotating a full 360 degrees.

For example, consider the following map, where the asteroid with the new
monitoring station (and laser) is marked X:

.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##

The first nine asteroids to get vaporized, in order, would be:

.#....###24...#..
##...##.13#67..9#
##...#...5.8####.
..#.....X...###..
..#.#.....#....##

Note that some asteroids (the ones behind the asteroids marked 1, 5, and 7)
won't have a chance to be vaporized until the next full rotation. The laser
continues rotating; the next nine to be vaporized are:

.#....###.....#..
##...##...#.....#
##...#......1234.
..#.....X...5##..
..#.9.....8....76

The next nine to be vaporized are then:

.8....###.....#..
56...9#...#.....#
34...7...........
..2.....X....##..
..1..............

Finally, the laser completes its first full rotation (1 through 3), a second
rotation (4 through 8), and vaporizes the last asteroid (9) partway through its
third rotation:

......234.....6..
......1...5.....7
.................
........X....89..
.................

In the large example above (the one with the best monitoring station location
at 11,13):

The 1st asteroid to be vaporized is at 11,12.
The 2nd asteroid to be vaporized is at 12,1.
The 3rd asteroid to be vaporized is at 12,2.
The 10th asteroid to be vaporized is at 12,8.
The 20th asteroid to be vaporized is at 16,0.
The 50th asteroid to be vaporized is at 16,9.
The 100th asteroid to be vaporized is at 10,16.
The 199th asteroid to be vaporized is at 9,6.
The 200th asteroid to be vaporized is at 8,2.
The 201st asteroid to be vaporized is at 10,9.
The 299th and final asteroid to be vaporized is at 11,1.

The Elves are placing bets on which will be the 200th asteroid to be
vaporized. Win the bet by determining which asteroid that will be; what do you
get if you multiply its X coordinate by 100 and then add its Y coordinate? (For
example, 8,2 becomes 802.)

 */

use std::collections::HashSet;

use crate::utils::{ProblemInput, ProblemResult, SimpleError};

use std::convert::TryInto;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Space {
    Asteroid,
    Empty,
}

type Coord = (usize, usize);

#[derive(Debug)]
struct AsteroidMap {
    asteroids: HashSet<Coord>,
    pub nrows: usize,
    pub ncols: usize,
}

impl AsteroidMap {
    pub fn new(asteroids: HashSet<Coord>, nrows: usize, ncols: usize) -> AsteroidMap {
        AsteroidMap {
            asteroids,
            nrows,
            ncols,
        }
    }

    pub fn best_station_coord(&self) -> (Coord, u64) {
        self.asteroids
            .iter()
            .map(|&c| (c, self.num_visible_from(c)))
            .max_by_key(|(_c, v)| *v)
            .unwrap()
    }

    pub fn destruction_order(mut self, coord: Coord) -> Vec<Coord> {
        let directions = all_directions(self.nrows, self.ncols);

        let mut out = Vec::new();

        // > 1 because we never destroy ``coord``.
        while self.asteroids.len() > 1 {
            for dir in directions.iter() {
                match self.cast_ray(coord, *dir) {
                    Some(destroyed) => {
                        out.push(destroyed);
                        self.asteroids.remove(&destroyed);
                    }
                    None => {}
                }
            }
        }

        out
    }

    /// Count number of asteroids visible from a cell.
    fn num_visible_from(&self, coord: Coord) -> u64 {
        let mut count = 0;

        for direction in all_directions(self.nrows, self.ncols) {
            if let Some(_) = self.cast_ray(coord, direction) {
                count += 1;
            }
        }

        count
    }

    fn cast_ray(&self, (mut x, mut y): Coord, (dx, dy): (i64, i64)) -> Option<Coord> {
        loop {
            let new_x: usize = ((x as i64) + dx).try_into().ok()?;
            let new_y: usize = ((y as i64) + dy).try_into().ok()?;

            match self.at((new_x, new_y)) {
                Some(Space::Asteroid) => return Some((new_x, new_y)),
                Some(Space::Empty) => {
                    x = new_x;
                    y = new_y;
                }
                None => return None,
            }
        }
    }

    fn at(&self, (x, y): Coord) -> Option<Space> {
        if x >= self.ncols || y >= self.nrows {
            None
        } else {
            if self.asteroids.contains(&(x, y)) {
                Some(Space::Asteroid)
            } else {
                Some(Space::Empty)
            }
        }
    }
}

impl FromStr for AsteroidMap {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<Space>> = s
            .trim()
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|c| match c {
                        b'#' => Space::Asteroid,
                        b'.' => Space::Empty,
                        _ => panic!("Bad asteroid character: {}", c),
                    })
                    .collect()
            })
            .collect();

        let lengths = lines.iter().map(|v| v.len()).collect::<HashSet<usize>>();

        if lengths.len() != 1 {
            Err(SimpleError(format!(
                "Line lengths not unique: {:?}",
                lengths
            )))
        } else {
            let mut asteroids = HashSet::new();
            for (j, line) in lines.iter().enumerate() {
                for (i, value) in line.iter().enumerate() {
                    match value {
                        Space::Asteroid => {
                            asteroids.insert((i, j));
                        }
                        Space::Empty => {}
                    }
                }
            }

            let nrows = lines.len();
            let ncols = *lengths.iter().nth(0).unwrap();
            Ok(AsteroidMap::new(asteroids, nrows, ncols))
        }
    }
}

fn all_directions(nrows: usize, ncols: usize) -> Vec<(i64, i64)> {
    // All possible positive ratios that can be expressed on a grid of shape
    // (nrows, ncols).
    let mut positive_ratios: Vec<(i64, i64)> = (0..ncols)
        .flat_map(|dx| (0..nrows).map(move |dy| (dx, dy)))
        .filter_map(|(dx, dy)| match (dx, dy) {
            (0, 0) => None,
            (0, _) => Some((0, 1)),
            (_, 0) => Some((1, 0)),
            (n, m) => {
                match gcd(n, m) {
                    1 => Some((n, m)),
                    // Filter out non-relatively-prime pairs.
                    _ => None,
                }
            }
        })
        .map(|(dx, dy)| (dx as i64, dy as i64))
        .collect();

    positive_ratios.sort_by(|&(dx1, dy1), &(dx2, dy2)| {
        let slope1 = (dy1 as f64) / (dx1 as f64);
        let slope2 = (dy2 as f64) / (dx2 as f64);
        slope1.partial_cmp(&slope2).unwrap()
    });
    positive_ratios.dedup();

    // The y coordinate is negated from what you'd normally expect because
    // that's the coordinate system given in the problem. The first and third
    // quadrants are reversed because positive_ratios are sorted by
    // **increasing** absolute value of slope, but in quadrants 1 and 3, we
    // want to go in decreasing order.
    let first = positive_ratios
        .iter()
        .copied()
        .rev()
        .map(|(dx, dy)| (dx, -dy));
    let second = positive_ratios.iter().copied();
    let third = positive_ratios
        .iter()
        .rev()
        .copied()
        .map(|(dx, dy)| (-dx, dy));
    let fourth = positive_ratios.iter().copied().map(|(dx, dy)| (-dx, -dy));

    let mut out: Vec<(i64, i64)> = first.chain(second).chain(third).chain(fourth).collect();

    // XXX: This is super ugly. Remove duplicates due to first and last elements being the same.
    out.dedup();
    out.pop();
    out
}

fn gcd(n: usize, m: usize) -> usize {
    use std::cmp::Ordering;

    match n.cmp(&m) {
        Ordering::Equal => n,
        Ordering::Greater => gcd(n - m, m),
        Ordering::Less => gcd(n, m - n),
    }
}

mod tests {
    #[test]
    fn test_gcd() {
        use super::gcd;

        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(100, 100), 100);

        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(4, 2), 2);

        assert_eq!(gcd(3, 6), 3);
        assert_eq!(gcd(6, 3), 3);

        assert_eq!(gcd(24, 18), 6);
        assert_eq!(gcd(18, 24), 6);
    }
}

pub fn run() -> ProblemResult<()> {
    let map = AsteroidMap::for_problem(10)?;

    // Part 1
    let (station, max_visible) = map.best_station_coord();
    println!("Max visibility: {} at {:?}", max_visible, station);

    // Part 2
    let coords = map.destruction_order(station);
    println!("200th asteroid: {:?}", coords[199]);

    Ok(())
}
