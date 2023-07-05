// 160. 相交链表
// <https://leetcode.cn/problems/intersection-of-two-linked-lists/>
// 
// 剑指 Offer 52. 两个链表的第一个公共节点
// <https://leetcode.cn/problems/liang-ge-lian-biao-de-di-yi-ge-gong-gong-jie-dian-lcof/>
#include "data_structure.h"~

class Solution {
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        ListNode* pA = headA;
        ListNode* pB = headB;

        // while (pA != nullptr || pB != nullptr) {
        while (pA != pB) {
            // if (pA == pB) return pA;

            if (pA != nullptr) {
                pA = pA->next;
            } else {
                pA = headB;
            }

            if (pB != nullptr) {
                pB = pB->next;
            } else {
                pB = headA;
            }
        }

        // return nullptr;
        return pA;
    }
};