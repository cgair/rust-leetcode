// [HJ23.删除字符串中出现次数最少的字符]
// <https://www.nowcoder.com/practice/05182d328eb848dda7fdd5e029a56da9?tpId=37&tqId=21246&rp=1&ru=/exam/oj/ta&qru=/exam/oj/ta&sourceUrl=%2Fexam%2Foj%2Fta%3Fpage%3D1%26pageSize%3D50%26search%3D%26tpId%3D37%26type%3D37&difficulty=undefined&judgeStatus=undefined&tags=&title=> 
#include <iostream>
#include <string>

std::string deleteStr(std::string str) {
    std::string result;
    // there are 26 characters in the Alphabet
    // let ascii <=> index 
    int nums[26] = {0};
    int min = INT_MAX;

    for (auto ch : str) {
        nums[ch - 'a'] ++;
    }

    for (int i = 0; i < 26; ++i) {
        if (nums[i] != 0 && nums[i] < min) min = nums[i];
    }

    for (int i = 0; i < str.length(); ++i) {
        if (nums[str[i] - 'a'] > min) result += str[i];
    }

    return result;
}

int main() {
    std::string s;
    while (std::cin >> s) {
        std::cout << deleteStr(s) << std::endl;
    }

    return 0;
}