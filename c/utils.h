#include "data_structure.h"
#include <stdio.h>


void insert(ListNode **head, int item) {
    ListNode *n;
    ListNode *ptr;
    *n = {item, NULL};

    if(*head = NULL) {
        *head = n;
    } else {
        ptr = *head;
        while (ptr->next != NULL) ptr = ptr->next;
        ptr->next = n;
    }

}

void display(ListNode *head) {
    while(head != NULL) {
        printf("%d -> ", head->val);
        head = head->next;
    }
    printf("NULL\n");
}

ListNode *to_list(int arr[], int n) {
    ListNode *head = NULL;
    for (int i = 0; i < n; i++) {
        insert(&head, arr[i]);
    }
    
    return head;
}