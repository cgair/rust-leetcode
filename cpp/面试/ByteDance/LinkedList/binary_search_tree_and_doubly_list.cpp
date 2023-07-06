// 问题描述:
// 二叉搜索树转排序的循环双向链表
// 要求不能创建任何新的节点, 只能调整树中节点指针的指向.
// problem: https://leetcode.cn/problems/er-cha-sou-suo-shu-yu-shuang-xiang-lian-biao-lcof/

class Node {
public:
    int val;
    Node* left;
    Node* right;

    Node() {}

    Node(int _val) {
        val = _val;
        left = nullptr;
        right = nullptr;
    }

    Node(int _val, Node* _left, Node* _right) {
        val = _val;
        left = _left;
        right = _right;
    }
};

Node* pre = nullptr, *head;

Node* tree_to_doubly_list(Node* root) {
    if (root == nullptr) return nullptr;
    dfs(root);
    head->left = pre;
    pre->right = head;

    return head;
}

void dfs(Node* root) {
    if (root == nullptr) return;

    dfs(root->left);
    if (pre == nullptr) { head = root; }
    else pre->right = root;
    root->left = pre;
    pre = root;
    dfs(root->right);
}

int main()
{

    return 0;
}