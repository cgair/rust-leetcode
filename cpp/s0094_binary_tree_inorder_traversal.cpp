#include "data_structure.h"
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> inorderTraversal(TreeNode* root) {
        vector<int> ret;
        Solution::helper(root, ret);

        return ret;
    }

    void helper(TreeNode* root, vector<int>& ret) {
        if (root == nullptr) { return; }
        Solution::inorderTraversal(root->left);
        ret.push_back(root->val);
        Solution::inorderTraversal(root->right);
    }
};