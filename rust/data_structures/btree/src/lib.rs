/// The default number of keys that a node can have.
const DEFAULT_MAX_NODE_KEYS: usize = 3;

pub struct Btree<K, V>
where
    K: PartialOrd,
{
    len: usize,
    max_node_keys: usize,
    root_node: Node<K, V>,
}

impl<K, V> std::fmt::Debug for Btree<K, V>
where
    K: PartialOrd + std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Btree")
            .field("len", &self.len)
            .field("max_node_keys", &self.max_node_keys)
            .field("root_node", &self.root_node)
            .finish()
    }
}

/*

                   &self.children
                       .iter()
                       .map(|ptr| unsafe { &**ptr })
                       .collect::<Vec<_>>(),
*/
impl<K, V> std::fmt::Debug for Node<K, V>
where
    K: PartialOrd + std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("keys", &self.keys)
            .field("values", &self.values)
            .field(
                "children",
                &self
                    .children
                    .iter()
                    .map(|ptr| unsafe { &**ptr })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

struct Node<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<*mut Node<K, V>>,
}

impl<K, V> Node<K, V>
where
    K: PartialEq,
{
    fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl<K, V> Btree<K, V>
where
    K: PartialOrd + Clone + std::fmt::Debug,
    V: std::fmt::Debug,
{
    pub fn new() -> Self {
        Btree::with_max_node_keys(DEFAULT_MAX_NODE_KEYS)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Instantiates a tree where `n` is the maximum number of keys a node can have.
    pub fn with_max_node_keys(n: usize) -> Self {
        Self {
            len: 0,
            max_node_keys: n,
            root_node: Node::new(),
        }
    }

    pub fn insert(&mut self, new_key: K, new_value: V) {
        unsafe {
            let mut stack = Vec::new();

            let mut leaf_node = &mut self.root_node as *mut Node<K, V>;
            loop {
                let node = &mut (*leaf_node);
                // Node is a leaf.

                let i = binary_search_traversal_position(&node.keys, &new_key);

                if node.is_leaf() {
                    break;
                }

                stack.push(leaf_node);

                leaf_node = node.children[i];
            }

            let leaf_node = &mut (*leaf_node);

            let i = binary_search_traversal_position(&leaf_node.keys, &new_key);

            leaf_node.keys.insert(i, new_key);
            leaf_node.values.insert(i, new_value);

            self.len += 1;

            let mut node_to_check_for_overlow = leaf_node;

            // While the node is full, split it.
            while node_to_check_for_overlow.keys.len() == self.max_node_keys {
                let middle = self.max_node_keys / 2;

                let mut right_node = Node::new();

                for _ in 0..middle {
                    right_node
                        .keys
                        .push(node_to_check_for_overlow.keys.pop().unwrap());
                    right_node
                        .values
                        .push(node_to_check_for_overlow.values.pop().unwrap());
                }
                right_node.keys.reverse();
                right_node.values.reverse();

                let middle_key = node_to_check_for_overlow.keys.pop().unwrap();
                let middle_value = node_to_check_for_overlow.values.pop().unwrap();

                let mut left_node = Node::new();
                for _ in 0..middle {
                    left_node
                        .keys
                        .push(node_to_check_for_overlow.keys.pop().unwrap());
                    left_node
                        .values
                        .push(node_to_check_for_overlow.values.pop().unwrap());
                }
                left_node.keys.reverse();
                left_node.values.reverse();

                // Get the parent of the current node.
                match stack.pop() {
                    // This node is the root node so there's no parent.
                    None => {
                        // Create a new root that points to the new nodes.
                        self.root_node = Node {
                            keys: vec![middle_key],
                            values: vec![middle_value],
                            children: vec![
                                Box::leak(Box::new(left_node)),
                                Box::leak(Box::new(right_node)),
                            ],
                        };

                        break;
                    }
                    Some(parent) => {
                        let parent = &mut (*parent);

                        let i = binary_search_insertion_position(&parent.keys, &middle_key);

                        parent.keys.insert(i, middle_key);
                        parent.values.insert(i, middle_value);

                        parent.children.insert(i, Box::leak(Box::new(left_node)));
                        parent
                            .children
                            .insert(i + 1, Box::leak(Box::new(right_node)));

                        // Check if the parent became full after inserting the new children and needs to be split at the next loop iteration.
                        node_to_check_for_overlow = parent;
                    }
                }
            }
        }
    }
}

fn binary_search_traversal_position<K>(xs: &[K], x: &K) -> usize
where
    K: PartialOrd + std::fmt::Debug,
{
    let mut start = 0;
    let mut end = xs.len();

    while start < end {
        let mid = (start + end) / 2;

        let value = &xs[mid];

        if value == x {
            return mid;
        }

        if x < value {
            if mid == 0 {
                // Loop would break on next iteration but 0_usize - 1 will panic.
                break;
            }
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    start
}

fn binary_search_insertion_position<K>(xs: &[K], x: &K) -> usize
where
    K: PartialOrd + std::fmt::Debug,
{
    let mut start = 0;
    let mut end = xs.len();

    while start < end {
        let mid = (start + end) / 2;

        let value = &xs[mid];

        if value == x {
            return mid;
        }

        if x < value {
            if mid == 0 {
                // Loop would break on next iteration but 0_usize - 1 will panic.
                break;
            }
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    end
}

#[cfg(test)]
mod btree_tests {
    use super::*;

    use proptest::prelude::*;

    #[derive(Debug, proptest_derive::Arbitrary)]
    enum Op {
        Insert(String, String),
    }

    proptest! {
        #[test]
        fn model(ops: Vec<Op>) {
            let mut model = Vec::new();
            let mut tree = Btree::new();

            for op in ops {
                match op {
                    Op::Insert(key, value) => {
                        model.push((key.clone(), value.clone()));
                        tree.insert(key, value);
                        assert_eq!(model.len(), tree.len());
                    }
                }
            }
        }
    }

    #[test]
    fn insert() {
        let mut tree = Btree::new();

        tree.insert("", "");
        tree.insert("a", "");
        tree.insert("𐭸", "");
        tree.insert("𖭐", "");
        tree.insert("\u{dd6}", "");
        tree.insert("b", "");
        tree.insert("ͺ", "");
        tree.insert("", "");

        dbg!(&tree);
    }
}
