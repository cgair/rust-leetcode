// problem: https://leetcode.cn/problems/binary-tree-level-order-traversal/
#include "../data_structure.h"
#include <vector>
#include <queue>
using namespace std;

class Solution {
public:
    vector<vector<int>> levelOrder(TreeNode* root) {
        vector<vector<int>> ret;
        if(root == nullptr) { return ret; }
        queue<TreeNode*> q;
        q.push(root);

        while(!q.empty()) {
            int sz = q.size();
            vector<int> level;
            for(int i = 0; i < sz; ++i) {
                TreeNode* n = q.front();
                q.pop();
                level.push_back(n->val);
                if(n->left) { q.push(n->left); }
                if(n->right) { q.push(n->right); }
            }
            if(!level.empty()) { ret.push_back(level); }
        }
        
        return ret;
    }
};

int main() { return 0; }