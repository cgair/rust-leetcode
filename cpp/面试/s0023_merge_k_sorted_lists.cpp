// 23. 合并 K 个升序链表
// problem: https://leetcode.cn/problems/merge-k-sorted-lists/
#include "data_structure.h"
#include <vector>
#include <queue>

using namespace std;

class Solution {
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        priority_queue<ListNode*, vector<ListNode*>> pq;

        // TODO
        for (auto &l : lists) {

        }
        
    }

    ListNode* mergeKLists2(vector<ListNode*>& lists) {
        priority_queue<int> pq;

        for (auto &l : lists) {
            ListNode* p = l;
            while (l != nullptr) {
                pq.emplace(l->val);
                l = l->next;
            }
        }

        ListNode* pre = nullptr, *cur;
        while (!pq.empty()) {
            int val = pq.top();
            pq.pop();
            cur = new ListNode(val);
            cur->next = pre;
            pre = cur;
        }

        return pre;
    }
};

