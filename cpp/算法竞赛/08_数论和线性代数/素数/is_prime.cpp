// 如何判断一个数是不是素数?
// <https://oi-wiki.org/math/number-theory/prime/#%E7%B4%A0%E6%95%B0%E5%88%A4%E5%AE%9A>
// 
#include <iostream>
#include <chrono>
#include <math.h>

// //////////// //
// 小素数的判定   //
// //////////// //
// 
// 由素数定义, 可用试除法:
// [2, n-1] 内所有整数试着除 n, 若都不能整除则 n 是素数
bool isPrime(int n) {
    if (n == 1) return true;
    for (int i = 2; i <= n - 1; ++i) {
        if (n % i == 0) return false;
    }

    return true;
}

// 但是真的有必要每个数都去判断吗?
// 
// 很容易发现这样一个事实:
// 如果 x 是 a 的约数, 那么
// a / x 也是 a 的约数.
// 假设存在一个大于 1 小于 n 的整数 d 能够整除 n.
// 那么必然存在另一个整数 q, 使得 n = d * q. (因数)
// 如果 d 大于 根号n, 那么 q 必然小于 根号n.
// 反之, 如果 q 大于 根号n, 那么 d 必然小于 根号n.
// 即把 [2, n-1] 缩小到 [2, √n]
bool isPrime2(int n) {
    if (n < 2) return true;
    for (int i = 2; i * i <= n; ++i) {
    // for (int i = 2; i <= sqrt(n); ++i) {
        if (n % i == 0) return false;
    }

    return true;
}


// //////////// //
// 大素数的判定   //
// //////////// //
// TBD

int main()
{
    // 1~100 所有素数
    int n = 100;
    int nums[n];

    for (int i = 0; i < n; ++i) {
        nums[i] = i+1;
    }

    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < n; ++i) {
        isPrime(nums[i]);
    }
    auto end = std::chrono::high_resolution_clock::now();
    // 计算时间差
    // std::chrono::duration<double> duration = end - start;
    auto duration_us = std::chrono::duration_cast<std::chrono::microseconds>(end - start);
    // 输出执行时间 (以秒为单位)
    std::cout << "time: " << duration_us.count() << " us" << std::endl;

    auto start1 = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < n; ++i) {
        isPrime2(nums[i]);
    }
    auto end1 = std::chrono::high_resolution_clock::now();
    // 计算时间差
    auto duration_us1 = std::chrono::duration_cast<std::chrono::microseconds>(end1 - start1);
    // 输出执行时间 (以秒为单位)
    std::cout << "time: " << duration_us1.count() << " us" << std::endl;

    return 0;
}