#include <iostream>

// 1. 用倍增法计算快速幂
// 
// a^11 为例: a^11 = a^8 * a^2 * a^1, 所有幂 a^i 都是倍乘关系 (a^2 = a^1 * a^1, ...)
// 可以逐级递推: a *= a
// 两步走: 1) 幂次用二进制分解; 2) 跳过没有的幂次 (如 a^11 需跳过 a^4)
// 
int fastPow(int a, int n) {
    int ret = 1;
    while (n) {
        if (n & 1) ret *= a;
        a *= a;
        n >>= 1;
    }

    return ret;
}

// 幂运算结果往往是大数, 往往取模在输出
// a^n mod m = (a mod m)^n mod m
typedef long long ll;
ll fastPow(ll a, ll n, ll mod) {
    ll ret = 1;
    a = a % mod;    // 一定程度上防止 a * a 越界
    while(n) {
        if (n & 1) ret = (ret * a) % mod;  // 注意❗️ 可能越界
        a = (a * a) % mod;   // 注意❗️ 可能越界
        n >>= 1;
    }

    return ret;
}


// 2. 分治法
int fastPow2(int a, int n) {
    if (n == 0) return 1;
    int mid = fastPow2(a, n / 2);
    return n % 2 == 0 ? mid * mid : mid * mid * a;
}

int main()
{
    std::cout << fastPow(2, 3) << std::endl;
    std::cout << fastPow2(2, 3) << std::endl;
    return 0;
}