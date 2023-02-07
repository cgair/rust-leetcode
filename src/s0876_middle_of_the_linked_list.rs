/**
 * [1] Two Sum
 *
 * Given the head of a singly linked list, return the middle node of the linked list.
 * If there are two middle nodes, return the second middle node.
 * 
 * Example:
 *
 *
 * Input: head = [1,2,3,4,5]
 * Output: [3,4,5]
 * Explanation: The middle node of the list is node 3.
 * 
 * Input: head = [1,2,3,4,5,6]
 * Output: [4,5,6]
 * Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.
 */
// problem: https://leetcode.cn/problems/middle-of-the-linked-list/
use crate::{ListNode, to_list};

pub struct Solution;

impl Solution {
    pub fn middle_node<T: Clone>(head: Option<Box<ListNode<T>>>) -> Option<Box<ListNode<T>>> {
        let mut slow = &head;
        let mut fast = &head;
        while slow.is_some() && fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        slow.clone()
    }
}

#[test]
fn test_876() {
    assert_eq!(
        to_list(vec![3, 4, 5]),
        Solution::middle_node(to_list(vec![1, 2, 3, 4, 5]))
    );
    
    assert_eq!(
        to_list(vec![4, 5, 6]),
        Solution::middle_node(to_list(vec![1, 2, 3, 4, 5, 6]))
    )
}