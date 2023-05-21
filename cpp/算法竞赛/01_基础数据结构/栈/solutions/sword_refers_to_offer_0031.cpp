// 剑指 Offer 31. 栈的压入、弹出序列
// problem: https://leetcode.cn/problems/zhan-de-ya-ru-dan-chu-xu-lie-lcof/
#include <vector>
#include <stack>
#include <iostream>
using namespace std;

class Solution {
public:
    bool validateStackSequences(vector<int>& pushed, vector<int>& popped) {
        if (pushed.empty() || popped.empty()) return false;
        stack<int> stack;
        for (int i = 0, j = 0; i < pushed.size(); ++i) {
            stack.push(pushed[i]);
            while (!stack.empty() && stack.top() == popped[j]) {
                stack.pop();
                j++;
            }
        }

        return stack.empty();
    }
};

int main()
{
    Solution s;
    vector<int> pushed = {1,2,3,4,5}, popped = {4,5,3,2,1};
    std::cout << s.validateStackSequences(pushed, popped) << std::endl;
    pushed.clear();
    popped.clear();
    std::cout << s.validateStackSequences(pushed, popped) << std::endl;

    return 0;
}