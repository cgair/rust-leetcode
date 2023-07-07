// 问题描述:
// 一个链表, 奇数下标递增, 偶数下标递减, 排序使其总体递增.
// 输入: 1->8->3->6->5->4->7->2->NULL
// 输出: 1->2->3->4->5->6->7->8->NULL
#include "listnode.h"
#include <vector>

// 按奇偶位置拆分链表: 得 1->3->5->7->NULL 和 8->6->4->2->NULL
// 反转偶链表: 得 1->3->5->7->NULL和2->4->6->8->NULL
// 合并两个有序链表: 得 1->2->3->4->5->6->7->8->NULL

ListNode* reverse_list(ListNode* head) {
    if (head == nullptr) return nullptr;
    ListNode *pre = nullptr, *cur = head;
    while (cur != nullptr) {
        ListNode* tmp = cur->next;
        cur->next = pre;
        pre = cur;
        cur = tmp;
    }

    return pre;
}

ListNode* mege_two_sorted_list(ListNode* l1, ListNode* l2) {
    if (l1 == nullptr || l2 == nullptr) return l1 == nullptr ? l2 : l1;
    ListNode* dummy_head = new ListNode(-1);
    ListNode* p = dummy_head;
    while (l1 != nullptr && l2 != nullptr) {
        if (l1->val >= l2->val) {
            p->next = new ListNode(l2->val);
            l2 = l2->next;
        } else {
            p->next = new ListNode(l1->val);
            l1 = l1->next;
        }
        p = p->next;
    }
    p->next = l1 != nullptr ? l1 : l2;
    return dummy_head->next;
}

ListNode* sort_odd_even_list(ListNode* head) {
    if (head == nullptr) return nullptr;
    ListNode *p = head, *dummy_odd = new ListNode(-1), *dummy_even = new ListNode(-1);
    ListNode *odd = dummy_odd, *even = dummy_even;
    int pos = 1;
    while (p != nullptr) {
        if (pos % 2 == 1) {
            odd->next = new ListNode(p->val);
            odd = odd->next;
        } else {
            even->next = new ListNode(p->val);
            even = even->next;
        }
        pos ++;
        p = p->next;
    }
    // print_list(dummy_even);
    // print_list(dummy_odd);

    ListNode* reversed_even = reverse_list(dummy_even->next);
    print_list(reversed_even);

    // return mege_two_sorted_list(dummy_odd->next, reversed_even);
}

int main()
{
    std::vector<int> v = {1, 8, 3, 6, 5, 4, 7, 2};
    ListNode* head = construct_linkedlist(v);
    print_list(head);
    print_list(sort_odd_even_list(head));

    return 0;
}