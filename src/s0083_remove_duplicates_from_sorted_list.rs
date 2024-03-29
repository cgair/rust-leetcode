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
    fn delete_duplicates(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
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

    // Magic of dummy_head
    fn _delete_duplicates(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
        if head.is_none() { return None; }
    
        let mut head = head;
        let mut dummy_head = Some(Box::new(ListNode::new(-1)));
        let mut new_head = head.as_mut().unwrap().next.take();
        dummy_head.as_mut().unwrap().next = head;
        
        let mut curr = &mut dummy_head.as_mut().unwrap().next;
        
        while let Some(mut node) = new_head {
            new_head = node.next.take();
            if node.val != curr.as_ref().unwrap().val {
                curr.as_mut().unwrap().next = Some(node);
                curr = &mut curr.as_mut().unwrap().next;
            }
        }
    
        dummy_head.unwrap().next
    }
}

#[test]
fn test83() {
    assert_eq!(
        Solution::delete_duplicates(to_list(vec![1, 1, 2])),
        to_list(vec![1, 2])
    );
}