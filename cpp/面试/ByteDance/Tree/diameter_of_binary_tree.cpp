// 问题描述:
// 给定一颗二叉数, 每两个结点路径为1, 求相隔最远的两个结点的距离.
// problem: https://leetcode.cn/problems/diameter-of-binary-tree/
#include "treenode.h"
#include <climits>
#include <algorithm>

int depth_of_binary_tree(TreeNode* root, int& max_diameter) {
    if (root == nullptr) return 0; 

    int L = depth_of_binary_tree(root->left, max_diameter);
    int R = depth_of_binary_tree(root->right, max_diameter);

    max_diameter = std::max(max_diameter, L + R + 1);

    return std::max(L, R) + 1;
}


int diameter_of_binary_tree(TreeNode* root) {
    int ret = INT_MIN;
    depth_of_binary_tree(root, ret);
    return ret - 1;
}
