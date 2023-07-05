// 问题描述:
// 排序数组中绝对值不同的个数
#include <iostream>
#include <vector>
#include <unordered_set>
#include <cmath>

int count_distinct_abs(const std::vector<int>& nums) {
    std::unordered_set<int> uset;
    for (auto n : nums) {
        uset.emplace(std::abs(n));
    }

    return uset.size();
}

// 其实看到有序数组, 首先想到的就是
// 二分查找 / 双指针 (尺取法)
int count_distinct_abs2(const std::vector<int>& nums) {
    int ret = 0;
    int n = nums.size();
    int l = 0, r = n - 1;
    while (l < r) {
        while (l < n - 1 && nums[l] == nums[l + 1]) { l ++; }
        while (r > 1 && nums[r] == nums[r - 1]) { r --; }
        if (nums[l] + nums[r] < 0) { l ++; ret++; }
        else if (nums[l] + nums[r] > 0) { r --; ret ++; }
        else if (nums[l] + nums[r] == 0) { l++; r --; ret ++; }
    }

    return ret + 1;
}

int main()
{
    int t;
    std::cin >> t;
    while (t--) {
        int n;
        std::cin >> n;
        std::vector<int> nums(n);
        for (int i = 0; i < n; ++i) {
            std::cin >> nums[i];
        }

        std::cout << count_distinct_abs(nums) << std::endl;
        std::cout << count_distinct_abs2(nums) << std::endl;
    }

    return 0;
}