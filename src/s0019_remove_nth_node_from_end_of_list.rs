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

/**
 * 如何寻找从后往前数的第 k 个节点呢 ?
 * 假设链表有 n 个节点，倒数第 k 个节点就是正数第 n - k + 1 个节点, 也是一个 for 循环的事儿.
 * 是的, 但是算法题一般只给你一个 ListNode 头结点代表一条单链表, 你需要先遍历一遍链表算出 n 的值, 然后再遍历链表计算第 n - k + 1 个节点.
 * 也就是说, 这个解法需要遍历两次链表才能得到出倒数第 k 个节点.
 * 
 * 我们能不能只遍历一次链表, 就算出倒数第 k 个节点?
 * 可以做到的.
 * 
 * 首先, 我们先让一个指针 p1 指向链表的头节点 head, 然后走 k 步: <https://labuladong.github.io/algo/images/%e9%93%be%e8%a1%a8%e6%8a%80%e5%b7%a7/1.jpeg>
 * 现在的 p1, 只要再走 n - k 步, 就能走到链表末尾的空指针了.
 * 趁这个时候, 再用一个指针 p2 指向链表头节点 head.
 * 接下来就很显然了, 让 p1 和 p2 同时向前走, p1 走到链表末尾的空指针时前进了 n - k 步, p2 也从 head 开始前进了 n - k 步, 
 * 停留在第 n - k + 1 个节点上, 即恰好停链表的倒数第 k 个节点上: <https://labuladong.github.io/algo/images/%e9%93%be%e8%a1%a8%e6%8a%80%e5%b7%a7/3.jpeg>
 */

// one pass (two pointer runner pattern) cannot make borrow checker happy
pub fn remove_nth_from_end(mut head: Option<Box<ListNode<i32>>>, n: i32) -> Option<Box<ListNode<i32>>> {
    let mut p1 = &mut head;
    for _ in 0..n {
        p1 = &mut p1.as_mut().unwrap().next;
    }

    let mut step = 0;
    while !p1.as_ref().unwrap().next.is_none() {
        step += 1;
        p1 = &mut p1.as_mut().unwrap().next;
    }
    let next = p1.take();

    let mut p2 = &mut head;
    for _ in 0..step {
        p2 = &mut p2.as_mut().unwrap().next;
    }
    p2.as_mut().unwrap().next = next;

    head
}

// ok I'm mad.
pub unsafe fn unsafe_remove_nth_from_end(mut head: Option<Box<ListNode<i32>>>, n: i32) -> Option<Box<ListNode<i32>>> {
    let mut p1 = head.as_mut().unwrap() as *mut Box<ListNode<i32>>;
    let mut p2 = head.as_mut().unwrap() as *mut Box<ListNode<i32>>;
    // move p1 n forward
    for _ in 0..n {
        p1 = (*p1).next.as_mut().unwrap();
    }

    while !(*p1).next.is_none() {
        p1 = (*p1).next.as_mut().unwrap();
        p2 = (*p2).next.as_mut().unwrap();
    }
    (*p2).next = (*p2).next.as_mut().unwrap().next.take();

    head
}


#[test]
fn test_19() {
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

    assert_eq!(
        remove_nth_from_end(to_list(vec![1,2,3,4,5]), 2),
        to_list(vec![1,2,3,5])
    );
    unsafe {
        assert_eq!(
            unsafe_remove_nth_from_end(to_list(vec![1,2,3,4,5]), 2),
            to_list(vec![1,2,3,5])
        )
    }
}