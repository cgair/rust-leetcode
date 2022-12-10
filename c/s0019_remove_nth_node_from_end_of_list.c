/**
 * [19] Remove Nth Node From End of List
 *
 * Given a linked list, remove the n-th node from the end of list and return its head.
 *
 * Example:
 *
 *
 * Given linked list: 1->2->3->4->5, and n = 2.
 *
 * After removing the second node from the end, the linked list becomes 1->2->3->5.
 *
 *
 * Note:
 *
 * Given n will always be valid.
 *
 * Follow up:
 *
 * Could you do this in one pass?
 *
 */

// problem: https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
#include "data_structure.h"
#include <stdio.h>

struct ListNode* remove_nth_from_end(struct ListNode *head, int n)
{
    int length;
    int pos;
    struct ListNode *curr = head;
    while (curr != NULL) {
        length += 1;
        curr = curr->next;
    }
    pos = length - n;
    if (pos == 0) {
        return head->next;
    } else {
        struct ListNode *pre = head;
        for (int i = 0; i < pos - 1; i++) {
            pre = pre->next;
        }
        pre->next = pre->next->next;
        return head;
    }
}

int main()
{

}