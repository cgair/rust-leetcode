/**
 * [102] Binary Tree Level Order Traversal
 *
 * Given a binary tree, return the level order traversal of its nodes' values. (ie, from left to right, level by level).
 *
 *
 * For example:
 * Given binary tree [3,9,20,null,null,15,7],
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 *
 *
 * return its level order traversal as:
 *
 * [
 *   [3],
 *   [9,20],
 *   [15,7]
 * ]
 *
 *
 */

pub struct Solution;

// problem: https://leetcode.cn/problems/binary-tree-level-order-traversal/

// submission codes start here
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::tree;

type TreeNode = crate::TreeNode<i32>;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() { return vec![]; }
        let mut ret = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(root);
        while !queue.is_empty() {
            let mut level = vec![];
            let size = queue.len();
            for _ in 0..size {
                if let Some(maybe_n) = queue.pop_front() {
                    match maybe_n {
                        Some(n) => {
                            level.push(n.borrow().val);
                            queue.push_back(n.borrow().left.clone());
                            queue.push_back(n.borrow().right.clone());
                        },
                        None => ()
                    }
                }
            }
            if !level.is_empty() { ret.push(level); }
        }

        ret
    }
}

#[test]
fn test_102() {
    assert_eq!(
        Solution::level_order(tree![3, 9, 20, null, null, 15, 7]),
        vec![vec![3], vec![9, 20], vec![15, 7]]
    );
}