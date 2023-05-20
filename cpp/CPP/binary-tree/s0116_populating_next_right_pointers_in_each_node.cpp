/** 
 * [116] Populating Next Right Pointers in Each Node
 * You are given a perfect binary tree where all leaves are on the same level, and every parent has two children.
 * Populate each next pointer to point to its next right node. If there is no next right node, the next pointer should be set to NULL.
 * 
 * 
 * Example 1:
 * Initially, all next pointers are set to NULL.
 * Input: root = [1,2,3,4,5,6,7]
 * Output: [1,#,2,3,#,4,5,6,7,#]
 * [Figure A/B](https://assets.leetcode.com/uploads/2019/02/14/116_sample.png)
 * Explanation: Given the above perfect binary tree (Figure A), your function should populate each next pointer to point to its next right node, 
 * just like in Figure B. The serialized output is in level order as connected by the next pointers, with '#' signifying the end of each level.
*/
// problem: https://leetcode.cn/problems/populating-next-right-pointers-in-each-node/
#include "../data_structure.h"
#include <queue>
#include <vector>

class Solution {
public:
    Node* connect(Node* root) {
        if (root == nullptr) return nullptr;
        std::queue<Node*> q;
        q.push(root);

        while (!q.empty()) {
            int sz = q.size();
            // std::vector<Node*> level;
            for (int i = 0; i < sz; ++i) {
                Node* n = q.front();
                q.pop();
                if (i < sz - 1) n->next = q.front();    // 明明可以一步到位!
                // level.push_back(n);
                if(n->left != nullptr) q.push(n->left);
                if(n->right != nullptr) q.push(n->right);
            }
            // int l = level.size();
            // if ( l > 1) {
            //     for(int i = 0; i < l - 1; ++i) level[i]->next = level[i + 1];
            // }
        }

        return root;
    }
};

int main()
{

    return 0;
}