/**
 * [144] Binary Tree Preorder Traversal
 *
 * Given a binary tree, return the preorder traversal of its nodes' values.
 *
 * Example:
 *
 *
 * Input: [1,null,2,3]
 *    1
 *     \
 *      2
 *     /
 *    3
 *
 * Output: [1,2,3]
 *
 *
 * Follow up: Recursive solution is trivial, could you do it iteratively?
 *
 */
use std::{
    rc::Rc,
    cell::RefCell
};
use crate::{TreeNode, tree};

// problem: https://leetcode.cn/problems/binary-tree-preorder-traversal/
pub struct Solution;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<i32> {
        if !root.is_some() {
            return vec![];
        }
        let mut ret = Vec::new();
        // recursively traverse
        // Self::hepler(root, &mut ret);
        
        let mut stack = Vec::new();
        stack.push(root);
        while !stack.is_empty() {
            if let Some(maybe_root) = stack.pop() {
                match maybe_root {
                    Some(root) => {
                        ret.push(root.borrow().val);
                        // stack.push(root.borrow().right.clone());
                        stack.push(root.borrow_mut().right.take());
                        stack.push(root.borrow().left.clone());
                    },
                    _ => {}
                }
            }            
        }

        ret
    }

    // recursive
    pub fn hepler(root: Option<Rc<RefCell<TreeNode<i32>>>>, ret: &mut Vec<i32>) {
        if let Some(n) = root {
            ret.push(n.borrow().val);
            Self::hepler(n.borrow().left.clone(), ret);
            Self::hepler(n.borrow().right.clone(), ret);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_144() {
        assert_eq!(
            Solution::preorder_traversal(tree![1, null, 2, 3]),
            vec![1, 2, 3]
        );
        assert_eq!(
            Solution::preorder_traversal(tree![1, null, 2]),
            vec![1, 2]
        );
    }
}