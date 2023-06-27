#include <vector>
#include <iostream>

void insertion_sort(std::vector<int>& nums) {
    int n = nums.size();
    for (int i = 1; i < n; ++i) {
        int j = i;
        while (j > 0 && nums[j] < nums[j - 1]) {
            std::swap(nums[j], nums[j - 1]);
            j --;
        }
    }
}


int main()
{

    std::cout << "Sort numbers ascending" << std::endl;
    std::vector<int> numbers = {4, 65, 2, -31, 0, 99, 2, 83, 782, 1};
    insertion_sort(numbers);
    for(auto i : numbers) {
        std::cout << i << ", ";
    }
    std::cout << std::endl;
}