/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order and each of their nodes
 * contain a single digit. Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 *
 * Example:
 *
 *
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 *
 */
use crate::data_structure::ListNode;
pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut curr_dumb = &mut dummy_head;
        let mut carry = 0;
        let (mut l1, mut l2) = (l1, l2);
        let (mut l1_end, mut l2_end) = (false, false);

        loop {
            let lhs =  match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                },
                None => {
                    l1_end = true;
                    0
                }
            };

            let rhs = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                },
                None => {
                    l2_end = true;
                    0
                }
            };
            
            // if l1, l2 end and there is not overflow from previous operation, return the result
            if l1_end && l2_end && carry != 1 {
                return dummy_head.unwrap().next;
            }
            
            let sum = (lhs + rhs + carry) % 10;
            carry = (lhs + rhs + carry) / 10;
            
            // cannot move out of `*curr_dumb` which is behind a mutable reference
            // as_mut()
            curr_dumb.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            curr_dumb = &mut curr_dumb.as_mut().unwrap().next;
        }
    }

    // Construct a linked list
    pub fn construct_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut curr = None;    // construct from
        for &v in vec.iter().rev() {
            let mut new_node = Some(Box::new(ListNode::new(v)));
            new_node.as_mut().unwrap().next = curr;
            curr = new_node;
        }

        curr
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(Solution::construct_list(vec![2, 4, 3]), Solution::construct_list(vec![5, 6, 4])),
            Solution::construct_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(Solution::construct_list(vec![9, 9, 9, 9]), Solution::construct_list(vec![9, 9, 9, 9, 9, 9])),
            Solution::construct_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(Solution::construct_list(vec![0]), Solution::construct_list(vec![0])),
            Solution::construct_list(vec![0])
        )
    }
}
