/*! --- Day 15: Oxygen System ---

Out here in deep space, many things can go wrong. Fortunately, many of those
things have indicator lights. Unfortunately, one of those lights is lit: the
oxygen system for part of the ship has failed!

According to the readouts, the oxygen system must have failed days ago after a
rupture in oxygen tank two; that section of the ship was automatically sealed
once oxygen levels went dangerously low. A single remotely-operated repair
droid is your only option for fixing the oxygen system.

The Elves' care package included an Intcode program (your puzzle input) that
you can use to remotely control the repair droid. By running that program, you
can direct the repair droid to the oxygen system and fix the problem.

The remote control program executes the following steps in a loop forever:

Accept a movement command via an input instruction.
Send the movement command to the repair droid.
Wait for the repair droid to finish the movement operation.
Report on the status of the repair droid via an output instruction.

Only four movement commands are understood: north (1), south (2), west (3), and
east (4). Any other command is invalid. The movements differ in direction, but
not in distance: in a long enough east-west hallway, a series of commands like
4,4,4,4,3,3,3,3 would leave the repair droid back where it started.

The repair droid can reply with any of the following status codes:

0: The repair droid hit a wall. Its position has not changed.
1: The repair droid has moved one step in the requested direction.
2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.

You don't know anything about the area around the repair droid, but you can
figure it out by watching the status codes.

For example, we can draw the area using D for the droid, # for walls, . for
locations the droid can traverse, and empty space for unexplored
locations. Then, the initial state looks like this:



   D



To make the droid go north, send it 1. If it replies with 0, you know that
location is a wall and that the droid didn't move:


   #
   D


To move east, send 4; a reply of 1 means the movement was successful:


   #
   .D


Then, perhaps attempts to move north (1), south (2), and east (4) are all met with replies of 0:


   ##
   .D#
    #

Now, you know the repair droid is in a dead end. Backtrack with 3 (which you
already know will get a reply of 1 because you already know that location is
open):


   ##
   D.#
    #

Then, perhaps west (3) gets a reply of 0, south (2) gets a reply of 1, south
again (2) gets a reply of 0, and then west (3) gets a reply of 2:

   ##
  #..#
  D.#
   #

Now, because of the reply of 2, you know you've found the oxygen system! In
this example, it was only 2 moves away from the repair droid's starting
position.

What is the fewest number of movement commands required to move the repair
droid from its starting position to the location of the oxygen system?

--- Part Two ---

You quickly repair the oxygen system; oxygen gradually fills the area.

Oxygen starts in the location containing the repaired oxygen system. It takes
one minute for oxygen to spread to all open locations that are adjacent to a
location that already contains oxygen. Diagonal locations are not adjacent.

In the example above, suppose you've used the droid to explore the area fully
and have the following map (where locations that currently contain oxygen are
marked O):

 ##
#..##
#.#..#
#.O.#
 ###

Initially, the only location which contains oxygen is the location of the
repaired oxygen system. However, after one minute, the oxygen spreads to all
open (.) locations that are adjacent to a location containing oxygen:

 ##
#..##
#.#..#
#OOO#
 ###

After a total of two minutes, the map looks like this:

 ##
#..##
#O#O.#
#OOO#
 ###

After a total of three minutes:

 ##
#O.##
#O#OO#
#OOO#
 ###

And finally, the whole region is full of oxygen after a total of four minutes:

 ##
#OO##
#O#OO#
#OOO#
 ###

So, in this example, all locations contain oxygen after 4 minutes.

Use the repair droid to get a complete map of the area. How many minutes will
it take to fill with oxygen?

Although it hasn't changed, you can still get your puzzle input.

 */

use std::collections::{HashMap, VecDeque};

use crate::grid::{Coord, Direction, Grid};
use crate::intcode::{Program, IO};
use crate::tree::Tree;
use crate::utils::{ProblemInput, ProblemResult};

#[derive(Debug, Clone, Copy)]
enum MoveResult {
    HitWall = 0,
    Moved = 1,
    FoundOxygen = 2,
}

impl From<i64> for MoveResult {
    fn from(value: i64) -> MoveResult {
        match value {
            0 => MoveResult::HitWall,
            1 => MoveResult::Moved,
            2 => MoveResult::FoundOxygen,
            n => panic!("Invalid MoveResult: {}", n),
        }
    }
}

impl Into<i64> for MoveResult {
    fn into(self) -> i64 {
        self as i64
    }
}

fn direction_between(here: Coord, there: Coord) -> Direction {
    match (there.0 - here.0, there.1 - here.1) {
        (1, 0) => Direction::East,
        (-1, 0) => Direction::West,
        (0, -1) => Direction::North,
        (0, 1) => Direction::South,
        _ => {
            panic!("Invalid coords: here={:?}, there={:?}", here, there);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Unknown,
    Empty,
    Wall,
    Oxygen,
    Start,
}

impl Tile {
    fn passable(self) -> bool {
        match self {
            Tile::Wall => false,
            _ => true,
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Unknown => ' ',
            Tile::Empty => '.',
            Tile::Oxygen => 'O',
            Tile::Start => 'X',
        }
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::Unknown
    }
}

#[derive(Debug, Clone, Copy)]
struct SpanningTreeState {
    /// Parent of coord in the shortest path back to the root.
    parent: Option<Coord>,
    /// Depth of coordinate in the spanning tree.
    depth: u64,
}

#[derive(Debug)]
struct Droid {
    /// Grid of currently known tile states.
    grid: Grid<Tile>,

    /// Queue of unexplored locations that are known to be reachable.
    frontier: VecDeque<Coord>,

    /// Map from explored location to parent and depth from start.
    spanning_tree: HashMap<Coord, SpanningTreeState>,

    /// Sequence of planned moves, in order from from to back.
    plan: VecDeque<Coord>,

    /// Current location.
    location: Coord,

    /// The position we're currently trying to move to.
    moving_to: Coord,

    /// Location of the oxygen tank.
    oxygen: Option<Coord>,
}

impl Droid {
    fn new() -> Droid {
        let mut spanning_tree = HashMap::new();
        spanning_tree.insert(
            (0, 0),
            SpanningTreeState {
                parent: None,
                depth: 0,
            },
        );

        let mut droid = Droid {
            grid: Grid::empty(),
            frontier: VecDeque::new(),
            spanning_tree,
            plan: VecDeque::new(),
            location: (0, 0),
            moving_to: (0, 0),
            oxygen: None,
        };

        droid.mark_explored((0, 0), Tile::Start);

        droid
    }

    fn mark_explored(&mut self, loc: Coord, tile: Tile) {
        if self.grid.get(&loc) != Tile::Unknown {
            return;
        }

        self.grid.set(loc, tile);

        match tile {
            Tile::Wall => {}
            Tile::Empty | Tile::Start | Tile::Oxygen => {
                let depth = self
                    .spanning_tree
                    .get(&loc)
                    .expect(&format!("Failed to get spanning tree info for {:?}", loc))
                    .depth;

                for (coord, content) in self.grid.neighbors(&loc) {
                    match content {
                        Tile::Unknown => {
                            self.spanning_tree.insert(
                                coord,
                                SpanningTreeState {
                                    parent: Some(loc),
                                    depth: depth + 1,
                                },
                            );
                            self.frontier.push_back(coord);
                        }
                        _ => {} // Already explored here.
                    }
                }
            }
            Tile::Unknown => unreachable!("Got unknown after explore: {:?}", loc),
        }
    }

    fn pop_unexplored(&mut self) -> Option<Coord> {
        self.frontier.pop_front()
    }

    /// Compute the shortest path from self.location to goal.
    fn path_to(&self, goal: Coord) -> Vec<Coord> {
        self.shortest_path(self.location, goal)
            .into_iter()
            .collect()
    }

    fn tree_depth(&self, loc: &Coord) -> Option<u64> {
        self.spanning_tree.get(&loc).map(|state| state.depth)
    }

    fn passable_neighbors(&self, loc: Coord) -> Vec<Coord> {
        self.grid
            .neighbors(&loc)
            .filter_map(
                |(coord, tile)| {
                    if tile.passable() {
                        Some(coord)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }

    fn max_distance_from(&self, start: Coord) -> u64 {
        let mut tree: HashMap<Coord, SpanningTreeState> = HashMap::new();
        tree.insert(
            start,
            SpanningTreeState {
                parent: None,
                depth: 0,
            },
        );

        let mut queue = VecDeque::from(vec![start]);

        while let Some(parent) = queue.pop_front() {
            let parent_depth = tree.get(&parent).unwrap().depth;

            for child in self.passable_neighbors(parent) {
                if tree.get(&child).is_some() {
                    continue;
                }

                tree.insert(
                    child,
                    SpanningTreeState {
                        parent: Some(parent),
                        depth: parent_depth + 1,
                    },
                );
                queue.push_back(child);
            }
        }

        tree.values().map(|v| v.depth).max().unwrap()
    }
}

impl Tree<Coord> for Droid {
    fn parent(&self, node: Coord) -> Option<Coord> {
        match self.spanning_tree.get(&node) {
            Some(state) => state.parent,
            None => None,
        }
    }
}

impl IO for Droid {
    fn input(&mut self) -> Option<i64> {
        // Choose a new location to explore to, if necessary.
        if self.plan.len() == 0 {
            let new_goal = match self.pop_unexplored() {
                Some(loc) => loc,
                None => return None, // Nothing more to explore.
            };

            self.plan = self.path_to(new_goal).into_iter().collect();
            self.plan.pop_front(); // Path contains current location. Remove it.

            assert!(self.plan.len() > 0);
        }

        // Move toward the next location in the plan.
        let next_location = self.plan.pop_front().unwrap();
        self.moving_to = next_location;
        return Some(direction_between(self.location, next_location) as i64);
    }

    fn output(&mut self, value: i64) -> Option<()> {
        match MoveResult::from(value) {
            MoveResult::HitWall => {
                self.mark_explored(self.moving_to, Tile::Wall);
            }
            MoveResult::Moved => {
                self.mark_explored(self.moving_to, Tile::Empty);
                self.location = self.moving_to;
            }
            MoveResult::FoundOxygen => {
                self.mark_explored(self.moving_to, Tile::Oxygen);
                self.location = self.moving_to;
                self.oxygen = Some(self.location);
            }
        }

        Some(())
    }
}

pub fn run() -> ProblemResult<()> {
    let program = Program::for_problem(15)?;

    let mut droid = Droid::new();

    // We expect to end with error because we terminate by returning an error
    // status from an input() call.
    program.run(&mut droid).unwrap_err();

    println!("{}", droid.grid.render());

    match droid.oxygen {
        Some(ref loc) => {
            // Part 1.
            println!("Found oxygen at {:?}", loc);
            println!("Steps from entrance: {:?}", droid.tree_depth(loc).unwrap());

            // Part 2
            println!(
                "Max distance from oxygen: {:?}",
                droid.max_distance_from(*loc)
            );
        }
        None => panic!("Failed to find oxygen tank!"),
    }

    Ok(())
}
