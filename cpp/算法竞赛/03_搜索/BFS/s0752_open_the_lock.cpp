// 752. 打开转盘锁
// problem: https://leetcode.cn/problems/open-the-lock/
#include <vector>
#include <string>
// #include <set>
#include <unordered_set>
#include <queue>
using namespace std;

class Solution {
public:
    // ops: minus 1 --> 0, plus 1 --> 1;
    string operateCode(string& code, int pos, int ops) {
        string ret = code;
        if (ops) {
            if (ret[pos] == '9') ret[pos] = '0';
            else {
                ret[pos] += 1;
            }
        } else {
            if (ret[pos] == '0') ret[pos] = '9';
            else ret[pos] = ret[pos] -= 1;
        }

        return ret;
    }

    int openLock(vector<string>& deadends, string target) {
        // set<string> st;
        unordered_set<string> deads, st;
        for (string s : deadends) {
            deads.insert(s);
        }
        queue<string> q;
        q.push("0000");
        st.insert("0000");
        int depth = 0;
        
        while (!q.empty()) {
            int sz = q.size();
            for (int i = 0; i < sz; ++i) {
                string secret = q.front();
                q.pop();
                if (deads.count(secret)) {
                    continue;
                }
                if (secret == target) return depth;

                for (int i = 0; i < 4; ++i) {
                    string minus_one = operateCode(secret, i, 0);
                    if (st.find(minus_one) == st.end()) {
                        q.push(minus_one);
                        st.insert(minus_one);
                    }

                    string plus_one = operateCode(secret, i, 1);
                    if (!st.count(plus_one)) {
                        q.push(plus_one);
                        st.insert(plus_one);
                    }
                }
            }
            depth++;
        }

        return -1;
    }
};