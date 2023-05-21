// 239. 滑动窗口最大值
// problem: https://leetcode.cn/problems/sliding-window-maximum/
#include <iostream>
#include <vector>
#include <queue>
using namespace std;

class Solution {
public:
    // 方法一: 优先队列
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        int n = nums.size();
        priority_queue<pair<int, int > > pq;
        for (int i = 0; i < k; ++i) {
            pq.emplace(nums[i], i);
        }
        vector<int> ret = { pq.top().first };

        for (int i = k; i < n; ++i) {
            pq.emplace(nums[i], i);
            while (pq.top().second <= i - k) pq.pop();
            ret.push_back(pq.top().first);
        }

        return ret;
    }
};

// 方法二: 单调队列 (用 STL dequeue 实现单调队列)
vector<int> maxSlidingWindow(vector<int>& nums, int k) {
    vector<int> ret;
    // TBD

    return ret;
}

int main()
{

    return 0;
}