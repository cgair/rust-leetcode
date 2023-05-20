// problem: https://leetcode.cn/problems/koko-eating-bananas/
#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
    // 算法要求的是 H 小时内吃完香蕉的最小速度 (speed), 
    // 请问 speed 最大可能为多少, 最少可能为多少呢?
    // 显然最少为 1, 最大为 max(piles), 因为一小时最多只能吃一堆香蕉
    int minEatingSpeed(vector<int>& piles, int h) {
        int left = 1;
        int right = 0;
        for(auto p : piles) {
            right = max(p, right);
        }
        right += 1;
        
        while(left < right) {
            int mid = left + (right - left) / 2;
            if(canFinnish(piles, mid, h)) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }

    bool canFinnish(vector<int>& piles, int speed, int h) {
        int time = timeOf(piles, speed);
        
        return time <= h ? true : false;
    }

    int timeOf(vector<int>& piles, int speed) {
        int time = 0;
        for(auto p : piles) {
            time = time + p / speed + (( p % speed > 0) ? 1: 0);
        }

        return time;
    }
};

int main() 
{
    Solution s;
    vector<int> piles = {30,11,23,4,20};
    int ret = s.minEatingSpeed(piles, 5);
    assert(ret == 30);
}