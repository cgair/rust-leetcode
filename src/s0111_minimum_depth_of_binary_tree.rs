/**
 * [111] Minimum Depth of Binary Tree
 *
 * Given a binary tree, find its minimum depth.
 *
 * The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 * Given binary tree [3,9,20,null,null,15,7],
 *
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 * return its minimum depth = 2.
 *
 */
// problem: https://leetcode.com/problems/minimum-depth-of-binary-tree/
use crate::{tree, TreeNode};
use std::rc::Rc;
use std::cell::RefCell;

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if !root.is_some() {
            return 0;
        }
        let mut queue = VecDeque::new();
        
        queue.push_back(root);
        let mut min_depth = 1;
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let curr = queue.pop_front().unwrap();
                if curr.as_ref().unwrap().borrow().left.is_none() && curr.as_ref().unwrap().borrow().right.is_none() {
                    return min_depth;
                } 
                if !curr.as_ref().unwrap().borrow().left.is_none() {
                    queue.push_back(curr.as_ref().unwrap().borrow().left.clone());
                }
                if !curr.as_ref().unwrap().borrow().right.is_none() {
                    queue.push_back(curr.as_ref().unwrap().borrow().right.clone());
                }
            }

            // if let Some(node) = queue.pop_front().unwrap() {
            //     if node.borrow().left.is_none() && node.borrow().right.is_none() {
            //         return min_depth;
            //     }
            //     queue.push_back(node.borrow().left.clone());
            //     queue.push_back(node.borrow().right.clone());
            // }
            min_depth += 1;
        }

        0
    }
}


#[test]
fn test_111() {
    assert_eq!(Solution::min_depth(tree![3, 9, 20, null, null, 15, 7]), 2);
}