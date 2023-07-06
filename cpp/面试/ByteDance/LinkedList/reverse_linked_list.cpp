// 问题描述:
// 递归和迭代的反转链表
// problem: https://leetcode.cn/problems/reverse-linked-list/
#include "listnode.h"

ListNode* reverse_list(ListNode* head) {
    ListNode* pre = nullptr;
    ListNode* cur = head;
    
    while (cur != nullptr) {
        ListNode* nxt = cur->next;
        cur->next = pre;
        pre = cur;
        cur = nxt;
    }

    return pre;
}


ListNode* reverse_list_recur(ListNode* head) {
    if (head == nullptr || head->next == nullptr) return head;

    ListNode* new_head = reverse_list_recur(head->next);
    head->next->next = head;
    head->next = nullptr;

    return new_head;
}

int main()
{

    return 0;
}