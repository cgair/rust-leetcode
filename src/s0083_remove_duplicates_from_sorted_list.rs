/**
 * [83] Remove Duplicates from Sorted List
 *
 * Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.
 *
 * Example:
 *
 *
 * Given linked list: 1->1->2
 *
 * After removing the second node, the linked list becomes 1->2
 * 
 */

// problem: https://leetcode.cn/problems/remove-duplicates-from-sorted-list/
use crate::{ListNode, to_list};

struct Solution;

impl Solution {
    // 夺走, 校验不过再还回去
    fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut head = head;
        let mut pre = head.as_mut().unwrap();
        let mut value = pre.val;
        while let Some(n) = pre.next.take() {
            if value == n.val {
                pre.next = n.next;
            } else {
                value = n.val;
                pre.next = Some(n);
                pre = pre.next.as_mut().unwrap();
            }
        }

        head
    }
}

#[test]
fn test83() {
    assert_eq!(
        Solution::delete_duplicates(to_list(vec![1, 1, 2])),
        to_list(vec![1, 2])
    );
}