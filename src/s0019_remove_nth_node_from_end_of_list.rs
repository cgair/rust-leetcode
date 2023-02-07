/**
 * [19] Remove Nth Node From End of List
 *
 * Given a linked list, remove the n-th node from the end of list and return its head.
 *
 * Example:
 *
 *
 * Given linked list: 1->2->3->4->5, and n = 2.
 *
 * After removing the second node from the end, the linked list becomes 1->2->3->5.
 *
 *
 * Note:
 *
 * Given n will always be valid.
 *
 * Follow up:
 *
 * Could you do this in one pass?
 *
 */

// problem: https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
use crate::{ListNode, to_list};

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode<i32>>>, n: i32) -> Option<Box<ListNode<i32>>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head } ));
        let mut len = 0;
        {
            let mut p = dummy_head.as_ref().unwrap().next.as_ref();
            while let Some(n) = p {
                len += 1;
                p = n.next.as_ref();
            }
        }

        let idx = len - n;
        // let mut p = dummy_head.as_mut();

        // for _ in 0..idx {
        //     p = p.unwrap().next.as_mut();
        // }
        // let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
        // p.unwrap().next = next;
        let mut p = dummy_head.as_mut().unwrap();
        for _ in 0..idx {
            p = p.next.as_mut().unwrap();
        }
        let next = p.next.as_mut().unwrap().next.take();
        p.next = next;
        
        dummy_head.unwrap().next
    }

    pub fn remove_nth_from_end2(mut head: Option<Box<ListNode<i32>>>, n: i32) -> Option<Box<ListNode<i32>>> {
        let mut length = 0i32;
        {
            let mut curr = head.as_ref();
            while !curr.is_none() {
                length += 1;
                curr = curr.unwrap().next.as_ref();
            }
        }
        let mut pos = length - n;
        if pos == 0 {
            return head.unwrap().next;
        }
        let mut p = head.as_mut();
        for _ in 0..pos - 1 {
            p = p.unwrap().next.as_mut();
            println!("{:?}", p);
        }
        let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
        p.unwrap().next = next;

        head
    }
}

#[test]
fn test19() {
    let to_be_removed = Some(Box::new(
        ListNode {
            val: 1,
            next: Some(Box::new(
                ListNode {
                    val: 2,
                    next: Some(Box::new(
                        ListNode {
                            val: 3, 
                            next: Some(Box::new(
                                ListNode {
                                    val: 4,
                                    next: Some(Box::new(
                                        ListNode {
                                            val: 5,
                                            next: Some(Box::new(
                                                ListNode::new(6)
                                            ))
                                        }
                                    ))
                                }
                            ))
                        }
                    ))
                }
            ))
        }
    ));

    let expected = Some(Box::new(
        ListNode {
            val: 1,
            next: Some(Box::new(
                ListNode {
                    val: 2,
                    next: Some(Box::new(
                        ListNode {
                            val: 3, 
                            next: Some(Box::new(
                                ListNode {
                                    val: 4,
                                    next: Some(Box::new(
                                        ListNode {
                                            val: 6,
                                            next: None
                                        }
                                    ))
                                }
                            ))
                        }
                    ))
                }
            ))
        }
    ));
    let ret = Solution::remove_nth_from_end(to_be_removed, 2);
    assert_eq!(expected, ret);

    assert_eq!(
        Solution::remove_nth_from_end(to_list(vec![1,2,3,4,5]), 2),
        to_list(vec![1,2,3,5])
    );
    
    assert_eq!(
        Solution::remove_nth_from_end2(to_list(vec![1,2,3,4,5]), 2),
        to_list(vec![1,2,3,5])
    );
}