/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}


#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}


impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // Insert a node into the tree
    fn insert(&mut self, new_node: Box<TreeNode<T>>) {
        if new_node.value < self.value {
            match self.left {
                Some(ref mut left) => left.insert(new_node),
                None => self.left = Some(new_node),
            }
        } else if new_node.value > self.value {
            match self.right {
                Some(ref mut right) => right.insert(new_node),
                None => self.right = Some(new_node),
            }
        } else {
            // println!("Duplicate value: {}", new_node.value);
        }
    }
}


impl<T> BinarySearchTree<T>
where
    T: Ord + std::fmt::Debug,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let new_node = Box::new(TreeNode::new(value));
        match self.root {
            Some(ref mut node) => node.insert(new_node),
            None => self.root = Some(new_node),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut current = &self.root;
        while let Some(ref node) = current {
            if node.value == value {
                return true;
            } else if value < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }


    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        println!("{:?}", bst.root);
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}

