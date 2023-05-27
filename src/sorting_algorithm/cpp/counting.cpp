// 计数排序 (Counting sort)
// 作为一种线性时间复杂度的排序, 计数排序要求输入的数据必须是有确定范围的**整数.**
// 当然如果输入包含负数, count[] 大小 max - min + 1, 然后做元素映射 count[nums[i] - min];
// See also: 
// 1. [什么是计数排序?](https://cloud.tencent.com/developer/article/1684188)
// 2. <https://oi-wiki.org/basic/counting-sort/>
#include <vector>
#include <iostream>

void countingSort(std::vector<int>& nums) {
    int max = INT32_MIN;
    for (int i = 0; i < nums.size(); ++i) {
        max = std::max(nums[i], max);
    }

    std::vector<int> count(max + 1, 0);
    for (auto n : nums) {
        count[n] ++;
    }

    // 如果不考虑计数排序的稳定性, 顺序输出数组 count[] 即可:
    std::cout << "Do not consider stabilization:\n";
    for (int i = 0; i < count.size(); ++i) {
        if (count[i] != 0) {
            for (int j = 0; j < count[i]; ++j) {
                std::cout << i << " ";
            }
        }
    }
    std::cout << std::endl;

    std::cout << "       Consider stabilization:\n";
    // TODO
}

int main() {
    std::vector<int> nums = {};
    countingSort(nums);

    return 0;
}