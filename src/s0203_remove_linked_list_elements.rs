/**
 * [203] Remove Linked List Elements
 *
 * Remove all elements from a linked list of integers that have value val.
 *
 * Example:
 *
 *
 * Input:  1->2->6->3->4->5->6, val = 6
 * Output: 1->2->3->4->5
 *
 *
 */
// problem: https://leetcode.cn/problems/remove-linked-list-elements/
use crate::{to_list, ListNode};
pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode<i32>>>, val: i32) -> Option<Box<ListNode<i32>>> {
        if !head.is_some() {
            return None;
        }
        let mut dummy_head = Some(Box::new(ListNode {val: 0, next: head} ));
        let mut curr = dummy_head.clone().unwrap().next;
        let mut pre = dummy_head.as_mut();
        while let Some(ref boxed_node) = curr {
            if boxed_node.val == val {
                let next = pre.as_mut().unwrap().next.as_mut().unwrap().next.take();
                pre.as_mut().unwrap().next = next;
                curr = curr.unwrap().next;
            } else {
                curr = curr.unwrap().next;
                pre = pre.unwrap().next.as_mut();
            }
        }

        dummy_head.unwrap().next
    }
}

#[test]
fn test_203() {
    assert_eq!(
        Solution::remove_elements(to_list(vec![1, 2, 6, 3, 4, 5, 6]), 6),
        to_list(vec![1, 2, 3, 4, 5])
    );
}