// problem: https://leetcode.cn/problems/binary-tree-zigzag-level-order-traversal/
#include "../data_structure.h"
#include <vector>
#include <queue>
#include <algorithm>
using namespace std;

class Solution {
public:
    vector<vector<int>> zigzagLevelOrder(TreeNode* root) {
        vector<vector<int>> ret;
        if (root == nullptr) return ret;
        bool rev = false;

        queue<TreeNode*> q;
        q.push(root);
        
        while (!q.empty())
        {
            int sz = q.size();
            vector<TreeNode*> level;
            for (int i = 0; i < sz; ++i) {
                TreeNode* tn = q.front();
                q.pop();
                level.push_back(tn);
                if (tn->left) q.push(tn->left);
                if (tn->right) q.push(tn->right);
            }
            vector<int> value;
            if (rev) {
                std::reverse(level.begin(), level.end());
            }
            for (auto n: level) {
                value.push_back(n->val);
            }
            rev = !rev;
            ret.push_back(value);
        }
        
        return ret;
    }
};

