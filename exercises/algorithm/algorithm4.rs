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
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match &mut self.root {
            Some(node) => {
                let mut cur_node = node;
                loop {
                    // println!("cur_node.value {} value {} cur_node.left {}", cur_node.value, value,cur_node.left.is_none());
                    if value < cur_node.value {
                        if cur_node.left.is_none() {
                            cur_node.left = Some(Box::new(TreeNode::new(value)));
                            break;
                        } else {
                            cur_node = cur_node.left.as_mut().ok_or(false).unwrap();
                        }
                    } else if value > cur_node.value {
                        if cur_node.right.is_none() {
                            cur_node.right = Some(Box::new(TreeNode::new(value)));
                            break;
                        } else {
                            cur_node = cur_node.right.as_mut().ok_or(false).unwrap();
                        }
                    } else {
                        break;
                    }
                }
            }
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
        //TODO
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match &self.root {
            Some(root) => {
                let mut cur_node = root;
                loop {
                    if cur_node.value == value {
                        return true;
                    } else if cur_node.value > value {
                        cur_node = if let Some(ref temp) = cur_node.left {
                            temp
                        } else {
                            return false;
                        };
                    } else {
                        cur_node = if let Some(ref temp) = cur_node.right {
                            temp
                        } else {
                            return false;
                        };
                    }
                }
            }
            None => {
                return false;
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
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
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
