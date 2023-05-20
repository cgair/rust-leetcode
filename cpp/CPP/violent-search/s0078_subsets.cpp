// problem: https://leetcode.cn/problems/subsets/
#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
    vector<vector<int>> subsets(vector<int>& nums) {
        vector<vector<int>> ret;
        if(nums.empty()) { return ret; }
        vector<int> path;
        Solution::backtrack(nums, path, 0, ret);

        return ret;
    }

    void backtrack(vector<int>& choices, vector<int>& path, int start, vector<vector<int>>& result) {
        result.push_back(vector<int>(path.begin(), path.end()));

        for(int i = start; i < choices.size(); ++i) {
            path.push_back(choices[i]);
            Solution::backtrack(choices, path, i + 1, result);
            path.pop_back();
        }
    }
};

int main() {
    Solution s;
    vector<int> nums = {1, 2, 3};
    auto ret = s.subsets(nums);
    for(auto v : ret) {
        cout << "[";
        for (auto x : v) {
            cout << x << " ";
        }
        cout << "]";
        cout << endl;
    }
}