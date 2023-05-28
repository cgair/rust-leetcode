// 问题描述: 给定一个数, 求它的平方根
// (不能使用内置函数, 如 sqrt() 函数)
// 
#include <iostream>
#include <cmath>

// 二分法
double mySqrt(int n, double precision) {
    double left = 0, right = n;
    
    while (right - left > precision) {
        double mid = left + (right - left) / 2;
        if (mid * mid == n) return mid;
        else if (mid * mid < n) {
            left = mid;
        } else if (mid * mid > n) {
            right = mid;
        }
    }

    // the variable left will hold 
    // the approximate square root value 
    // after the binary search converges.
    return left;
}

// 牛顿迭代法

int main()
{
    int n = 2;
    double precision = 1e-10;
    std::cout << "from std::sqrt(): " << std::sqrt(n) << std::endl;
    std::cout << "   from mySqrt(): " << mySqrt(n, precision) << std::endl;

    return 0;
}