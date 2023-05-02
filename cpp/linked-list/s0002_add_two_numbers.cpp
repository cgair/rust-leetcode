// problem: https://leetcode.cn/problems/add-two-numbers/
// 
// 示例 1:
// 输入: l1 = [2,4,3], l2 = [5,6,4]
// 输出: [7,0,8]
// 解释: 342 + 465 = 807.
// 
// 示例 2:
// 输入: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// 输出: [8,9,9,9,0,0,0,1]
#include "../data_structure.h"

class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        int carry = 0;
        ListNode* dummy_head = new ListNode(-1);
        ListNode* curr = dummy_head;
        while (l1 != nullptr && l2 != nullptr) {
            int sum = l1->val + l2->val + carry;
            carry = sum / 10;
            int remain = sum % 10;
            curr->next = new ListNode(remain);
            curr = curr->next;
            l1 = l1->next;
            l2 = l2->next;
        }
        while (l1 != nullptr) {
            int sum = l1->val + carry;
            carry = sum / 10;
            int remain = sum % 10;
            curr->next = new ListNode(remain);
            curr = curr->next;
            l1 = l1->next;
        }  
        while(l2 != nullptr) {
            int sum = l2->val + carry;
            carry = sum / 10;
            int remain = sum % 10;
            curr->next = new ListNode(remain);
            curr = curr->next;
            l2 = l2->next;
        }
        if (carry == 1) curr->next = new ListNode(1); 

        return dummy_head->next;
    }
};

int main()
{
    ListNode* l1 = nullptr;
    std::vector<int> vec1 = {9,9,9,9,9,9,9};
    constructLinkedList(vec1, l1);
    printLinkedList(l1);

    ListNode* l2 = nullptr;
    std::vector<int> vec2 = {9,9,9,9};
    constructLinkedList(vec2, l2);
    printLinkedList(l2);

    Solution s;
    ListNode* sum =  s.addTwoNumbers(l1, l2);
    printLinkedList(sum);
}
