#ifndef __LISTNODE_H__
#define __LISTNODE_H__
#include <vector>

struct ListNode
{
    int val;
    ListNode* next;
    ListNode() : val(0), next(nullptr) { }
    ListNode(int x) : val(x), next(nullptr) { }
};

void insert(ListNode*& head, int value) {
    ListNode* n = new ListNode(value);
    n->next = head;
    head = n;
}

ListNode* construct_linkedlist(const std::vector<int>& value) {
    ListNode* head = nullptr;
    for (int i = value.size() - 1; i >= 0; i--) {
        insert(head, value[i]);
    }

    return head;
}


#endif