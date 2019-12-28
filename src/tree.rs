use std::collections::HashSet;
use std::hash::Hash;

/// Trait for implementing tree-traversal algorithms on any structure providing
/// a parent-of relation.
pub trait Tree<T: Clone + Copy + Eq + Hash> {
    /// Get the parent of `node`.
    fn parent(&self, node: T) -> Option<T>;

    /// Get all ancestors of `node`, up to the root of the tree.
    fn ancestors(&self, mut node: T) -> Vec<T> {
        let mut out: Vec<T> = vec![node];

        while let Some(parent) = self.parent(node) {
            out.push(parent);
            node = parent;
        }

        out
    }

    /// Find the shortest path from `start` to `end`, inclusive of both endpoints.
    fn shortest_path(&self, start: T, end: T) -> Vec<T> {
        let mut start_to_root = self.ancestors(start);

        // If `end` is on the path from `start` to the root, return prefix of that path.
        if let Some(n) = start_to_root.iter().position(|&item| item == end) {
            start_to_root.split_off(n + 1);
            return start_to_root;
        }

        // Otherwise, traverse up to the nearest common ancestor, then traverse back down.
        let end_to_root = self.ancestors(end);
        let common_ancestor: &T = {
            let candidates: HashSet<T> = start_to_root.iter().cloned().collect();
            end_to_root
                .iter()
                .find(|x| candidates.contains(x))
                .expect("No common ancestor!")
        };

        let prefix = start_to_root.iter().take_while(|&x| x != common_ancestor);
        let suffix = end_to_root
            .iter()
            .rev()
            .skip_while(|&x| x != common_ancestor);

        let out = prefix.chain(suffix).cloned().collect();

        out
    }
}
