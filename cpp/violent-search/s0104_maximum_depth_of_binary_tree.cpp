// problem: https://leetcode.cn/problems/maximum-depth-of-binary-tree/
#include "../data_structure.h"
#include <algorithm>

class Solution {
public:
    int maxDepth(TreeNode* root) {
        if(root == nullptr) { return 0; }
        int max_depth = INT_MIN;
        int depth = 0;
        Solution::helper(root, depth, max_depth);
        return max_depth;
    }

    void helper(TreeNode* root, int &depth, int &max_depth) {
        if(root == nullptr) { return; }

        depth ++;
        max_depth = std::max(depth, max_depth);
        Solution::helper(root->left, depth, max_depth);
        Solution::helper(root->right, depth, max_depth);
        depth --;
    }

};


int main() { }