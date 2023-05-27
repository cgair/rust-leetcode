// 桶排序 (Bucket sort)
// See also:
// <https://oi-wiki.org/basic/bucket-sort/>
#include <vector>
#include <iostream>

const int N = 1000010; // 桶的最大大小

void insertion_sort(std::vector<int>& nums) {
    for (int i = 1; i < nums.size(); ++i) {
        int j = i;
        while (j > 0 && nums[j] < nums[j - 1]) {
            std::swap(nums[i=j], nums[j-1]);
            j--;
        }
    }
}

void bucket_sort(std::vector<int>& nums) {
    int n = nums.size();
    int k = 5; // k 个桶

    // 获取输入元素最大值
    int max = INT_MIN;
    for (int i = 0; i < n; ++i) {
        max = std::max(max, nums[i]);
    }

    int bucket_size = max / k + 1; // w 分成 k 个桶, 加上 1 应对 max 无法整除 n 的情况

    // 初始化桶
    std::vector<int> bucket[k];
    for (int i = 0; i < k; ++i) {
        bucket[i].clear();
    }

    // 遍历输入数组 nums, 
    // 将每个元素根据其值放入相应的桶中
    for (int i = 0; i < n; ++i) {
        bucket[nums[i] / bucket_size].push_back(nums[i]);
    }

    int p = 0;
    for (int i = 0; i < k; ++i) {
        // 将每个桶内的元素进行排序
        insertion_sort(bucket[i]);
        // 遍历所有的桶输出所有元素
        for (int j = 0; j < bucket[i].size(); ++j) {
            nums[p++] = bucket[i][j];
        }
    }
}

int main()
{
    std::vector<int> nums = {9, 3, 7, 1, 5, 2, 8, 6, 4};
    bucket_sort(nums);

    for (int i = 0; i < nums.size(); ++i) {
        std::cout << nums[i] << " ";
    }
    std::cout << std::endl;

    return 0;
}