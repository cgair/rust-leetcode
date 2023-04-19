// problem: https://leetcode.cn/problems/permutations/
#include <vector>
using namespace std;

class Solution {
public:
    vector<vector<int>> permute(vector<int>& nums) {
        vector<vector<int>> ret;
        if(nums.empty()) { return ret; }

        int len = nums.size();
        vector<bool> used;
        used.assign(len, false);
        vector<int> path;
        Solution::backtrack(nums, path, used, ret);

        return ret;
    }

    void backtrack(vector<int>& choices, vector<int>& path, vector<bool>& used, vector<vector<int>>& ret) {
        if(path.size() == choices.size()) {
            ret.push_back(vector<int>(path.begin(), path.end()));
            return;
        }
        for(int i = 0; i < choices.size(); ++i) {
            if(used[i]) { continue; }
            path.push_back(choices[i]);
            used[i] = true;
            Solution::backtrack(choices, path, used, ret);
            path.pop_back();
            used[i] = false;
        } 
    }
};

int main()
{}