/**
 * [142] Linked List Cycle II
 *
 * Given the head of a linked list, return the node where the cycle begins. If there is no cycle, return null.
 *
 * **Do not modify** the linked list.
 *
 * Example:
 *
 *
 * Input: head = [3,2,0,-4], pos = 1
 * Output: tail connects to node index 1
 * Explanation: There is a cycle in the linked list, where tail connects to the second node.
 *
 */

// problem: https://leetcode.cn/problems/linked-list-cycle/
#include "data_structure.h"
#include <stdio.h>

// 任意时刻, fast 指针走过的距离都为 slow 指针的 2 倍.
struct ListNode *detectCycle(struct ListNode *head) {
    struct ListNode *slow = head, *fast = head;
    while (fast != NULL) {
        if (fast->next == NULL) {
            return NULL;
        }
        slow = slow->next;
        fast = fast->next->next;
        if (fast == slow) {
            struct ListNode *ptr = head;
            while (ptr != slow) {
                ptr = ptr->next;
                slow = slow->next;
            }
            return ptr;   
        }
    }
    
    return NULL;
}

int main() {

}
