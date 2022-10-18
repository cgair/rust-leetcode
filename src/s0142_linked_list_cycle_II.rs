/**
 * [142]  Linked List Cycle II
 *
 * Given the head of a linked list, return the node where the cycle begins. If there is no cycle, return null.
 *
 * **Do not modify** the linked list.
 *
 * Example:
 *
 *
 * Input: head = [3,2,0,-4], pos = 1
 * Output: tail connects to node index 1
 * Explanation: There is a cycle in the linked list, where tail connects to the second node.
 *
 */

// problem: https://leetcode.cn/problems/linked-list-cycle/
use crate::{ListNode, to_list};

struct Solution;

impl Solution {
    pub fn detect_cycle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut fast = head.as_ref().unwrap().next.as_ref();
        let mut slow = head.as_ref();

        while !slow.unwrap().next.as_ref().is_none(){
            while !fast.is_none() {
                if fast.unwrap().next.as_ref() == slow {
                    // [How to convert Option<&T> to Option<T> in the most idiomatic way in Rust?](https://stackoverflow.com/questions/51338579/how-to-convert-optiont-to-optiont-in-the-most-idiomatic-way-in-rust)
                    return slow.cloned();
                }
                fast = fast.unwrap().next.as_ref();
            }
            slow = slow.unwrap().next.as_ref();
        }

        None
    }
}

#[test]
fn test142() {
    let head = to_list(vec![3, 2, 0, -4]);
    assert_eq!(
        None,
        Solution::detect_cycle(head)
    );
}