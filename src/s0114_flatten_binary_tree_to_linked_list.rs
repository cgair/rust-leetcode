/**
 * [114] Flatten Binary Tree to Linked List
 *
 * Given a binary tree, flatten it to a linked list in-place.
 *
 * For example, given the following tree:
 *
 *
 *     1
 *    / \
 *   2   5
 *  / \   \
 * 3   4   6
 *
 *
 * The flattened tree should look like:
 *
 *
 * 1
 *  \
 *   2
 *    \
 *     3
 *      \
 *       4
 *        \
 *         5
 *          \
 *           6
 *
 *
 */
use crate::tree;
type TreeNode = crate::TreeNode<i32>;

pub struct Solution;

// problem: https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) { 
        if root.is_none() { return; }

        Solution::flatten(&mut root.as_mut().unwrap().borrow_mut().left);
        Solution::flatten(&mut root.as_mut().unwrap().borrow_mut().right);

        /*后序遍历位置 */
        // 此时左右子树已经被拉平成一条链表
        let next = root.as_mut().unwrap().borrow_mut().right.take();
        let left = root.as_mut().unwrap().borrow_mut().left.take();
        root.as_mut().unwrap().borrow_mut().right = left;
        let mut p = root.clone();
        while !p.as_ref().unwrap().borrow().right.is_none() {
            p = p.unwrap().borrow().right.clone();
        }
        p.as_mut().unwrap().borrow_mut().right = next;
    }
}


#[test]
fn test_114() {
    let mut tree = tree![1, 2, 5, 3, 4, null, 6];
    Solution::flatten(&mut tree);
    assert_eq!(tree, tree![1, null, 2, null, 3, null, 4, null, 5, null, 6]);

    let mut tree = tree![1, 2, null, 3];
    Solution::flatten(&mut tree);
    assert_eq!(tree, tree![1, null, 2, null, 3]);

    let mut tree = tree![1, null, 2, 3];
    Solution::flatten(&mut tree);
    assert_eq!(tree, tree![1, null, 2, null, 3]);
}