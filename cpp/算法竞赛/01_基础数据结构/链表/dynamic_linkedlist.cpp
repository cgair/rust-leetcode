#include <iostream>
#include <vector>

/**
 * [动态链表]
 * 优点: 及时释放空间, 不使用多余内存
 * Definition for singly-linked list.
 */
struct ListNode
{
    int val;
    ListNode* next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int _val) : val(_val), next(nullptr) {}
    ListNode(int _val, ListNode* _next) : val(_val), next(_next) {}
};

void insert(ListNode*& head, int val) {
    ListNode* nn = new ListNode(val);
    nn->next = head;
    head = nn;
}

ListNode* constructLinkedList(std::vector<int>& vals) {
    ListNode* head = nullptr;
    for (int i = vals.size() - 1; i >= 0; i--) {
        insert(head, vals[i]);
    }

    return head;
}

void printLinkedList(ListNode* head) {
    ListNode* p = head;
    while (p != nullptr) {
        std::cout << p->val << " -> ";
        p = p->next;
    }
    std::cout << "NULL" << std::endl;
}


int main()
{
    std::vector<int> vals = {1, 2, 3, 4, 5, 6};
    ListNode* head = constructLinkedList(vals);
    printLinkedList(head);
    delete head;

    return 0;
}
