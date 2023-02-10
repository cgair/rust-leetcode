/**
 * [226] Invert Binary Tree
 *
 * Invert a binary tree.
 *
 * Example:
 *
 * Input:
 *
 *
 *      4
 *    /   \
 *   2     7
 *  / \   / \
 * 1   3 6   9
 *
 * Output:
 *
 *
 *      4
 *    /   \
 *   7     2
 *  / \   / \
 * 9   6 3   1
 *
 * 
 * This problem was inspired by:
 *
 * Google: 90% of our engineers use the software you wrote (Homebrew), but you can invert a binary tree on a whiteboard so f*** off.
 *
 */
// problem: https://leetcode.cn/problems/invert-binary-tree/
use crate::{tree, TreeNode};

pub struct Solution;


use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        if let Some(n) = root.clone() {
            let left = n.borrow_mut().left.take();
            let right = n.borrow_mut().right.take();
            n.borrow_mut().left = right.clone();
            n.borrow_mut().right = left.clone();
            Self::invert_tree(left.clone());
            Self::invert_tree(right.clone());
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_226() {
        assert_eq!(
            Solution::invert_tree(tree![4, 2, 7, 1, 3, 6, 9]),
            tree![4, 7, 2, 9, 6, 3, 1]
        );
    }
}