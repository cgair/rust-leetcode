// 107. 二叉树的层序遍历 II
// problem: https://leetcode.cn/problems/binary-tree-level-order-traversal-ii/
#include "../../data_structure.h"
#include <vector>
#include <queue>
using namespace std;

class Solution {
public:
    vector<vector<int>> levelOrderBottom(TreeNode* root) {
        vector<vector<int> > ret;
        if (root == nullptr) return ret;
        queue<TreeNode*> q;
        q.push(root);

        while (!q.empty())
        {
            int sz = q.size();
            vector<int> level;
            for (int i = 0; i < sz; ++i) {
                auto n = q.front();
                q.pop();
                level.push_back(n->val);
                if (n->left) q.push(n->left);
                if (n->right) q.push(n->right);
            }
            ret.push_back(level);
        }
        reverse(ret.begin(), ret.end());

        return ret;
    }
};