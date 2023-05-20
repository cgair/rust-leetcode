// 循环队列
#include <iostream>

#define N 1003 // size of queue


class Cqueue {
    int* data;
    int head;
    int rear;
    int n;
public:
    Cqueue() {
        data = (int*)malloc(N * sizeof(int));
        if (!data) exit(EXIT_FAILURE);
        n = N;
        head = rear = 0;
    }

    Cqueue(int _size) {
        data = (int*)malloc(_size * sizeof(int));
        if (!data) exit(EXIT_FAILURE);
        n = _size;
        head = rear = 0;
    }

    int size() {
        return (rear - head + n) % n; // rear == head 时为 0
    }

    // 当 rear == head 时, 队列是空的
    bool isEmpty() {
        return size() == 0;
    }

    // 当 (rear + 1) % N == head 时, 队列是满的.
    bool isFull() {
        //  The modulo operator % 
        // has higher precedence 
        // than the addition operator +
        if ((rear + 1) % n == head) return true;
        return false;
    }

    bool push(int value) {
        if (!isFull()) {
            data[rear] = value;
            rear = (rear + 1) % n;
            return true;
        }
        return false;
    }

    bool pop() {
        if (!isEmpty()) {
            data[head] = 0;
            head = (head  + 1) % n;
            return true;
        }
        return false;
    }

    int front() {
        return data[head];
    }
};

int main() 
{
    Cqueue cq(6);
    for (int i = 1; i <= 6; ++i) {
        cq.push(i);
    }

    std::cout << "front: " << cq.front() << std::endl;
    std::cout << cq.size() << std::endl;

    assert(cq.pop() == true);
    std::cout << "front: " << cq.front() << std::endl;

    // assert(cq.isFull() == true);

    return 0;
}