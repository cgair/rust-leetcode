#include <iostream>
#include <vector>
#include <algorithm>

void print(std::vector<int>& vec) {
    for (auto v : vec) {
        std::cout << v << ((v != vec.back()) ? ", " : "");
    }
    std::cout << std::endl;
}

// sort() 可以用自定义顺序排序
// 也可以用提供的 4 中顺序: less, greater, less_equal, greater_equal

bool myLess(int i, int j) {
    return i < j;
}

int main()
{
    // size = 8
    std::vector<int> nums = {3, 5, 6, 2, 1, 8, 5, 6};
    std::sort(nums.begin(), nums.begin() + 5); // 对前 5 个排序
    print(nums);
    std::sort(nums.begin(), nums.end());
    print(nums);

    std::sort(nums.begin(), nums.end(), std::less<int>());
    std::sort(nums.begin(), nums.end(), myLess);
    print(nums);

    int arr[5] = {3,5,6,2,1};
    std::sort(arr, arr + 5, std::greater<int>());
    for (int i = 0; i < 5; ++i) {
        std::cout << arr[i] << (i == 4 ? "" : ", ");
    }
    std::cout << std::endl;

    return 0;
}