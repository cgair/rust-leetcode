#include <iostream>
#include <vector>
#include <random>

#define MAX 100  // 10000000
#define MIN -100


// 基本二分代码
int binarySearch(std::vector<int>& nums, int target) {
    int left = 0, right = nums.size(); // [0, n)

    while (left < right)
    {
        int mid = left + (right - left) / 2;
        if (nums[mid] == target) {
            return mid;
        } else if (nums[mid] < target) {
            left = mid + 1;
        } else if (nums[mid] > target) {
            right = mid;
        }
    }

    return -1;
}


// 搜索左侧边界
int left_bound(std::vector<int>& nums, int target) {
    int left = 0, right = nums.size();
    while (left < right) {
        int mid = left + (right - left) / 2;
        if (nums[mid] == target) {
            right = mid;
        } else if (nums[mid] < target) {
            left = mid + 1;
        } else if (nums[mid] > target) {
            right = mid;
        }
    }
    if (left == nums.size()) return -1;
    return nums[left] == target ? left : -1;
}


// 搜索右侧边界
int right_bound(std::vector<int>& nums, int target) {
    int left = 0, right = nums.size();
    while (left < right) {
        int mid = left + (right - left) / 2;
        if (nums[mid] == target) {
            left = mid + 1;
        } else if (nums[mid] < target) {
            left = mid + 1;
        } else if (nums[mid] > target) {
            right = mid;
        }
    }
    if (right - 1 < 0) return -1; 
    return nums[right - 1] == target ? right - 1 : -1;
}


unsigned long ulrand() {
    return (
        (((unsigned long)rand() << 24) & 0xFF000000ul)
       |(((unsigned long)rand() << 12) & 0x00FFF000ul)
       | (((unsigned long)rand())      & 0x00000FFFul)
    );
}


void print(std::vector<int>& nums) {
    int n = nums.size();
    std::cout << "[";
    for (int i= 0; i < n; ++i) {
        std::cout << nums[i] << ((i != n - 1)? ", " : "");
    }
    std::cout << "]\n";
}


int main(int argc, char* argv[])
{
    if (argc == 1)
        std::cout << "No extra Command Line Argument passed "
                     "other than program name"
                  << std::endl;

    // initialize the random number generator 
    // with a seed based on the current time.
    srand(time(0));

    int n = std::stoi(argv[1]);
    std::vector<int> nums(n);
    for (int i = 0; i < n; ++i) {
        nums[i] = ulrand() % (MAX - MIN + 1) + MIN; // 产生 [MIN, MAX]内的随机数, 有正有负
    }
    std::sort(nums.begin(), nums.end());
    print(nums);

    std::vector<int> arr = {5,7,7,8,8,10};
    std::cout << binarySearch(arr, 8) << std::endl;

    // 与 STL lower_bound()/upper_bound()对比
    // 
    // Syntax: std::lower_bound(first, last, value)
    // Returns an iterator pointing to the first element that is not less than value in the sorted range [first, last).
    // If no such element is found, it returns an iterator pointing to the position where the element would be inserted to maintain the sorted order.
    // The range [first, last) must be sorted in ascending order for std::lower_bound to work correctly.
    auto lower = std::lower_bound(arr.begin(), arr.end(), 7);
    if (lower != arr.end()) {
        std::cout << "lower bound: "
                  << std::distance(arr.begin(), lower) << std::endl;
        std::cout << "left bound: "
                << left_bound(arr, 7) << std::endl;
    }

    // The upper_bound function returns an iterator pointing to 
    // the first element that is greater than the target value 
    auto upper = std::upper_bound(arr.begin(), arr.end(), 8);
    if (upper != arr.end()) {
        std::cout << "upper bound: "
                  << std::distance(arr.begin(), upper) << std::endl;
        std::cout << "right bound: "
                  << right_bound(arr, 8) << std::endl;
    }

    return 0;
}

