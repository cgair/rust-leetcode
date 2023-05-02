// problem: https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
#include "../data_structure.h"

class Solution {
public:
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode* dummy_head = new ListNode(-1);
        dummy_head->next = head;
        ListNode* slow = dummy_head;
        ListNode* fast = dummy_head;

        for (int i = 0; i < n; ++i) {
            fast = fast->next;
        }

        while (fast->next != nullptr) {
            slow = slow->next;
            fast = fast->next;
        }
        ListNode* tmp = slow->next->next;
        slow->next = tmp;

        return dummy_head->next;
    }
};