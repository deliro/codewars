// mod preloaded;

// use preloaded::Node;
use std::collections::VecDeque;

struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut res = vec![];
    let mut q = VecDeque::new();
    q.push_front(root);
    while let Some(x) = q.pop_back() {
        res.push(x.value);
        if let Some(l) = &x.left {
            q.push_front(&*l)
        }
        if let Some(r) = &x.right {
            q.push_front(&*r)
        }
    }
    res
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

// Use the builder pattern to create your own tests:
//   let root = Node::new(1)           // create root
//              .left(Node::new(2))    // chain left child (returns root)
//              .right(Node::new(3));  // chain right child (returns root)

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn root_only() {
        assert_eq!(
            tree_by_levels(&Node::new(42)),
            [42],
            "\nYour result (left) didn't match the expected output (right)."
        );
    }

    #[test]
    fn complete_tree() {
        let root = Node::new(1)
            .left(Node::new(2).left(Node::new(4)).right(Node::new(5)))
            .right(Node::new(3).left(Node::new(6)));
        assert_eq!(
            tree_by_levels(&root),
            [1, 2, 3, 4, 5, 6],
            "\nYour result (left) didn't match the expected output (right)."
        );
    }
}
