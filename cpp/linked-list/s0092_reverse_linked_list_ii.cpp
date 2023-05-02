// problem: https://leetcode.cn/problems/reverse-linked-list-ii/
#include "../data_structure.h"
#include <vector>

class Solution {
public:
    ListNode* reverseBetween(ListNode* head, int left, int right) {
        ListNode* dummy_head = new ListNode(-1);
        dummy_head->next = head;
        ListNode* pre = dummy_head;
        ListNode* p1 = dummy_head;
        ListNode* p2 = dummy_head;
        for (int i = 0; i < left - 1; ++i) {
            pre = pre->next;
        }
        for (int i = 0; i < left; ++i) {
            p1 = p1->next;
        }
        for (int i = 0; i < right; ++i) {
            p2 = p2->next;
        }
        ListNode* rest = p2->next;
        p2->next = nullptr;

        ListNode* rev = reverse(p1);
        ListNode* p3 = rev;
        while (p3->next != nullptr) p3 = p3->next;
        p3->next = rest;
        pre->next = rev;

        return dummy_head->next;
    }

    ListNode* reverse(ListNode* head) {
        ListNode* pre = nullptr;
        ListNode* curr = head;

        while (curr != nullptr) {
            ListNode* next = curr->next;
            curr->next = pre;
            pre = curr;
            curr = next;
        }


        return pre;
    }
};

int main()
{
    ListNode* head = nullptr;
    std::vector<int> vec = {1,2,3,4,5};
    constructLinkedList(vec, head);
    printLinkedList(head);
    Solution s;
    ListNode* rev =  s.reverseBetween(head, 1, 4);
    printLinkedList(rev);

    return 0;
}