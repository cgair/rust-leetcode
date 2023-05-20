// problem: https://leetcode.cn/problems/permutation-ii-lcci/
// [面试题 08.08.] 有重复字符串的排列组合
// 编写一种方法, 计算某字符串的所有排列组合.
// 
// 
// 示例1:
// 输入: S = "qqe"
// 输出: ["eqq","qeq","qqe"]
// 
// 
// 示例2:
// 输入: S = "ab"
// 输出: ["ab", "ba"]
// 
// 
// 提示:
// 字符都是英文字母.
// 字符串长度在 [1, 9] 之间.
#include <vector>
#include <string>
#include <iostream>
#include <algorithm>

class Solution {
public:
    std::vector<std::string> permutation(std::string S) {
        int s = S.length();
        std::vector<bool> used(s, false);
        std::vector<std::string> ret;
        std::string path = "";
        std::sort(S.begin(), S.end());
        backtrack(S, path, used, ret);

        return ret;
    }

    void backtrack(
        std::string& choices, 
        std::string& path,
        std::vector<bool>& used,
        std::vector<std::string>& ret
    ) {
        // std::cout << path << std::endl;
        if (path.length() == choices.length()) {
            ret.push_back(path);
            return;
        }

        for (int i = 0; i < choices.length(); ++i) {
            if (used[i]) continue;
            if (i > 0 && choices[i] == choices[i - 1] && used[i - 1]) continue;
            path += choices[i]; 
            used[i] = true;
            backtrack(choices, path, used, ret);
            path.pop_back();
            used[i] = false;
        }

    }
};


int main() 
{
    std::string str = "qqe";
    Solution s;
    std::vector<std::string> ret = s.permutation(str);
    for(auto &s : ret) {
        std::cout << s << " ";
    }
    std::cout << std::endl;

    return 0;
}