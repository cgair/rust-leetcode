// 问题描述:
// 给定一个数组, 前面一部分已经排好序, 后面一部分也排好序, 将整个数组排序
#include <vector>

void merge(std::vector<int>& nums) {
    int n = nums.size();
    int mid = n / 2;
    int i = 0, j = mid + 1;
    std::vector<int> aux(n);
    for (int p = 0; p < n; ++p) {
        if (i == mid + 1) {
            nums[p] = nums[j++];
            continue;
        }
        if (j == n) {
            nums[p] = nums[i++];
            continue;
        }

        if (nums[i] > nums[j]) {
            nums[p] = nums[j++];
        } else {
            nums[p] = nums[i++];
        }
    }
    nums = aux;
}

int main()
{

    return 0;
}