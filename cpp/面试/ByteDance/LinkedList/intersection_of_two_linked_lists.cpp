// 问题描述:
// 输入两个链表, 找出它们的第一个公共节点.
// 可假定整个链表结构中没有循环
// 1. 在返回结果后, 两个链表仍须保持原有的结构.
// 2. 程序尽量满足 O(n) 时间复杂度, 且仅用 O(1) 内存.
// problem: https://leetcode.cn/problems/liang-ge-lian-biao-de-di-yi-ge-gong-gong-jie-dian-lcof/
#include "listnode.h"
#include <vector>

ListNode* get_intersection_node(ListNode *headA, ListNode *headB) {
    if (headA == nullptr || headB == nullptr) {
        return nullptr;
    }

    ListNode* pa = headA, *pb = headB;
    while (pa != pb) {
        pa = pa == nullptr ? headB : pa->next;
        pb = pb == nullptr ? headA : pb->next;
    }

    return pa;
}

int main() {

    return 0;
}