/**
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
 *
 * Given preorder and inorder traversal of a tree, construct the binary tree.
 *
 * Note:
 * You may assume that duplicates do not exist in the tree.
 *
 * For example, given
 *
 *
 * preorder = [3,9,20,15,7]
 * inorder = [9,3,15,20,7]
 *
 * Return the following binary tree:
 *
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 */
use crate::{to_tree, TreeNode};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        if preorder.is_empty() { return None; }

        Solution::build_tree_recursive(&preorder, &inorder, 0, preorder.len() - 1, 0, inorder.len() - 1)
    }

    fn build_tree_recursive(preorder: &[i32], inorder: &[i32], pre_start: usize, pre_end: usize, in_start: usize, in_end: usize) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        if pre_start > pre_end || in_start > in_end { return None; }
        
        let root_val = preorder[pre_start];
        let root = Some(Rc::new(RefCell::new(TreeNode::new(root_val))));

        let index = inorder[..].iter().position(|&x| { x == root_val }).unwrap();
        // TODO(cgair): optimize, 因为题目说二叉树节点的值不存在重复, 所以可以使用一个 HashMap 存储元素到索引的映射, 这样就可以直接通过 HashMap 查到 rootVal 对应的 index.

        // println!("{index:}, {in_start:}");
        let left_size = index.checked_sub(in_start).or(Some(0)).unwrap();
        
        root.as_ref().unwrap().borrow_mut().left = Solution::build_tree_recursive(preorder, inorder, pre_start + 1, pre_start + left_size, in_start, index.checked_sub(1).or(Some(0)).unwrap());
        root.as_ref().unwrap().borrow_mut().right = Solution::build_tree_recursive(preorder, inorder, pre_start + left_size + 1, pre_end, index + 1, in_end);

        root
    }
}


#[test]
fn test_105() {
    assert_eq!(
        Solution::build_tree(vec![3,9,20,15,7], vec![9,3,15,20,7]),
        to_tree(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])
    )
}