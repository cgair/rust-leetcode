#include <iostream>
#include <vector>

using namespace std;
 
// auxiliary: O(1)
// best:      O(n)
// average:   O(n^2)
void bubbleSort(vector<int>& nums) {
    size_t len = nums.size();
    for(auto i = 0; i < len - 1; ++i) {
        for(auto j = 0; j < len - 1 - i; ++j) {
            if(nums[j] > nums[j + 1]) {
                swap(nums[j], nums[j + 1]);
            }
        }
    }
}

int main()
{
    cout << "Sort numbers ascending" << endl;
    vector<int> numbers = {4, 65, 2, -31, 0, 99, 2, 83, 782, 1};
    bubbleSort(numbers);
    for(auto i : numbers) {
        cout << i << ", ";
    }
    cout << endl;
}