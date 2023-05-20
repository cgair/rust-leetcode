// problem: https://leetcode.cn/problems/reverse-nodes-in-k-group/
#include "../data_structure.h"

class Solution {
public:
    ListNode* reverseKGroup(ListNode* head, int k) {
        int len = 0;
        ListNode * curr = head;
        while (curr != nullptr) {
            len ++;
            curr = curr->next;
        }
        int n = len / k;
        ListNode* p = head;
        for (int i = 0; i < n; ++i) {
            p = reverseBetween(p, i * k + 1, (i + 1) * k);
        }

        return p;
    }

    ListNode* reverseBetween(ListNode* head, int left, int right) {
        ListNode* dummy_head = new ListNode(-1);
        dummy_head->next = head;
        ListNode* pre = dummy_head;
        ListNode* to_reverse = dummy_head;
        for(int i = 1; i < left; ++i) pre = pre->next;
        for(int i = 0; i < left; ++i) to_reverse = to_reverse->next;

        ListNode* end = dummy_head;
        for (int i = 0; i < right; ++i) end = end->next;

        ListNode* rest = end->next;
        end->next = nullptr;
        ListNode* reversed = reverse(to_reverse);
        pre->next = reversed;
        ListNode* p = reversed;
        while(p->next != nullptr) p = p->next;
        p->next = rest;
        
        return dummy_head->next;
    }

    ListNode* reverse(ListNode* head) {
        ListNode* pre = nullptr;
        ListNode* curr = head;

        while(curr != nullptr) {
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
    ListNode* revK =  s.reverseKGroup(head, 2);
    printLinkedList(revK);

    return 0;
}