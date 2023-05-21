#include <iostream>
// 堆的操作有两种: 上浮和下沉
// 上浮对应着插入操作; 下沉对应删除队头操作.
// 二叉树有 O(logn) 层, 进堆和出堆都是逐层调整
// 计算复杂度都为 O(logn)

// 最大堆
template <typename T>
class BinaryHeap {
    T* nums;
    int capacity;
    int len;
public:
    BinaryHeap(int _n) {
        nums = (T*)malloc(sizeof(T) * _n);
        nums[0] = 0;
        capacity = _n;
        len = 0;
    }

    void push(T val) {
        nums[++len] = val;
        int i = len;
        while (i > 1 && nums[i] > nums[i/2]) {
            // if (nums[i] > nums[i/2]) swap(nums[i], nums[i/2]);
            std::swap(nums[i], nums[i/2]);
            i = i / 2;
        }
    }

    // 下沉, 删除堆头, 调整堆
    T pop() {
        if (len == 0) {
            std::cerr << "Heap is empty.\n";
            exit(EXIT_FAILURE);
        }

        T ret = nums[1];
        nums[1] = nums[len--];
        int i = 1;
        while (2 * i <= len) {
            int son = 2 * i;
            // 先从儿子中选最大的那个
            if (son < len && nums[son] < nums[son + 1]) son ++;
            if (nums[i] < nums[son]) {
                std::swap(nums[i], nums[son]);
                i = son;
            } else {
                break;
            }
        }
        return ret;
    }
};

int main()
{
    BinaryHeap<int> binary_heap(10);
    // std::cout << binary_heap.pop() << std::endl;
    for (int i = 1; i <= 10; ++i) binary_heap.push(i);
    std::cout << binary_heap.pop() << std::endl;
    std::cout << binary_heap.pop() << std::endl;
    std::cout << binary_heap.pop() << std::endl;
    std::cout << binary_heap.pop() << std::endl;

    return 0;
}