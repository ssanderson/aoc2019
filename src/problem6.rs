/// -- Day 6: Universal Orbit Map ---

/// You've landed at the Universal Orbit Map facility on Mercury. Because
/// navigation in space often involves transferring between orbits, the orbit
/// maps here are useful for finding efficient routes between, for example, you
/// and Santa. You download a map of the local orbits (your puzzle input).

/// Except for the universal Center of Mass (COM), every object in space is in
/// orbit around exactly one other object. An orbit looks roughly like this:

///                   \
///                    \
///                     |
///                     |
/// AAA--> o            o <--BBB
///                     |
///                     |
///                    /
///                   /

/// In this diagram, the object BBB is in orbit around AAA. The path that BBB
/// takes around AAA (drawn with lines) is only partly shown. In the map data,
/// this orbital relationship is written AAA)BBB, which means "BBB is in orbit
/// around AAA".

/// Before you use your map data to plot a course, you need to make sure it
/// wasn't corrupted during the download. To verify maps, the Universal Orbit
/// Map facility uses orbit count checksums - the total number of direct orbits
/// (like the one shown above) and indirect orbits.

/// Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain
/// can be any number of objects long: if A orbits B, B orbits C, and C orbits
/// D, then A indirectly orbits D.

/// For example, suppose you have the following map:

/// COM)B
/// B)C
/// C)D
/// D)E
/// E)F
/// B)G
/// G)H
/// D)I
/// E)J
/// J)K
/// K)L

/// Visually, the above map of orbits looks like this:

///         G - H       J - K - L
///        /           /
/// COM - B - C - D - E - F
///                \
///                 I

/// In this visual representation, when two objects are connected by a line,
/// the one on the right directly orbits the one on the left.

/// Here, we can count the total number of orbits as follows:

/// D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
/// L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.
/// COM orbits nothing.

/// The total number of direct and indirect orbits in this example is 42.

/// What is the total number of direct and indirect orbits in your map data?

/// --- Part Two ---

/// Now, you just need to figure out how many orbital transfers you (YOU) need
/// to take to get to Santa (SAN).

/// You start at the object YOU are orbiting; your destination is the object
/// SAN is orbiting. An orbital transfer lets you move from any object to an
/// object orbiting or orbited by that object.

/// For example, suppose you have the following map:

/// COM)B
/// B)C
/// C)D
/// D)E
/// E)F
/// B)G
/// G)H
/// D)I
/// E)J
/// J)K
/// K)L
/// K)YOU
/// I)SAN

/// Visually, the above map of orbits looks like this:

///                           YOU
///                          /
///         G - H       J - K - L
///        /           /
/// COM - B - C - D - E - F
///                \
///                 I - SAN

/// In this example, YOU are in orbit around K, and SAN is in orbit around
/// I. To move from K to I, a minimum of 4 orbital transfers are required:

/// K to J
/// J to E
/// E to D
/// D to I

/// Afterward, the map of orbits looks like this:

///         G - H       J - K - L
///        /           /
/// COM - B - C - D - E - F
///                \
///                 I - SAN
///                  \
///                   YOU

/// What is the minimum number of orbital transfers required to move from the
/// object YOU are orbiting to the object SAN is orbiting? (Between the objects
/// they are orbiting - not between YOU and SAN.)
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::tree::Tree;
use crate::utils;

#[derive(Debug)]
struct Orbits {
    /// Root node.
    root: String,

    /// Map from parent to children.
    children: HashMap<String, Vec<String>>,

    /// Map from child to parent.
    ancestors: HashMap<String, String>,
}

impl Orbits {
    pub fn from_pairs(pairs: Vec<(String, String)>) -> Orbits {
        let mut children: HashMap<String, Vec<String>> = HashMap::new();
        let mut ancestors: HashMap<String, String> = HashMap::new();

        for (parent, child) in pairs {
            match ancestors.insert(child.clone(), parent.clone()) {
                Some(_) => panic!("{:?} has multiple parents", child),
                None => {}
            }

            children.entry(parent).or_insert_with(|| vec![]).push(child);
        }

        let root = find_root(ancestors.keys().nth(0).unwrap(), &ancestors);

        Orbits {
            root: root.into(),
            children,
            ancestors,
        }
    }

    /// Find distances of all nodes from the root node.
    pub fn distances_from_root(&self) -> HashMap<String, u64> {
        find_depths(&self.root, &self.children)
    }
}

impl<'a> Tree<&'a str> for &'a Orbits {
    fn parent(&self, node: &'a str) -> Option<&'a str> {
        self.ancestors.get(node).map(|s| &s[..])
    }
}

fn find_depths(node: &str, tree: &HashMap<String, Vec<String>>) -> HashMap<String, u64> {
    let mut out: HashMap<String, u64> = HashMap::new();
    find_depths_helper(node, tree, &mut out, 0);

    out
}

fn find_depths_helper(
    node: &str,
    tree: &HashMap<String, Vec<String>>,
    out: &mut HashMap<String, u64>,
    depth: u64,
) {
    match out.insert(node.into(), depth) {
        Some(_) => panic!("Cycle in orbit graph at {}!", node),
        None => {
            for child in tree.get(node).unwrap_or(&vec![]) {
                find_depths_helper(child, tree, out, depth + 1)
            }
        }
    }
}

fn find_root<'a>(start: &'a str, ancestors: &'a HashMap<String, String>) -> &'a str {
    let mut node = start;

    while let Some(parent) = ancestors.get(node) {
        node = parent;
    }

    node
}

impl FromStr for Orbits {
    type Err = OrbitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Result<Vec<(String, String)>, OrbitParseError> = s
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split(')').collect();
                if parts.len() == 2 {
                    return Ok((parts[0].into(), parts[1].into()));
                } else {
                    return Err(OrbitParseError(line.into()));
                }
            })
            .collect();

        Ok(Orbits::from_pairs(parsed?))
    }
}

#[derive(Debug)]
struct OrbitParseError(String);

impl fmt::Display for OrbitParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for OrbitParseError {}

fn read_orbits(path: &Path) -> utils::ProblemResult<Orbits> {
    let file_content = fs::read_to_string(path)?;
    file_content.parse::<Orbits>().map_err(|e| e.into())
}

pub fn run() -> utils::ProblemResult<()> {
    let here = Path::new(file!()).parent().unwrap();
    let input_path = here.join("inputs/problem6_input.txt");

    match read_orbits(&input_path) {
        Ok(ref orbits) => {
            println!("Part 1");
            println!("------");
            let total: u64 = orbits.distances_from_root().values().sum();
            println!("Total Number of Orbits: {:?}", total);

            println!("Part 2");
            println!("------");

            // Find shortest path from YOU to the planet santa is orbiting.
            let target = orbits.parent("SAN").expect("Santa has no parent!");
            let path = orbits.shortest_path("YOU", target);
            println!("Shortest Path from YOU to parent(SAN): {:?}", path);

            // The number of transitions is the number of nodes in the path,
            // minus 2. To see this, consider that if we're already orbiting
            // the same planet as santa, the path from YOU to parent(SAN) has
            // length 2 ([YOU, parent(SAN)], and there are no hops to
            // perform. For each node between YOU and parent(SAN), we add one
            // node to the path, and one hop.
            println!("Number of Orbital Transitions: {}", path.len() - 2);

            Ok(())
        }
        Err(e) => utils::bail(format!("Error reading orbits:\n {}", e)),
    }
}
