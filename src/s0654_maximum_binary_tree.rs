/**
 * [654] Maximum Binary Tree
 *
 * You are given an integer array nums with no duplicates. 
 * 
 * A maximum binary tree can be built recursively from nums using the following algorithm:
 * 
 * Create a root node whose value is the maximum value in nums.
 *     Recursively build the left subtree on the subarray prefix to the left of the maximum value.
 *     Recursively build the right subtree on the subarray suffix to the right of the maximum value.
 *     Return the maximum binary tree built from nums.
 *
 * Example:
 *
 *
 * Input: nums = [3,2,1,6,0,5]
 * Output: [6,3,5,null,2,0,null,null,1]
 *
 */

// problem: https://leetcode.cn/problems/maximum-binary-tree/
use std::rc::Rc;
use std::cell::RefCell;
use crate::{TreeNode, to_tree};

pub struct Solution;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        if nums.is_empty() { return None; }
        
        Solution::construct_maximum_binary_tree_recursive(&nums, 0, (nums.len() - 1) as i32)
    }

    fn construct_maximum_binary_tree_recursive(nums: &[i32], lo: i32, hi: i32) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        // base case
        if lo > hi { return None; }
        // let max_idx = Solution::index_by_max(&nums[lo as usize..=hi as usize]);
        
        let (mut max_idx, mut max) = (-1, i32::MIN);
        for i in lo..=hi {
            if max < nums[i as usize] {
                max_idx = i;
                max = nums[i as usize];
            }
        }

        let root = Some(Rc::new(RefCell::new(TreeNode::new(nums[max_idx as usize]))));

        println!("{max_idx:}");
        root.as_ref().unwrap().borrow_mut().left = Solution::construct_maximum_binary_tree_recursive(nums, lo, max_idx - 1);
        root.as_ref().unwrap().borrow_mut().right = Solution::construct_maximum_binary_tree_recursive(nums, max_idx+ 1, hi);

        root
    }

    fn index_by_max(nums: &[i32]) -> usize {
        use std::cmp::Ordering;
        assert!(nums.len() > 0);
        nums
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| { a.cmp(b) } )
            .map(|(index, _)| index)
            .unwrap()
    }
}

#[test]
fn test_654() {
    assert_eq!(
        Solution::construct_maximum_binary_tree(vec![3,2,1,6,0,5]),
        to_tree(vec![Some(6), Some(3), Some(5), None, Some(2), Some(0), None, None, Some(1)])
    )
}