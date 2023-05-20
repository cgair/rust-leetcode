#include "../data_structure.h"

class Solution {
public:
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        ListNode *dummy_head = new ListNode(-1);
        ListNode *p = dummy_head;

        while(list1 != nullptr && list2 != nullptr) {
            if(list1->val <= list2->val) {
                p->next = list1;
                list1 = list1->next;
            } else {
                p->next = list2;
                list2 = list2->next;
            }
            p = p->next;
        }

        p->next = list1 == nullptr ? list2 : list1;

        return dummy_head->next;
    }
};