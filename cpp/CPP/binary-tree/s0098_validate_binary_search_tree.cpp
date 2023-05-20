// problem: https://leetcode.cn/problems/validate-binary-search-tree/
#include "../data_structure.h"

class Solution {
public:
    // the following code is wrong!
    // consider:
    //      10
    //     /  \
    //    5   15
    //       /  \
    //      6   20
    bool isValidBSTWrong(TreeNode* root) {
        if (root == nullptr) {
            return true;
        }
        if (root->left != nullptr && root->left->val >= root->val) {
            return false;
        }
        if (root->right != nullptr && root->right->val <= root->val) {
            return false;
        }

        return isValidBSTWrong(root->left) && isValidBSTWrong(root->right);
    }
};