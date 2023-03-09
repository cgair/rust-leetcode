/**
 * [25] Reverse Nodes in k-Group
 *
 * Given a linked list, reverse the nodes of a linked list k at a time and return its modified list.
 *
 * k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes in the end should remain as it is.
 *
 *
 *
 *
 * Example:
 *
 * Given this linked list: 1->2->3->4->5
 *
 * For k = 2, you should return: 2->1->4->3->5
 *
 * For k = 3, you should return: 3->2->1->4->5
 *
 * Note:
 *
 *
 * 	Only constant extra memory is allowed.
 * 	You may not alter the values in the list's nodes, only nodes itself may be changed.
 *
 *
 */

use crate::to_list;
type ListNode = crate::ListNode<i32>;

// problem: https://leetcode.cn/problems/reverse-nodes-in-k-group/
pub struct Solution;

// submission codes start here

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(-1)));
        let mut dh = &mut dummy_head;

        let mut head = head;

        let mut p = &head;
        let mut len = 0;
        while !p.is_none() { 
            len += 1; 
            p = &p.as_ref().unwrap().next;
        }
        let mut round = len / k;
        for _ in 0..round {
            let mut pre = None;
            let mut step = k;
            while let Some(mut n) = head {
                head = n.next.take();
                n.next = pre;
                pre = Some(n);
                step -= 1;
                if step == 0 { break; }
            }
            while !dh.as_ref().unwrap().next.is_none() { dh = &mut dh.as_mut().unwrap().next; }
            dh.as_mut().unwrap().next = pre;
        }
        while !dh.as_ref().unwrap().next.is_none() { dh = &mut dh.as_mut().unwrap().next; }
        dh.as_mut().unwrap().next = head;


        dummy_head.unwrap().next
    }
}


#[test]
fn test_25() {
    assert_eq!(
        Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 2),
        to_list(vec![2, 1, 4, 3, 5])
    );
    assert_eq!(
        Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 3),
        to_list(vec![3, 2, 1, 4, 5])
    );
    assert_eq!(
        Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 5),
        to_list(vec![5, 4, 3, 2, 1])
    );
    assert_eq!(
        Solution::reverse_k_group(to_list(vec![1]), 1),
        to_list(vec![1])
    );
}