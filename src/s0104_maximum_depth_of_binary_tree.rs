/**
 * [104] Maximum Depth of Binary Tree
 *
 * Given a binary tree, find its maximum depth.
 *
 * The maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
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
 * return its depth = 3.
 *
 */
use crate::{tree, TreeNode};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

pub struct Solution;


// problem: https://leetcode.cn/problems/maximum-depth-of-binary-tree/

impl Solution {
    pub fn max_depth<T>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
        if root.is_none() { return 0;}
        
        let mut max = 0;
        // depth 记录当前递归到的节点深度
        let mut depth = 0;

        Solution::transversal(root, &mut max, &mut depth);

        max
    }

    // 前序位置是进入一个节点的时候, 后序位置是离开一个节点的时候.
    fn transversal<T>(mut root: Option<Rc<RefCell<TreeNode<T>>>>, max: &mut i32, depth: &mut i32) {
        if root.is_none() { return; }

	    // 前序位置
        *depth += 1;
        if root.as_ref().unwrap().borrow().left.is_none() && root.as_ref().unwrap().borrow().right.is_none() {
            *max = cmp::max(*depth, *max);
        }
        Solution::transversal(root.as_mut().unwrap().borrow_mut().left.take(), max, depth);
        Solution::transversal(root.as_mut().unwrap().borrow_mut().right.take(), max, depth);
        // 后序位置
        *depth -= 1;

    }
}


#[test]
fn test_104() {
    assert_eq!(Solution::max_depth::<i32>(tree![]), 0);
    assert_eq!(Solution::max_depth::<i32>(tree![3, 9, 20, null, null, 15, 7]), 3);
}