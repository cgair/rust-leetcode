#include <iostream>
#include <vector>

using namespace std;

// auxiliary: O(1)
// best:      O(n^2)
// average:   O(n^2)
void selectionSort(vector<int>& nums) {
    size_t sz = nums.size();
    for(auto i = 0; i < sz - 1; ++i) {
        auto smallest = i;
        for(auto j = i + 1; j < sz; ++j) {
            if(nums[smallest] > nums[j]) { smallest = j; }
        }
        swap(nums[i], nums[smallest]);
    }
}

int main()
{
    cout << "Sort numbers ascending" << endl;
    vector<int> numbers = {4, 65, 2, -31, 0, 99, 2, 83, 782, 1};
    selectionSort(numbers);
    for(auto i : numbers) {
        cout << i << ", ";
    }
    cout << endl;
}