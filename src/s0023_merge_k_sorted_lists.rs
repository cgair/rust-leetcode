/**
 * [23] Merge k Sorted Lists
 *
 * Merge k sorted linked lists and return it as one sorted list. Analyze and describe its complexity.
 *
 * Example:
 *
 *
 * Input:
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * Output: 1->1->2->3->4->4->5->6
 *
 *
 */

use std::collections::BinaryHeap;

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut curr = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = curr;
        curr = Some(Box::new(node));
    }
    curr
}

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut heap = BinaryHeap::new();
        let _ret: Vec<_> = lists.iter().map(|head| {
            let mut head = head.clone();
            while let Some(ref n) = head {
                // use std::cmp::Reverse;
                // heap.push(Reverse(n.val));
                heap.push(n.val);
                head = head.unwrap().next;
            }
        }).collect();
        let mut curr = None;
        while let Some(v) = heap.pop() {
            let mut node = ListNode::new(v);
            node.next = curr;
            curr = Some(Box::new(node));
        }
        curr
    }
}


#[test]
fn test_binary_heap() {
    let mut binary_heap =  BinaryHeap::from([0, 1, 2, 3, 4, 5, 6]);
    println!("{:?}", binary_heap);
    binary_heap.pop();
    println!("{:?}", binary_heap);
    binary_heap.push(10);
    println!("{:?}", binary_heap);
}


#[test]
fn test_23() {
    // Solution::merge_k_lists(vec![
    //     to_list(vec![1, 4, 5]),
    //     to_list(vec![1, 3, 4]),
    //     to_list(vec![2, 6]),
    // ]);
    assert_eq!(
        Solution::merge_k_lists(vec![
            to_list(vec![1, 4, 5]),
            to_list(vec![1, 3, 4]),
            to_list(vec![2, 6]),
        ]),
        to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
    );
    assert_eq!(Solution::merge_k_lists(vec![]), None);
}