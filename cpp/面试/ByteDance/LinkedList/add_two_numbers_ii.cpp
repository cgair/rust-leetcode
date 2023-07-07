// 问题描述:
// 给你两个非空链表来代表两个非负整数. 数字最高位位于链表开始位置, 它们的每个节点只存储一位数字, 将这两数相加会返回一个新的链表.
// 你可以假设除了数字 0 之外, 这两个数字都不会以零开头
// problem: https://leetcode.cn/problems/add-two-numbers-ii/
#include "listnode.h"
#include <iostream>
#include <vector>

// https://leetcode.cn/problems/add-two-numbers-ii/solution/liang-shu-xiang-jia-ii-by-leetcode-solution/
// TODO: 栈 方法 (逆序先想到栈)

ListNode* reverse_list(ListNode* head) {
    ListNode* pre = nullptr, *cur = head;
    while (cur != nullptr) {
        ListNode* tmp = cur->next;
        cur->next = pre;
        pre = cur;
        cur = tmp;
    }

    return pre;
}

ListNode* add_two_numbers(ListNode* l1, ListNode* l2) {
    l1 = reverse_list(l1);
    l2 = reverse_list(l2);
    int carry = 0;

    ListNode *op1 = l1, *op2 = l2;
    ListNode* dummy_head = new ListNode(-1);
    ListNode* p = dummy_head;
    while (op1 != nullptr && op2 != nullptr) {
        int sum = op1->val + op2->val + carry;
        carry = sum / 10;
        int value = sum % 10;
        p->next = new ListNode(value);
        
        p = p->next;
        op1 = op1->next;
        op2 = op2->next;
    }
    while (op1 != nullptr) {
        int sum = op1->val + carry;
        carry = sum / 10;
        int value = sum % 10;
        p->next = new ListNode(value);
        p = p->next;
        op1 = op1->next;
    } 
    while(op2 != nullptr) {
        int sum = op2->val + carry;
        carry = sum / 10;
        int value = sum % 10;
        p->next = new ListNode(value);
        p = p->next;
        op2 = op2->next;
    }
    if (carry) {
        p->next = new ListNode(1);
    }

    ListNode* ret = dummy_head->next;
    ret = reverse_list(ret);

    return ret;
}


int main()
{   
    int t;
    std::cin >> t;
    while (t--) {
        std::vector<int> value1, value2;
        int input;
        while(std::cin >> input) {
            value1.push_back(input);
            if (getchar() == '\n') break;
        }
        while(std::cin >> input) {
            value2.push_back(input);
            if (getchar() == '\n') break;
        }

        ListNode* l1 = construct_linkedlist(value1);
        ListNode* l2 = construct_linkedlist(value2);
        print_list(l1);
        print_list(l2);

        print_list(add_two_numbers(l1, l2));
    }


    return 0;
}