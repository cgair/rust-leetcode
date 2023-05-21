// 剑指 Offer 09. 用两个栈实现队列
// problem: https://leetcode.cn/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof/
#include <stack>
#include <iostream>

class CQueue {
    std::stack<int> queue;
    std::stack<int> aux;
public:
    CQueue() { }
    
    void appendTail(int value) {
        while (!queue.empty()) {
            aux.push(queue.top());
            queue.pop();
        }
        queue.push(value);
        while (!aux.empty()) {
            queue.push(aux.top());
            aux.pop();
        }
    }
    
    int deleteHead() {
        int ret = -1;
        if (queue.empty()) return ret;
        ret = queue.top();
        queue.pop();
        
        return ret;
    }
};

int main()
{
    CQueue cq;
    cq.appendTail(1);
    cq.appendTail(2);
    std::cout << cq.deleteHead() << std::endl;

    return 0;
}