// C++ program to demonstrate the use of priority_queue
// <https://cplusplus.com/reference/queue/priority_queue/>
// [Priority Queue in C++ Standard Template Library](https://www.geeksforgeeks.org/priority-queue-in-cpp-stl/)
#include <iostream>
#include <queue>

// How to create a min heap for the priority queue?
// C++ program to demonstrate
// min heap for priority queue
void minHeap() {
    int arr[6] = { 10, 2, 4, 8, 6, 9 };
    std::priority_queue<int, std::vector<int>, std::greater<int> > gpq(arr, arr + 6);
    std::cout << "  Original arr: ";
    for (auto n : arr) {
        std::cout << n << " ";
    }
    std::cout << std::endl;
    std::cout << "Priority Queue: ";
    while (!gpq.empty()) {
        std::cout << gpq.top() << " ";
        gpq.pop();
    }
    std::cout << std::endl;
}


int main()
{   
    int nums[6] = {6, 1, 4, 2, 3, 5};
    std::priority_queue<int> pq;
    
    std::cout << " Original nums: ";
    for (auto n : nums) {
        std::cout << n << " ";
    }
    std::cout << std::endl;

    for (int i = 0; i < 6; ++i) {
        pq.push(nums[i]);
    }

    // printing priority queue
    // In C++ STL, the top element is always the greatest by default.
    std::cout << "Priority Queue: ";
    while (!pq.empty()) {
        std::cout << pq.top() << " ";
        pq.pop();
    }
    std::cout << std::endl;

    std::cout << "How to create a min heap for the priority queue?" << std::endl;
    minHeap();

    return 0;
}