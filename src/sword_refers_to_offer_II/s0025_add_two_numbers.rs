/**
 * [剑指 Offer II 025] 链表中的两数相加
 *
 * 给定两个非空链表 l1 和 l2 来代表两个非负整数. 数字最高位位于链表开始位置.
 * 它们的每个节点只存储一位数字, 将这两数相加会返回一个新的链表.
 * 可以假设除了数字 0 之外, 这两个数字都不会以零开头.
 * 
 * 
 * 示例1:
 *
 *
 * 输入: l1 = [7,2,4,3], l2 = [5,6,4]
 * 输出: [7,8,0,7]
 * 
 * 
 * 示例2:
 * 输入: l1 = [2,4,3], l2 = [5,6,4]
 * 输出: [8,0,7]
 *
 */
// problem: https://leetcode.cn/problems/lMSNwu/
use crate::{ListNode, to_list};
pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode<i32>>>, l2: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
        let (mut stack1, mut stack2) = (vec![], vec![]);
        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());
        let mut dummy_heaed = Some(Box::new(ListNode::new(0)));
        let mut curr = dummy_heaed.as_mut();
        let mut flag = 0;
        while !l1.is_none() {
            stack1.push(l1.unwrap().val);
            l1 = l1.unwrap().next.as_ref();
        }
        while !l2.is_none() {
            stack2.push(l2.unwrap().val);
            l2 = l2.unwrap().next.as_ref();
        }
        // println!("stack1 = {:?}", stack1);
        // println!("stack2 = {:?}", stack2);
        let mut ret = vec![];
        while !stack1.is_empty() || !stack2.is_empty() {
            let v1 = stack1.pop().unwrap_or(0);
            let v2 = stack2.pop().unwrap_or(0);
            let s = v1 + v2 + flag;
            flag = s / 10;
            let value = s % 10;
            println!("flag = {}, value = {}", flag, value);
            ret.push(value);
        }
        if flag > 0 {
            ret.push(flag);
        }
        ret.reverse();
        for v in ret {
            curr.as_mut().unwrap().next = Some(Box::new(ListNode::new(v)));
            curr = curr.unwrap().next.as_mut();
        }

        dummy_heaed.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sro25() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![7, 2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 8, 0, 7])
        );
    }
}