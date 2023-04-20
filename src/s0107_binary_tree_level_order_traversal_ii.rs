/**
 * [107] Binary Tree Level Order Traversal II
 *
 * Given a binary tree, return the bottom-up level order traversal of its nodes' values. (ie, from left to right, level by level from leaf to root).
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
 * return its bottom-up level order traversal as:
 *
 * [
 *   [15,7],
 *   [9,20],
 *   [3]
 * ]
 *
 *
 */
pub struct Solution;
use crate::tree;

type TreeNode =  crate::TreeNode<i32>;

// problem: https://leetcode.cn/problems/binary-tree-level-order-traversal-ii/

// submission codes start here

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() { return vec![]; }
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut ret = Vec::new();

        while !queue.is_empty() {
            let sz = queue.len();
            let mut level = vec![];
            for _ in 0..sz {
                if let Some(n) = queue.pop_front().unwrap() {
                    level.push(n.borrow().val);
                    queue.push_back(n.borrow().left.clone());
                    queue.push_back(n.borrow().right.clone());
                }
            }
            if !level.is_empty() { ret.push(level); }
        }

        ret.reverse();
        ret       
    }
}


#[test]
fn test_107() {
    assert_eq!(
        Solution::level_order_bottom(tree![3, 9, 20, null, null, 15, 7]),
        vec![vec![15, 7], vec![9, 20], vec![3],]
    );
}