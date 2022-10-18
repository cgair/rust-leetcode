// problem: https://leetcode.cn/problems/middle-of-the-linked-list/
use crate::{ListNode, to_list};

pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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