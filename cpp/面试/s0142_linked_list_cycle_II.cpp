// 142. 环形链表 II
// https://leetcode.cn/problems/linked-list-cycle-ii/
#include "data_structure.h"

class Solution {
public:
    ListNode *detectCycle(ListNode *head) {
        ListNode *slow = head, *fast = head, *p = head;
        while (fast != nullptr) {
            if (fast->next == nullptr) return nullptr;
            slow = slow->next;
            fast = fast->next->next;

            if (slow == fast) {
                while (p != fast) {
                    p = p->next;
                    fast = fast->next;
                }

                return p;
            }
        }

        return nullptr;
    }
};