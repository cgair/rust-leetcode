/**
 * [92] Reverse Linked List II
 *
 * Given the head of a singly linked list and two integers left and right where left <= right, 
 * reverse the nodes of the list from position left to position right, and return the reversed list.
 *
 *
 * Example:
 *
 *
 * Input: 1->2->3->4->5->NULL, m = 2, n = 4
 * Output: 1->4->3->2->5->NULL
 *
 *
 * Follow up: Could you do it in one pass?
 * 
 */
use crate::to_list;

type ListNode = crate::ListNode<i32>;
// problem: https://leetcode.cn/problems/reverse-linked-list-ii/

pub struct Solution;

// submission codes start here
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        if m == n { return head; }
        
        let mut dummy_head = Some(Box::new(ListNode::new(-1)));
        dummy_head.as_mut().unwrap().next = head;

        let mut p = &mut dummy_head;
        for _ in 1..m {
            p = &mut p.as_mut().unwrap().next;
        }

        // find the linkedlist(need reverse) start
        let mut found = p.as_mut().unwrap().next.take();
        println!("{found:?}");

        // do reversion
        let mut mid = None;
        let mut step = n - m;
        while step >= 0 {
            let mut n = found.unwrap();     // unwrap is safe here
            found = n.next.take();
            n.next = mid;
            mid = Some(n);
            step -= 1;
        }
        println!("{mid:?}");

        // do concatenation
        let mut p = &mut dummy_head;
        while let Some(_) = p.as_ref().unwrap().next {
            p = &mut p.as_mut().unwrap().next;
        }
        p.as_mut().unwrap().next = mid;

        while let Some(_) = p.as_ref().unwrap().next {
            p = &mut p.as_mut().unwrap().next;
        }
        p.as_mut().unwrap().next = found;

        dummy_head.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_92() {
        for _ in 1..0 {
            println!("Donot happen.")
        }
        assert_eq!(
            Solution::reverse_between(
                to_list(vec![1, 2, 3, 4, 5]), 
                2, 
                4
            ),
            to_list(vec![1, 4, 3, 2, 5])
        );
        assert_eq!(
            Solution::reverse_between(
                to_list(vec![3, 5]), 
                1, 
                2
            ),
            to_list(vec![5, 3])
        );
        assert_eq!(
            Solution::reverse_between(
                to_list(vec![5]), 
                1, 
                1
            ),
            to_list(vec![5])
        );
    }
}