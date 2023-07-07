// 问题描述:
// k 个有序链表合并, 问时间复杂度
// problem: https://leetcode.cn/problems/merge-k-sorted-lists/
// 
// 解法1: 两两合并
//       1 和 2 合并, 结果再和 3 合并, 以此类推, 直到结束.
//       时间复杂度为: 2n + 3n + ... + kn = [(k+1)*k/2-1]*n = O(nk^2), 空间复杂度为 O(1)
// 解法2: 分治法 Divide and Conquer
//        每次将所有的 list 两两之间合并, 直到所有 list 合并成一个.
//        时间复杂度：2n * k/2 + 4n * k/4 + ... + (2^x)n * k/(2^x) = O(nklogk)
//        如果用迭代空间复杂度为O(1), 用递归则空间复杂度为 O(logk)
// 解法3: 最小堆 MinHeap/priority queue由于heap的大小为始终为k, 而每次插入的复杂度是logk, 一共插入了 nk 个节点.
//       时间复杂度为 O(nklogk), 空间复杂度为 O(k)
#include "listnode.h"
#include <vector>
#include <queue>

ListNode* merge_k_lists(std::vector<ListNode*>& lists) {
    // 方法3.1 最大堆 (default) + 倒序构建链表
    std::priority_queue<int> pq;

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

    // 方法3.2 最小堆
    auto cmp = [](ListNode* lhs, ListNode*rhs) {
        return lhs->val > rhs->val;
    };
    
    std::priority_queue<ListNode*, std::vector<ListNode*>, decltype(cmp)> minpq(cmp);
    for (auto n : lists) {
        if (n) minpq.push(n);
    }

    ListNode* dummy_head = new ListNode(-1);
    ListNode* p = dummy_head;
    for (; !minpq.empty(); minpq.pop()) {
        ListNode* n = minpq.top();
        p->next = n;
        p = p->next;
        if (n->next != nullptr) minpq.push(n->next);
    }

    return dummy_head->next;
}