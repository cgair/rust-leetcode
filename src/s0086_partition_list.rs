/**
 * [86] Partition List
 *
 * Given a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.
 *
 * You should preserve the original relative order of the nodes in each of the two partitions.
 *
 * Example:
 *
 *
 * Input: head = 1->4->3->2->5->2, x = 3
 * Output: 1->2->2->4->3->5
 *
 *
 */
use crate::{to_list, ListNode};

// problem: https://leetcode.cn/problems/partition-list/
pub struct Solution;

// submission codes start here

/**
 * 在合并两个有序链表时让你合二为一, 而这里需要分解让你把原链表一分为二.
 * 具体来说, 我们可以把原链表分成两个小链表, 一个链表中的元素大小都小于 x, 另一个链表中的元素都大于等于 x;
 * 最后再把这两条链表接到一起, 就得到了题目想要的结果.
 */
impl Solution {
    pub fn partition(head: Option<Box<ListNode<i32>>>, x: i32) -> Option<Box<ListNode<i32>>> {
        let mut dummy_lt = Some(Box::new(ListNode::new(-1)));
        let mut dummy_ge = Some(Box::new(ListNode::new(-1)));

        let mut p1 = &mut dummy_lt;
        let mut p2 = &mut dummy_ge;
        let mut head = head;

        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                p1.as_mut().unwrap().next = Some(node);
                p1 = &mut p1.as_mut().unwrap().next;
            } else {
                p2.as_mut().unwrap().next = Some(node);
                p2 = &mut p2.as_mut().unwrap().next;
            }
        }

        p1.as_mut().unwrap().next = dummy_ge.unwrap().next.take();
        
        // loop {
        //     match head {
        //         Some(mut node) => {
        //             head = node.next.take();
        //         },
        //         _ => {}
        //     }
        // }
        dummy_lt.unwrap().next
    }
}

#[test]
fn test_86() {
    assert_eq!(
        Solution::partition(to_list(vec![1, 4, 3, 2, 5, 2]), 3),
        to_list(vec![1, 2, 2, 4, 3, 5])
    );
    assert_eq!(
        Solution::partition(to_list(vec![1, 4, 3, 2, 5, 2]), 8),
        to_list(vec![1, 4, 3, 2, 5, 2])
    );
    assert_eq!(Solution::partition(to_list(vec![]), 0), to_list(vec![]));
}