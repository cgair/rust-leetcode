/**
 * [剑指 Offer II 078] 合并排序链表
 *
 * 给定一个链表数组, 每个链表都已经按升序排列.
 * 请将所有链表合并到一个升序链表中, 返回合并后的链表.
 * 
 * 
 * 示例1:
 *
 *
 * 输入: lists = [[1,4,5],[1,3,4],[2,6]]
 * 输出: [1,1,2,3,4,4,5,6]
 * 解释: 链表数组如下:
 *      [
 *          1->4->5,
 *          1->3->4,
 *          2->6
 *       ]
 * 将它们合并到一个有序链表中得到.
 * 1->1->2->3->4->4->5->6
 * 
 * 
 * 示例2:
 * 输入: lists = []
 * 输出: []
 *
 * 
 * 示例3:
 * 输入: lists = [[]]
 * 输出: []
 * 
 */
// problem: https://leetcode.cn/problems/vvXgSW/
use std::collections::BinaryHeap;
type ListNode = crate::ListNode<i32>;

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() { return None; }
        let mut heap = BinaryHeap::new();   // max heap
        lists.iter().map(|head| {
           let mut curr = head;
           while let Some(n) = curr {
            heap.push(n.val);
            curr = &n.next;
           }
        }).collect::<Vec<_>>();
        let mut curr = None;
        while !heap.is_empty() {
            let mut node = Box::new(ListNode::new(heap.pop().unwrap()));
            node.next = curr;
            curr = Some(node);
        }

        curr
    }
}


use crate::to_list;
#[test]
fn test_sword_23() {
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