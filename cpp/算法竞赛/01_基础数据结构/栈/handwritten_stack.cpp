#include <iostream>

template <class T>
class MyStack {
    T* elements;
    int t;    // 栈顶位置
    int size;
public:
    MyStack(int n) {
        elements = (T*)malloc(sizeof(T) * n);
        t = -1;
        size = n;
    }

    void push(T val) {
        if (t > size - 1) {
            std::cout << "Stack is full\n";
            return;
        }
        elements[++t] = val;    // 注意❗️ use ++t instead of t++
    }
    
    T top() {
        return elements[t];
    }

    void pop() {
        if (isEmpty()) {
            std::cout << "Nothing to pop\n";
            return;
        }
        elements[t--] = 0;
    }

    bool isEmpty() {
        return t == 0;
    }
};

int main()
{
    MyStack<int> stack(10);
    for (int i = 1; i <= 10; ++i) {
        stack.push(i);
    }
    std::cout << stack.top() << std::endl;
    stack.pop();
    std::cout << stack.top() << std::endl;

    return 0;
}
