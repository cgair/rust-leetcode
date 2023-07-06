// [剑指 Offer 07] 重建二叉树
// problem: https://leetcode.cn/problems/zhong-jian-er-cha-shu-lcof/
#include <vector>
#include <unordered_map>
#include "../../data_structure.h"

using namespace std;

class Solution {
    // find index in O(1)
    unordered_map<int, int> umap;
public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        for (int i = 0; i < inorder.size(); ++i) 
            umap[inorder[i]] = i;
        return _buildTree(preorder, inorder, 0, preorder.size() - 1, 0, inorder.size() - 1);
    }

    TreeNode* _buildTree(
        vector<int>& preorder,
        vector<int>& inorder,
        int pstart,
        int pend,
        int istart,
        int iend
        ) 
        {
            if (pstart > pend || istart > iend) return nullptr;
            int root_val = preorder[pstart];
            TreeNode* root = new TreeNode(root_val);
            // int found = find(inorder, root_val);
            int found = umap[root_val];
            root->left = _buildTree(preorder, inorder, pstart + 1, pstart + found - istart, istart, found - 1);
            root->right = _buildTree(preorder, inorder, pstart + found - istart + 1, pend, found + 1, iend);

            return root;
        }

    int find(const vector<int>& vec, int val) {
        int pos = 0;
        for (auto &v : vec) {
            if (v == val) return pos;
            pos ++;
        }
        return -1;
    }
    
};

int main()
{
    vector<int> preorder = {3,9,20,15,7};
    vector<int> inorder = {9,3,15,20,7};
    Solution s;
    s.buildTree(preorder, inorder);

    return 0;
}