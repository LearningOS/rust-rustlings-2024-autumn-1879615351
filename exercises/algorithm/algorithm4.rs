/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
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
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Copy,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Helper function to insert a value into the BST starting from a given node
    fn insert_impl(node: &mut Box<TreeNode<T>>, value: T) {
        match value.cmp(&node.value) {
            Ordering::Less => {
                if node.left.is_none() {
                    node.left = Some(Box::new(TreeNode::new(value)));
                } else {
                    // Recurse down the left subtree
                    Self::insert_impl(node.left.as_mut().unwrap(), value);
                }
            }
            Ordering::Greater => {
                if node.right.is_none() {
                    node.right = Some(Box::new(TreeNode::new(value)));
                } else {
                    // Recurse down the right subtree
                    Self::insert_impl(node.right.as_mut().unwrap(), value);
                }
            }
            Ordering::Equal => {
                // Value is already present in the tree, no action needed
            }
        }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => {
                // If root exists, use the helper function to insert the value
                Self::insert_impl(node, value);
            }
            None => {
                // If root is None, create a new node as the root
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Less => {
                    current = &node.left;
                }
                Ordering::Greater => {
                    current = &node.right;
                }
                Ordering::Equal => {
                    return true;
                }
            }
        }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree (used in the recursive case)
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left_node) = self.left {
                    left_node.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right_node) = self.right {
                    right_node.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {
                // Value is already in the tree, do nothing
            }
        }
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

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


