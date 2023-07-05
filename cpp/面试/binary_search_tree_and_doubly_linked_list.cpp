// 剑指 Offer 36. 二叉搜索树与双向链表
// problem: https://leetcode.cn/problems/er-cha-sou-suo-shu-yu-shuang-xiang-lian-biao-lcof/
#include "data_structure.h"
#include <vector>

typedef TreeNode Node;

// 要求不能创建任何新的节点, 只能调整树中节点指针的指向.
class Solution {
public:
    Node* treeToDoublyList(Node* root) {
        if (root == nullptr) return nullptr;
        std::vector<Node*> nodes;
        inorderTraverse(root, nodes);

        int n = nodes.size();
        nodes[0]->left = nodes[n-1];
        nodes[n-1]->right = nodes[0];

        for (int i = 0; i < n - 1; ++i) {
            nodes[i]->right = nodes[i + 1];
            nodes[i + 1]->left = nodes[i];
        }

        return nodes[0];
    }

    void inorderTraverse(Node* root, std::vector<Node*>& vec) {
        if (root == nullptr) return;

        inorderTraverse(root->left, vec);
        vec.push_back(root);
        inorderTraverse(root->right, vec);
    }

    // dfs
    Node* treeToDoublyList2(Node* root) {
        if (root == nullptr) return nullptr;
        dfs(root);
        head->left = pre;
        pre->right = head;

        return head;
    }

    void dfs(Node* curr) {
        if (curr == nullptr) return;
        
        dfs(curr->left);
        if (pre == nullptr) head = curr;
        else pre->right = curr;
        curr->left = pre;
        pre = curr;
        dfs(curr->right);
    }

private: 
    Node* pre, *head;
};