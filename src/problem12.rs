/*!

--- Day 12: The N-Body Problem ---

The space near Jupiter is not a very safe place; you need to be careful of a
big distracting red spot, extreme radiation, and a whole lot of moons swirling
around. You decide to start by tracking the four largest moons: Io, Europa,
Ganymede, and Callisto.

After a brief scan, you calculate the position of each moon (your puzzle
input). You just need to simulate their motion so you can avoid them.

Each moon has a 3-dimensional position (x, y, and z) and a 3-dimensional
velocity. The position of each moon is given in your scan; the x, y, and z
velocity of each moon starts at 0.

Simulate the motion of the moons in time steps. Within each time step, first
update the velocity of every moon by applying gravity. Then, once all moons'
velocities have been updated, update the position of every moon by applying
velocity. Time progresses by one step once all of the positions are updated.

To apply gravity, consider every pair of moons. On each axis (x, y, and z), the
velocity of each moon changes by exactly +1 or -1 to pull the moons
together. For example, if Ganymede has an x position of 3, and Callisto has a x
position of 5, then Ganymede's x velocity changes by +1 (because 5 > 3) and
Callisto's x velocity changes by -1 (because 3 < 5). However, if the positions
on a given axis are the same, the velocity on that axis does not change for
that pair of moons.

Once all gravity has been applied, apply velocity: simply add the velocity of
each moon to its own position. For example, if Europa has a position of x=1,
y=2, z=3 and a velocity of x=-2, y=0,z=3, then its new position would be x=-1,
y=2, z=6. This process does not modify the velocity of any moon.

For example, suppose your scan reveals the following positions:

<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>

Simulating the motion of these moons would produce the following:

After 0 steps:
pos=<x=-1, y=  0, z= 2>, vel=<x= 0, y= 0, z= 0>
pos=<x= 2, y=-10, z=-7>, vel=<x= 0, y= 0, z= 0>
pos=<x= 4, y= -8, z= 8>, vel=<x= 0, y= 0, z= 0>
pos=<x= 3, y=  5, z=-1>, vel=<x= 0, y= 0, z= 0>

After 1 step:
pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>
pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>
pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>

After 2 steps:
pos=<x= 5, y=-3, z=-1>, vel=<x= 3, y=-2, z=-2>
pos=<x= 1, y=-2, z= 2>, vel=<x=-2, y= 5, z= 6>
pos=<x= 1, y=-4, z=-1>, vel=<x= 0, y= 3, z=-6>
pos=<x= 1, y=-4, z= 2>, vel=<x=-1, y=-6, z= 2>

After 3 steps:
pos=<x= 5, y=-6, z=-1>, vel=<x= 0, y=-3, z= 0>
pos=<x= 0, y= 0, z= 6>, vel=<x=-1, y= 2, z= 4>
pos=<x= 2, y= 1, z=-5>, vel=<x= 1, y= 5, z=-4>
pos=<x= 1, y=-8, z= 2>, vel=<x= 0, y=-4, z= 0>

After 4 steps:
pos=<x= 2, y=-8, z= 0>, vel=<x=-3, y=-2, z= 1>
pos=<x= 2, y= 1, z= 7>, vel=<x= 2, y= 1, z= 1>
pos=<x= 2, y= 3, z=-6>, vel=<x= 0, y= 2, z=-1>
pos=<x= 2, y=-9, z= 1>, vel=<x= 1, y=-1, z=-1>

After 5 steps:
pos=<x=-1, y=-9, z= 2>, vel=<x=-3, y=-1, z= 2>
pos=<x= 4, y= 1, z= 5>, vel=<x= 2, y= 0, z=-2>
pos=<x= 2, y= 2, z=-4>, vel=<x= 0, y=-1, z= 2>
pos=<x= 3, y=-7, z=-1>, vel=<x= 1, y= 2, z=-2>

After 6 steps:
pos=<x=-1, y=-7, z= 3>, vel=<x= 0, y= 2, z= 1>
pos=<x= 3, y= 0, z= 0>, vel=<x=-1, y=-1, z=-5>
pos=<x= 3, y=-2, z= 1>, vel=<x= 1, y=-4, z= 5>
pos=<x= 3, y=-4, z=-2>, vel=<x= 0, y= 3, z=-1>

After 7 steps:
pos=<x= 2, y=-2, z= 1>, vel=<x= 3, y= 5, z=-2>
pos=<x= 1, y=-4, z=-4>, vel=<x=-2, y=-4, z=-4>
pos=<x= 3, y=-7, z= 5>, vel=<x= 0, y=-5, z= 4>
pos=<x= 2, y= 0, z= 0>, vel=<x=-1, y= 4, z= 2>

After 8 steps:
pos=<x= 5, y= 2, z=-2>, vel=<x= 3, y= 4, z=-3>
pos=<x= 2, y=-7, z=-5>, vel=<x= 1, y=-3, z=-1>
pos=<x= 0, y=-9, z= 6>, vel=<x=-3, y=-2, z= 1>
pos=<x= 1, y= 1, z= 3>, vel=<x=-1, y= 1, z= 3>

After 9 steps:
pos=<x= 5, y= 3, z=-4>, vel=<x= 0, y= 1, z=-2>
pos=<x= 2, y=-9, z=-3>, vel=<x= 0, y=-2, z= 2>
pos=<x= 0, y=-8, z= 4>, vel=<x= 0, y= 1, z=-2>
pos=<x= 1, y= 1, z= 5>, vel=<x= 0, y= 0, z= 2>

After 10 steps:
pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>
pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>
pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>
pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>

Then, it might help to calculate the total energy in the system. The total
energy for a single moon is its potential energy multiplied by its kinetic
energy. A moon's potential energy is the sum of the absolute values of its x,
y, and z position coordinates. A moon's kinetic energy is the sum of the
absolute values of its velocity coordinates. Below, each line shows the
calculations for a moon's potential energy (pot), kinetic energy (kin), and
total energy:

Energy after 10 steps:
pot: 2 + 1 + 3 =  6;   kin: 3 + 2 + 1 = 6;   total:  6 * 6 = 36
pot: 1 + 8 + 0 =  9;   kin: 1 + 1 + 3 = 5;   total:  9 * 5 = 45
pot: 3 + 6 + 1 = 10;   kin: 3 + 2 + 3 = 8;   total: 10 * 8 = 80
pot: 2 + 0 + 4 =  6;   kin: 1 + 1 + 1 = 3;   total:  6 * 3 = 18

Sum of total energy: 36 + 45 + 80 + 18 = 179

In the above example, adding together the total energy for all moons after 10
steps produces the total energy in the system, 179.

Here's a second example:

<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>

Every ten steps of simulation for 100 steps produces:

After 0 steps:
pos=<x= -8, y=-10, z=  0>, vel=<x=  0, y=  0, z=  0>
pos=<x=  5, y=  5, z= 10>, vel=<x=  0, y=  0, z=  0>
pos=<x=  2, y= -7, z=  3>, vel=<x=  0, y=  0, z=  0>
pos=<x=  9, y= -8, z= -3>, vel=<x=  0, y=  0, z=  0>

After 10 steps:
pos=<x= -9, y=-10, z=  1>, vel=<x= -2, y= -2, z= -1>
pos=<x=  4, y= 10, z=  9>, vel=<x= -3, y=  7, z= -2>
pos=<x=  8, y=-10, z= -3>, vel=<x=  5, y= -1, z= -2>
pos=<x=  5, y=-10, z=  3>, vel=<x=  0, y= -4, z=  5>

After 20 steps:
pos=<x=-10, y=  3, z= -4>, vel=<x= -5, y=  2, z=  0>
pos=<x=  5, y=-25, z=  6>, vel=<x=  1, y=  1, z= -4>
pos=<x= 13, y=  1, z=  1>, vel=<x=  5, y= -2, z=  2>
pos=<x=  0, y=  1, z=  7>, vel=<x= -1, y= -1, z=  2>

After 30 steps:
pos=<x= 15, y= -6, z= -9>, vel=<x= -5, y=  4, z=  0>
pos=<x= -4, y=-11, z=  3>, vel=<x= -3, y=-10, z=  0>
pos=<x=  0, y= -1, z= 11>, vel=<x=  7, y=  4, z=  3>
pos=<x= -3, y= -2, z=  5>, vel=<x=  1, y=  2, z= -3>

After 40 steps:
pos=<x= 14, y=-12, z= -4>, vel=<x= 11, y=  3, z=  0>
pos=<x= -1, y= 18, z=  8>, vel=<x= -5, y=  2, z=  3>
pos=<x= -5, y=-14, z=  8>, vel=<x=  1, y= -2, z=  0>
pos=<x=  0, y=-12, z= -2>, vel=<x= -7, y= -3, z= -3>

After 50 steps:
pos=<x=-23, y=  4, z=  1>, vel=<x= -7, y= -1, z=  2>
pos=<x= 20, y=-31, z= 13>, vel=<x=  5, y=  3, z=  4>
pos=<x= -4, y=  6, z=  1>, vel=<x= -1, y=  1, z= -3>
pos=<x= 15, y=  1, z= -5>, vel=<x=  3, y= -3, z= -3>

After 60 steps:
pos=<x= 36, y=-10, z=  6>, vel=<x=  5, y=  0, z=  3>
pos=<x=-18, y= 10, z=  9>, vel=<x= -3, y= -7, z=  5>
pos=<x=  8, y=-12, z= -3>, vel=<x= -2, y=  1, z= -7>
pos=<x=-18, y= -8, z= -2>, vel=<x=  0, y=  6, z= -1>

After 70 steps:
pos=<x=-33, y= -6, z=  5>, vel=<x= -5, y= -4, z=  7>
pos=<x= 13, y= -9, z=  2>, vel=<x= -2, y= 11, z=  3>
pos=<x= 11, y= -8, z=  2>, vel=<x=  8, y= -6, z= -7>
pos=<x= 17, y=  3, z=  1>, vel=<x= -1, y= -1, z= -3>

After 80 steps:
pos=<x= 30, y= -8, z=  3>, vel=<x=  3, y=  3, z=  0>
pos=<x= -2, y= -4, z=  0>, vel=<x=  4, y=-13, z=  2>
pos=<x=-18, y= -7, z= 15>, vel=<x= -8, y=  2, z= -2>
pos=<x= -2, y= -1, z= -8>, vel=<x=  1, y=  8, z=  0>

After 90 steps:
pos=<x=-25, y= -1, z=  4>, vel=<x=  1, y= -3, z=  4>
pos=<x=  2, y= -9, z=  0>, vel=<x= -3, y= 13, z= -1>
pos=<x= 32, y= -8, z= 14>, vel=<x=  5, y= -4, z=  6>
pos=<x= -1, y= -2, z= -8>, vel=<x= -3, y= -6, z= -9>

After 100 steps:
pos=<x=  8, y=-12, z= -9>, vel=<x= -7, y=  3, z=  0>
pos=<x= 13, y= 16, z= -3>, vel=<x=  3, y=-11, z= -5>
pos=<x=-29, y=-11, z= -1>, vel=<x= -3, y=  7, z=  4>
pos=<x= 16, y=-13, z= 23>, vel=<x=  7, y=  1, z=  1>

Energy after 100 steps:

pot:  8 + 12 +  9 = 29;   kin: 7 +  3 + 0 = 10;   total: 29 * 10 = 290
pot: 13 + 16 +  3 = 32;   kin: 3 + 11 + 5 = 19;   total: 32 * 19 = 608
pot: 29 + 11 +  1 = 41;   kin: 3 +  7 + 4 = 14;   total: 41 * 14 = 574
pot: 16 + 13 + 23 = 52;   kin: 7 +  1 + 1 =  9;   total: 52 *  9 = 468

Sum of total energy: 290 + 608 + 574 + 468 = 1940

What is the total energy in the system after simulating the moons given in your
scan for 1000 steps?

--- Part Two ---

All this drifting around in space makes you wonder about the nature of the
universe. Does history really repeat itself? You're curious whether the moons
will ever return to a previous state.

Determine the number of steps that must occur before all of the moons'
positions and velocities exactly match a previous point in time.

For example, the first example above takes 2772 steps before they exactly match
a previous point in time; it eventually returns to the initial state:

After 0 steps:

pos=<x= -1, y=  0, z=  2>, vel=<x=  0, y=  0, z=  0>
pos=<x=  2, y=-10, z= -7>, vel=<x=  0, y=  0, z=  0>
pos=<x=  4, y= -8, z=  8>, vel=<x=  0, y=  0, z=  0>
pos=<x=  3, y=  5, z= -1>, vel=<x=  0, y=  0, z=  0>

After 2770 steps:

pos=<x=  2, y= -1, z=  1>, vel=<x= -3, y=  2, z=  2>
pos=<x=  3, y= -7, z= -4>, vel=<x=  2, y= -5, z= -6>
pos=<x=  1, y= -7, z=  5>, vel=<x=  0, y= -3, z=  6>
pos=<x=  2, y=  2, z=  0>, vel=<x=  1, y=  6, z= -2>

After 2771 steps:

pos=<x= -1, y=  0, z=  2>, vel=<x= -3, y=  1, z=  1>
pos=<x=  2, y=-10, z= -7>, vel=<x= -1, y= -3, z= -3>
pos=<x=  4, y= -8, z=  8>, vel=<x=  3, y= -1, z=  3>
pos=<x=  3, y=  5, z= -1>, vel=<x=  1, y=  3, z= -1>

After 2772 steps:

pos=<x= -1, y=  0, z=  2>, vel=<x=  0, y=  0, z=  0>
pos=<x=  2, y=-10, z= -7>, vel=<x=  0, y=  0, z=  0>
pos=<x=  4, y= -8, z=  8>, vel=<x=  0, y=  0, z=  0>
pos=<x=  3, y=  5, z= -1>, vel=<x=  0, y=  0, z=  0>

Of course, the universe might last for a very long time before
repeating. Here's a copy of the second example from above:

<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>

This set of initial positions takes 4686774924 steps before it repeats a
previous state! Clearly, you might need to find a more efficient way to
simulate the universe.

How many steps does it take to reach the first state that exactly matches a
previous state?

 */

use std::collections::HashMap;

use crate::utils::ProblemResult;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn zero() -> Vec3 {
        Vec3 { x: 0, y: 0, z: 0 }
    }

    fn normalize(self) -> Vec3 {
        use std::cmp::Ordering;
        let norm = |x: i64| match x.cmp(&0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        Vec3 {
            x: norm(self.x),
            y: norm(self.y),
            z: norm(self.z),
        }
    }
}

impl std::ops::Index<Axis> for Vec3 {
    type Output = i64;

    fn index(&self, axis: Axis) -> &Self::Output {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl std::ops::IndexMut<Axis> for Vec3 {
    fn index_mut(&mut self, axis: Axis) -> &mut Self::Output {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

#[derive(Debug, Clone)]
struct Moons {
    positions: [Vec3; 4],
    velocities: [Vec3; 4],
}

impl Moons {
    pub fn new(positions: [Vec3; 4]) -> Moons {
        Moons {
            positions,
            velocities: [Vec3::zero(); 4],
        }
    }

    pub fn step(&mut self) {
        // To apply gravity, consider every pair of moons. On each axis (x, y,
        // and z), the velocity of each moon changes by exactly +1 or -1 to
        // pull the moons together.

        for i in 0..4 {
            for j in i + 1..4 {
                let delta = (self.positions[i] - self.positions[j]).normalize();

                self.velocities[i] -= delta;
                self.velocities[j] += delta;
            }
        }

        // Once all gravity has been applied, apply velocity: simply add the
        // velocity of each moon to its own position.
        for i in 0..4 {
            self.positions[i] += self.velocities[i];
        }
    }

    pub fn total_energy(&self) -> i64 {
        (0..4).map(|i| self.total_energy_for(i)).sum()
    }

    fn total_energy_for(&self, i: usize) -> i64 {
        // A moon's potential energy is the sum of the absolute values of its
        // x, y, and z position coordinates.
        let pos = self.positions[i];
        let potential = pos.x.abs() + pos.y.abs() + pos.z.abs();

        let vel = self.velocities[i];
        let kinetic = vel.x.abs() + vel.y.abs() + vel.z.abs();

        potential * kinetic
    }

    pub fn axis_state(&self, axis: Axis) -> [i64; 8] {
        [
            self.positions[0][axis],
            self.positions[1][axis],
            self.positions[2][axis],
            self.positions[3][axis],
            self.velocities[0][axis],
            self.velocities[1][axis],
            self.velocities[2][axis],
            self.velocities[3][axis],
        ]
    }
}

#[derive(Debug)]
enum AxisTrace {
    NotRepeated(HashMap<[i64; 8], u64>),
    Repeated {
        first: u64,
        second: u64,
        state: [i64; 8],
    },
}

#[derive(Debug)]
struct Trace {
    pub x: AxisTrace,
    pub y: AxisTrace,
    pub z: AxisTrace,
    pub count: u64,
}

impl Trace {
    fn new() -> Trace {
        Trace {
            x: AxisTrace::NotRepeated(HashMap::new()),
            y: AxisTrace::NotRepeated(HashMap::new()),
            z: AxisTrace::NotRepeated(HashMap::new()),
            count: 0,
        }
    }

    pub fn record(&mut self, moons: &Moons) -> bool {
        let done = [Axis::X, Axis::Y, Axis::Z]
            .iter()
            .map(|&axis| self.update_axis(moons, axis))
            .all(|x| x);

        self.count += 1;

        done
    }

    fn update_axis(&mut self, moons: &Moons, axis: Axis) -> bool {
        let count = self.count;
        let axis_trace = self.for_axis_mut(axis);

        if let AxisTrace::NotRepeated(ref mut states) = axis_trace {
            let state = moons.axis_state(axis);
            match states.insert(state, count) {
                // We've seen this state before. Record the count at which we
                // first saw it, and the current count. From here on out, this
                // axis will just cycle between these states.
                Some(prev_count) => {
                    *axis_trace = AxisTrace::Repeated {
                        first: prev_count,
                        second: count,
                        state,
                    };
                }
                None => {}
            }
        }

        match axis_trace {
            AxisTrace::NotRepeated(..) => false,
            AxisTrace::Repeated { .. } => true,
        }
    }

    fn cycle_length(&self, axis: Axis) -> u64 {
        match self.for_axis(axis) {
            AxisTrace::Repeated { first, second, .. } => *second - *first,
            AxisTrace::NotRepeated(..) => panic!("Axis {:?} has not repeated.", axis),
        }
    }

    fn cycle_lengths(self) -> (u64, u64, u64) {
        (
            self.cycle_length(Axis::X),
            self.cycle_length(Axis::Y),
            self.cycle_length(Axis::Z),
        )
    }

    fn for_axis_mut(&mut self, axis: Axis) -> &mut AxisTrace {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }

    fn for_axis(&self, axis: Axis) -> &AxisTrace {
        match axis {
            Axis::X => & self.x,
            Axis::Y => & self.y,
            Axis::Z => & self.z,
        }
    }
}

fn lcm(n: u64, m: u64) -> u64 {
    (n * m) / gcd(n, m)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    use std::cmp::Ordering;

    loop {
        match n.cmp(&m) {
            Ordering::Equal => return n,
            Ordering::Greater => {
                n = n - m;
            }
            Ordering::Less => {
                m = m - n;
            }
        }
    }
}

pub fn run() -> ProblemResult<()> {
    let moons = Moons::new([
        Vec3::new(-13, -13, -13),
        Vec3::new(5, -8, 3),
        Vec3::new(-6, -10, -3),
        Vec3::new(0, 5, -5),
    ]);

    // Part 1
    {
        let mut moons = moons.clone();
        for _ in 0..1000 {
            moons.step();
        }
        println!("Total Energy: {}", moons.total_energy());
    }

    // Part 2
    {
        let mut moons = moons.clone();
        let mut trace = Trace::new();

        // Record until we've seen a repeat along all three axes.
        while !trace.record(&moons) {
            moons.step();
        }

        let (xlen, ylen, zlen) = trace.cycle_lengths();
        println!("Calculating gcd");
        let total = lcm(lcm(xlen, ylen), zlen);

        println!("State repeats after {} steps.", total);
    }

    Ok(())
}
