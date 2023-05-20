/**
 * [160] Intersection of Two Linked Lists
 *
 * Given the heads of two singly linked-lists headA and headB, return the node at which the two lists intersect. 
 * If the two linked lists have no intersection at all, return null.
 * 
 * For example, the following two linked lists begin to intersect at node c1: 
 * ![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/14/160_statement.png)
 * 
 * Note that the linked lists must retain their original structure after the function returns.
 *
 */

// problem: https://leetcode.cn/problems/intersection-of-two-linked-lists/
#include "../data_structure.h"
#include <unordered_set>

// submission codes start here
class Solution {
public:
    // 这道题的难点在于, 因为两个链表的长度可能不同, 两链表之间的节点无法对应
    // 两个指针分别在两个链表上前进并不能**同时**走到公共节点
    // 所以如何让两个指针 "齐头并进"?
    // 1. 可以先算出两个链表的长度
    // 2. 方法: logically 将两个链表接起来
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        ListNode* ha = headA;
        ListNode* hb = headB;
        while (ha != hb) {
            if (ha != nullptr) {
                ha = ha->next;
            } else {
                ha = headB;
            }
            if (hb != nullptr) {
                hb = hb->next;
            } else {
                hb = headA;
            }
        }

        return ha;
    }


    ListNode *getIntersectionNode2(ListNode *headA, ListNode *headB) {
        std::unordered_set<ListNode*> uset;
        ListNode* ha = headA;
        ListNode* hb = headB;

        while (ha != nullptr) {
            uset.insert(ha);
            ha = ha->next;
        }

        while (hb != nullptr) {
            if (uset.count(hb)) return hb;
            hb = hb->next;
        }
 
        return nullptr;
    }
};