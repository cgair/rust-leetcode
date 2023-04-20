// problem: https://leetcode.cn/problems/zlDJc7/
#include <iostream>
#include <vector>
#include <string>
#include <queue>
#include <unordered_set>

using namespace std;
/**
 * @brief 思路
 * 第一步, 不管所有的限制条件(不管 deadends 和 target 的限制), 就思考一个问题: 如果让你设计一个算法, 穷举所有可能的密码组合, 怎么做? 
 * 穷举: 从 "0000" 开始, 转一次, 可以穷举出 "1000", "9000", "0100", "0900"... 共 8 种密码.
 * 然后, 再以这 8 种密码作为基础, 对每个密码再转一下, 穷举出...
 * 
 * 于是, 这就可以抽象成一幅图, 每个节点有 8 个相邻的节点, 又让求最短距离 ---> 是典型的 BFS
 */

class Solution {
public:
    vector<string> deadends = {"0201", "0101", "0102", "1212", "2002"};
    int openLock(vector<string>& deadends, string target) {
        return Solution::bfs(deadends, target);
    }

    int bfs(vector<string>& deadends, string target) {
        unordered_set<string> deads;
        for(auto d : deadends) { deads.insert(d); }
        unordered_set<string> visited;

        queue<string> queue;
        int step = 0;
        queue.push("0000");
        visited.insert("0000");

        while(!queue.empty()) {
            int sz = queue.size();
            for(int i = 0; i < sz; ++i) {
                string curr = queue.front();
                queue.pop();

                cout << curr << endl;
                if(!deads.count(curr)) { continue; }
                if(curr == target) { return step;}

                for(int i = 0; i < 4; ++i) {
                    string up = plusOne(curr, i);
                    if(!visited.count(up)) {
                        queue.push(up);
                        visited.insert(up);
                    }
                    string down = minusOne(curr, i);
                    if(!visited.count(down)) {
                        queue.push(down);
                        visited.insert(down);
                    }
                }
            }
            step ++;
        }

        return -1;
    }    
};


// 将 s[index] 向上拨动一次
string plusOne(string s, int index) {
    char ch[s.length()];
    strcpy(ch, s.c_str());
    if(ch[index] == '9') { ch[index] = '0'; }
    else {
        ch[index] += 1;
    }
    string ret(ch);

    return ret; 
}

// 将 s[index] 向下拨动一次
string minusOne(string s, int index) {
    char ch[s.length()];
    strcpy(ch, s.c_str());
    if(ch[index] == '9') { ch[index] == '0'; }
    else {
        ch[index] -= 1;
    }
    string ret(ch);

    return ret;
}

int main() {
    return 0;
}