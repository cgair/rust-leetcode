/**
 * [剑指 Offer 07] 重建二叉树
 *
 * 输入某二叉树的前序遍历和中序遍历的结果, 请构建该二叉树并返回其根节点.
 * 假设输入的前序遍历和中序遍历的结果中都不含重复的数字.
 * 
 * 
 * 示例1:
 *
 *
 * 输入: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
 * 输出: [3,9,20,null,null,15,7]
 * 
 * 
 * 示例2:
 * 输入: preorder = [-1], inorder = [-1]
 * 输出: [-1]
 * 
 *
 */
// problem: https://leetcode.cn/problems/zhong-jian-er-cha-shu-lcof/
use super::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use crate::to_tree;

pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {

        Solution::helper(&preorder, &inorder, 0, preorder.len() as i32 - 1, 0, inorder.len() as i32 - 1)
    }

    fn helper(preorder: &[i32], inorder: &[i32], pstart: i32, pend: i32, istart: i32, iend: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || pstart > pend { return None; }

        let root_val = preorder[pstart as usize];
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(root_val))));

        let idx = inorder.into_iter().enumerate().find(|&t| *t.1 == root_val ).unwrap().0 as i32;


        root.as_mut().unwrap().borrow_mut().left = Solution::helper(preorder, inorder, pstart + 1, pstart + idx - istart, istart, idx - 1);
        root.as_mut().unwrap().borrow_mut().right = Solution::helper(preorder, inorder, pstart + idx - istart + 1, pend, idx + 1, iend);

        root
    }
}


#[test]
fn test_sword_7() {
    assert_eq!(
        Solution::build_tree(vec![3,9,20,15,7], vec![9,3,15,20,7]),
        to_tree(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])
    )
}