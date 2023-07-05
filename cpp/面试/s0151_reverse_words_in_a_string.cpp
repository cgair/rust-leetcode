// 151. 反转字符串中的单词
// https://leetcode.cn/problems/reverse-words-in-a-string/
#include <string>
#include <vector>
#include <iostream>
using namespace std;

// follow up: 
// 如果字符串在你使用的编程语言中是一种可变数据类型, 
// 请尝试使用 O(1) 额外空间复杂度的 原地解法.

class Solution {
public:
    vector<string> splitStr(string s, char delimiter) {
        string token("");
        vector<string> ret;
        for (auto &ch : s) {
            if (ch == delimiter) {
                ret.push_back(token);
                token = "";
            }
            token += ch;
        }
        if (s.back() != delimiter) ret.push_back(token);

        return ret;
    }

    void printVector(vector<string>& vec) {
        for (auto &v: vec) {
            cout << v << ((v != vec.back()) ? "," : "");
        }
        cout << endl;
    }

    string reverseWords(string s) {
        string ret("");
        vector<string> splitted = splitStr(s, ' ');
        // printVector(splitted);
        vector<string> trimmed;
        for (auto &spl : splitted) {
            if (spl != "" && spl != " ") {
                trimmed.push_back(spl);
            }
        }
        // printVector(trimmed);
        
        reverse(trimmed.begin(), trimmed.end());
        for (auto &t : trimmed) {
            ret += t;
            if (t != trimmed.back()) ret += " ";
        }
        
        return ret;
    }

    // string reverseWords2(string s) {

    // }
};


int main()
{
    Solution s;

    string s1 = "the sky is blue";
    string s2 = "  hello world  ";
    string s3 = "a good   example";

    cout << s.reverseWords(s1) << endl;
    // cout << s.reverseWords(s2) << endl;
    // cout << s.reverseWords(s3) << endl;

    return 0;
}