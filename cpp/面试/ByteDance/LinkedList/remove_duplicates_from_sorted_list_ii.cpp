// 问题描述:
// 给定一个已排序的链表的头 head, 删除原始链表中所有重复数字的节点, 只留下不同的数字.
// problem: https://leetcode.cn/problems/remove-duplicates-from-sorted-list-ii/
#include "listnode.h"
#include <vector>

ListNode* delete_duplicates(ListNode* head) {
    if (head == nullptr || head->next == nullptr) return head;
    ListNode* dummy_head = new ListNode(-1);
    dummy_head->next = head;

    ListNode *pre = dummy_head, *cur = head;

    while (cur != nullptr) {
        // 跳过当前的重复节点, 使得cur指向当前重复元素的最后一个位置
        while (cur-> next != nullptr && cur->val == cur->next->val) {
            cur = cur->next;
        }
        if (pre->next == cur) {
            pre = pre->next;
        } else {
            pre->next = cur->next;
        }

        cur = cur->next;
    }

    return dummy_head->next;
}

int main()
{
    std::vector<int> t1 = {1, 2, 3, 3, 4, 4, 5};
    ListNode* head1 = construct_linkedlist(t1);

    print_list(delete_duplicates(head1));

    std::vector<int> t2 = {1, 1, 1, 2, 3};
    ListNode* head2 = construct_linkedlist(t2);

    print_list(delete_duplicates(head2));


    std::vector<int> t3 = {1, 1};
    ListNode* head3 = construct_linkedlist(t3);

    print_list(delete_duplicates(head3));


    return 0;
}