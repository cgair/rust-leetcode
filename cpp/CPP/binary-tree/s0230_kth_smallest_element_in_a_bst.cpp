// 给定一个二叉搜索树的根节点 root, 和一个整数 k, 请你设计一个算法查找其中第 k 个最小元素 (从 1 开始计数).
// problem: https://leetcode.cn/problems/kth-smallest-element-in-a-bst/
#include "../data_structure.h"
#include <iostream>
#include <vector>

class Solution {
public:
    // 记录结果
    int res = 0;
    // 记录当前元素的排名
    int rank = 0;
    int kthSmallest(TreeNode* root, int k) {
        // BST 有一个重要的性质: BST 的中序遍历结果是有序的 (升序)
        // 1. bad
        std::vector<int> ret;
        inorderTraverse(root, ret);
    
        // 2. fine
        traverse(root, k);
        return res;
        
        return ret[k - 1];
    }

    void inorderTraverse(TreeNode* root, std::vector<int>& v) {
        if (root == nullptr) return;
        
        inorderTraverse(root->left, v);
        v.push_back(root->val);
        inorderTraverse(root->right, v);
    }

    void traverse(TreeNode* root, int k) {
        if (!root) {
            return;
        }
        traverse(root->left, k);
        /* 中序遍历代码位置 */
        rank++;
        if (k == rank) {
            // 找到第 k 小的元素
            res = root->val;
            return;
        }
        /*****************/
        traverse(root->right, k);
    }
};

int main() {
    std::vector<int> a1 = {1, 2, 3};
    std::vector<int> b1(3);
    std::copy(a1.begin(), a1.end(), b1.begin());
    for (auto i : b1) {
        std::cout << i << " ";
    }
    std::cout << std::endl;

    std::vector<int> a2 = {4, 5, 6, 7, 8};
    std::vector<int> b2;
    b2.assign(a2.begin(), a2.end());
    for (auto i : b2) {
        std::cout << i << " ";
    }
    std::cout << std::endl;

    return 0;
}