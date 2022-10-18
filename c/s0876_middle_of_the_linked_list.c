/**
 * [1] Two Sum
 *
 * Given the head of a singly linked list, return the middle node of the linked list.
 * If there are two middle nodes, return the second middle node.
 * 
 * Example:
 *
 *
 * Input: head = [1,2,3,4,5]
 * Output: [3,4,5]
 * Explanation: The middle node of the list is node 3.
 * 
 * Input: head = [1,2,3,4,5,6]
 * Output: [4,5,6]
 * Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.
 */
// problem: https://leetcode.cn/problems/middle-of-the-linked-list/
#include "data_structure.h"
#include <stdio.h>

struct ListNode* middle_node(struct ListNode* head){
   struct ListNode *slow = head, *fast = head;
   while(slow->next != NULL && fast != NULL && fast->next != NULL) {
    slow = slow->next;
    fast = fast->next->next;
   }
   return slow;
}

int main(int argc, char *argv[], char *envp[]) {
}