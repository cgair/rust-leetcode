/**
 * [21] Merge Two Sorted Lists
 *
 * Merge two sorted linked lists and return it as a new list. The new list should be made by splicing together the nodes of the first two lists.
 *
 * Example:
 *
 * Input: 1->2->4, 1->3->4
 * Output: 1->1->2->3->4->4
 *
 *
 */

use crate::{ListNode, to_list}; 
pub struct Solution;

impl Solution {
    fn merge_two_lists(l1: Option<Box<ListNode<i32>>>, l2: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut previous = &mut dummy_head;
        let (mut l1, mut l2) = (l1, l2);
        while l1.is_some() || l2.is_some() {
            if l1.is_none() {
                previous.as_mut().unwrap().next = l2;
                break;
            } else if l2.is_none() {
                previous.as_mut().unwrap().next = l1;
                break;
            }

            // use extra memeroy
            // let next = if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
            //     let n = ListNode::new(l1.as_ref().unwrap().val);
            //     l1 = l1.unwrap().next;
            //     n
            // } else {
            //     let n = ListNode::new(l2.as_ref().unwrap().val);
            //     l2 = l2.unwrap().next;
            //     n
            // };

            // in place manipulation
            let next  = if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                let origin = l1.as_mut().unwrap().next.take();
                let next = l1.take();
                l1 = origin;
                next
            } else {
                let origin = l2.as_mut().unwrap().next.take();
                let next = l2.take();
                l2 = origin;
                next
            };
            // previous.as_mut().unwrap().next = Some(Box::new(next));
            previous.as_mut().unwrap().next = next;
            previous = &mut previous.as_mut().unwrap().next;
        }

        dummy_head.unwrap().next
    }
}


/// Merge two ordered list.
///
/// Both `list1` and `list2` are sorted in ascending order.
pub fn merge_two_lists<T: Ord>(mut list1: Option<Box<ListNode<T>>>, mut list2: Option<Box<ListNode<T>>>) -> Option<Box<ListNode<T>>> {
    let mut head = None;

    let mut curr = &mut head;
    loop {
        match (list1, list2) {
            (Some(mut l1), Some(mut l2)) => {
                if l1.val <= l2.val {
                    list1 = l1.next.take();
                    list2 = Some(l2);
                    curr = &mut curr.insert(l1).next;
                } else {
                    list1 = Some(l1);
                    list2 = l2.next.take();
                    curr = &mut curr.insert(l2).next;
                }
            },
            // (_, None) => {},
            // (None, _) => {}
            (l1, l2) => {
                *curr = l1.or(l2);
                break;
            }
        }
    }

    head
}



#[test]
fn test_21() {
    let l1 = Some(Box::new(
        ListNode {
            val: 1,
            next: Some(Box::new(
                ListNode {
                    val: 2,
                    next: Some(Box::new(
                        ListNode {
                            val: 4, 
                            next: None
                        }
                    ))
                }
            ))
        }
    ));
    let l2 = Some(Box::new(
        ListNode {
            val: 1,
            next: Some(Box::new(
                ListNode {
                    val: 3,
                    next: Some(Box::new(
                        ListNode {
                            val: 4, 
                            next: None
                        }
                    ))
                }
            ))
        }
    ));

    let ret = Solution::merge_two_lists(l1, l2);
    let expected = Some(Box::new(
        ListNode {
            val: 1,
            next: Some(Box::new(
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
                                                ListNode::new(4)
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
    assert_eq!(ret, expected);
    println!("result: {:?}", ret);

    assert_eq!(
        merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
        to_list(vec![1, 1, 2, 3, 4, 4])
    );
}