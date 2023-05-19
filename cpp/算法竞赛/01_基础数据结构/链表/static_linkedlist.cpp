#include <iostream>
#include <vector>

#define MAX_NUM 150


/**
 * [静态链表]
 * 优点: 省去动态分配和释放内存的麻烦
 * 一维数组实现单向静态链表:
 *    定义一个一维数组 nodes[], 
 *    nodes[i] 中的 i 就是节点的值, 
 *    而 nodes[i]的值就是下一个节点.
 */
std::vector<int> constructLinkedList() {
    std::vector<int> nodes(MAX_NUM, 0);
    int n = 10;
    nodes.resize(n + 1);
    // int nodes[MAX_NUM];
    for (int i = 1; i < n; ++i) nodes[i] = i + 1;
    
    // uncomment the following line to construct a linkedlist cycle
    // nodes[n] = 1;

    return nodes;
}

void printLinkedList(const std::vector<int>& nodes) {
    for (auto &n : nodes) {
        std::cout << n << ((n != nodes.back()) ? " " : "");
    }
    std::cout << std::endl;
}

int main()
{
    std::vector<int> nodes = constructLinkedList();
    printLinkedList(nodes);

    return 0;
}