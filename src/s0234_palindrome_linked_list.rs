/**
 * [234] Palindrome Linked List
 *
 * Given the head of a singly linked list, return true if it is a palindrome or false otherwise.
 *
 * Example1:
 *
 * Input: head = [1,2,2,1]
 * Output: true
 *
 * 
 * Example2:
 *
 * 
 * Input: head = [1,2]
 * Output: false
 * 
 * 
 * Follow up: Could you do it in O(n) time and O(1) space?
 */

use crate::to_list;

type ListNode = crate::ListNode<i32>;

pub struct Solution;

impl Solution {
    // 单链表无法倒着遍历 (不能使用双指针)
    // 最简单的办法就是反转链表，然后比较两个链表是否相同.
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut head = head;
        let mut values = vec![];
        loop {
            match head {
                Some(n) => {
                    head = n.next;
                    values.push(n.val);
                },
                None => { break; }    
            }
        }
        let len = values.len();
        let (mut lo, mut hi) = (0usize, len - 1);
        while lo < hi {
            if values[lo] != values[hi] { return false; }
            lo += 1;
            hi -= 1;
        }

        true
    }
}

#[test]
fn test_234() {
    assert_eq!(
        Solution::is_palindrome(to_list(vec![1, 1, 2, 1])),
        false
    );
}