use std::collections::HashMap;
use std::hash::Hash;

struct TreeNode<T> {
    node: T,
    children: Vec<TreeNode>,
}

trait Graph<'a, T: Hash + Eq> {
    fn edges(&self, node: &T) -> Vec<&'a T>;

    fn spanning_tree(&self, root: &'a T) -> SpanningTree<&'a T> {
        let mut out = TreeNode {
            node: root,
            children: vec![],
        };

        // let mut seen = HashMap::new();
        // tree.insert(root, &mut out);

        let mut queue = VecDeque::from(vec![root]);

        while let Some(parent) = queue.pop_front() {
            for child in self.edges(parent) {
                if seen.contains_key(child) {
                    continue;
                }

                seen.get(parent).unwrap().children.push(child)

                tree.insert(child, TreeNode {
            }
        }
    }
}
