// 哈夫曼编码 (Huffman Coding)
// 前缀 (Prefix) 编码的一种最优算法
// 
// 给定一个字符串, 如何使编码后的总长度最小?
// i.e., 如何得到一个最优解?
// 
// 编码二叉树如何构造的?
// 1. 对所有字符按出现频次排序 (从小到大)
// 2. 从频次出现最低的字符开始, 按哈夫曼算法执行 详见 《算法竞赛》P25
// 
// Implementation of Huffman greedy algorithm based the 
// huffman algorithm pseudocode in "Introduction to Algorithms, Third Edition"
#include <vector>
#include <iostream>
#include <queue>

struct Char {
    char ch;
    unsigned int freq;
    Char() : ch(0), freq(0) { }
    Char(char c, int f) : ch(c), freq(f) { }
};

class TreeNode {
public:
    Char ch;
    TreeNode* left;
    TreeNode* right;
    TreeNode() : ch(), left(nullptr), right(nullptr) { }
    TreeNode(Char c) : ch(c), left(nullptr), right(nullptr) { }
    ~TreeNode() {
        // 添加额外的检查, 确保在删除指针之前它们不为空指针
        // 这可以防止潜在的错误.
        if (left != nullptr) {
            delete left;
            left = nullptr;
        }
        if (right != nullptr) {
            delete right;
            right = nullptr;
        }
    }

    bool operator< (const TreeNode& rhs) {
        return this->ch.freq < rhs.ch.freq;
    }

    bool operator> (const TreeNode& rhs) {
        return this->ch.freq > rhs.ch.freq;
    }
};

TreeNode* huffman(std::vector<Char>& charset) {
    int n = charset.size();

    std::priority_queue<TreeNode*, std::vector<TreeNode*>, std::greater<TreeNode*>> f;
    for (auto c : charset) {
        f.push(new TreeNode(c));
    }

    // Get non-leaf and leaf nodes:
    // get the lowest two frequencies objects, then merge them, and insert the merged object into pq
    for (int k = 0; k < n - 1; ++k) {
        TreeNode* x = new TreeNode();
        
        TreeNode* i = f.top();
        f.pop();
        TreeNode* j = f.top();
        f.pop();

        x->left = i;
        x->right = j; 
        x->ch.freq = i->ch.freq + j->ch.freq;
        f.push(x);
    }

    return f.top();
}

void preorder_traverse(TreeNode* root) {
    if (root == nullptr) return;
    std::cout << root->ch.freq
              << std::endl;
    preorder_traverse(root->left);
    preorder_traverse(root->right);
}

int main()
{
    unsigned int frequency[] = {45, 13, 12, 16, 9, 5};
    int n = sizeof(frequency) / sizeof(frequency[0]);

    std::vector<Char> charset(n);
    for (int i = 0; i < n; ++i) {
        charset.push_back(Char('a' + i, i));
    }

    TreeNode* root = huffman(charset);
    std::cout << " preorder sequence of huffman binary tree: " << std::endl;
    preorder_traverse(root);

        
    getchar();

    return 0;
}