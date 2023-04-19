// problem: https://leetcode.cn/problems/combination-sum/
#include <vector>
#include <numeric>

using namespace std;

class Solution {
public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<int> path;
        vector<vector<int>> ret;
        Solution::backtrack(candidates, target, 0, path, ret);
        
        return ret;
    }

    void backtrack(vector<int>& choices, int target, int start, vector<int>& path, vector<vector<int>>& ret) {
        int sum = accumulate(path.begin(), path.end(), 0);
        if(sum == target) { ret.push_back(vector<int>(path.begin(), path.end())); }
        if (sum > target) { return; }

        for(int i = start; i < choices.size(); ++i) {
            path.push_back(choices[i]);
            Solution::backtrack(choices, target, i, path, ret);
            path.pop_back();
        }    
    }
};

int main() {}