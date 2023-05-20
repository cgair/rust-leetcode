// problem: https://leetcode.cn/problems/reverse-linked-list/
#include "../data_structure.h"

class Solution {
public:
    ListNode* reverseList(ListNode* head) {
        if (head == nullptr) return nullptr;
        ListNode *pre = nullptr;
        ListNode *curr = head;
        while (curr->next != nullptr) {
            ListNode *tmp = curr->next;
            curr->next = pre;
            pre = curr;
            curr = tmp;
        }
        curr->next = pre;
        return curr;

        // while (curr != nullptr) {
        //     ListNode *tmp = curr->next;
        //     curr->next = pre;
        //     pre = curr;
        //     curr = tmp;
        // }
        // return pre;
    }
};