/**
 * [剑指 Offer 55 - I] 二叉树的深度
 *
 * 输入一棵二叉树的根节点, 求该树的深度.
 * 从根节点到叶节点依次经过的节点（含根、叶节点）形成树的一条路径, 最长路径的长度为树的深度.
 * 
 * 
 * 示例:
 *
 *
 * 给定二叉树 [3,9,20,null,null,15,7],
 *    3
 *   / \
 *  9  20
 *    /  \
 *   15   7
 * 返回它的最大深度 3
 * 
 * 
 */
// problem: https://leetcode.cn/problems/er-cha-shu-de-shen-du-lcof/
use std::rc::Rc;
use std::cell::RefCell;
use crate::tree;
use super::TreeNode;

pub struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut depth, mut ret) = (0, 0);
        Solution::traverse(&root, &mut depth, &mut ret);
        ret
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, depth: &mut i32, max: &mut i32) {
        if root.is_none() { return; }
        *depth += 1;
        if root.as_ref().unwrap().borrow().left.is_none() && root.as_ref().unwrap().borrow().right.is_none() {
            *max = std::cmp::max(*depth, *max);
        }
        Solution::traverse(&root.as_ref().unwrap().borrow().left, depth, max);
        Solution::traverse(&root.as_ref().unwrap().borrow().right, depth, max);
        *depth -= 1;
    }
}


#[test]
fn test_sword_55() {
    assert_eq!(Solution::max_depth(tree![]), 0);
    assert_eq!(Solution::max_depth(tree![3, 9, 20, null, null, 15, 7]), 3);
}