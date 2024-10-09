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
    fn insert_recursive(node: &mut Option<Box<TreeNode<T>>>, value: T) {
        match node {
            Some(ref mut n) => {
                match value.cmp(&n.value) {
                    Ordering::Less => {
                        Self::insert_recursive(&mut n.left, value);
                    },
                    Ordering::Greater => {
                        Self::insert_recursive(&mut n.right, value);
                    },
                    Ordering::Equal => {}
                }
            },
            None => {
                node.replace(Box::new(TreeNode::new(value)));
            }
        }
    }
    fn search_recursive(node: Option<&TreeNode<T>>, value: &T) -> bool {
        match node {
            Some(n) => {
                match value.cmp(&n.value) {
                    Ordering::Less => Self::search_recursive(n.left.as_deref(), value),
                    Ordering::Greater => Self::search_recursive(n.right.as_deref(), value),
                    Ordering::Equal => true,
                }
            },
            None => false,
        }
    }

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        BinarySearchTree::<T>::insert_recursive(&mut self.root, value);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        BinarySearchTree::<T>::search_recursive(self.root.as_ref().map(|v| &**v), &value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(left) = &mut self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            },
            Ordering::Greater => {
                if let Some(right) = &mut self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            },
            Ordering::Equal => {}
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


