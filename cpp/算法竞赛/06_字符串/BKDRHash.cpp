// Brian Kernighan & Dennis Ritchie
// BKDRHash: 一种进制哈希
// 
// 1. 设定一个进制 P (常用值: 31, 131, 1313, 13131, 131313)
// 2. 字符串的每个字符看作一个数字
// 3. 把每个字符按进制 P 的权值相加然后对 M 取余
// 
// "隐性取余": 避免低效的的取余运算
// 取空间大小 M = 2^64, 64 是 unsigned long long, H > M 自动溢出
// 在计算机中, 整数类型的溢出是一种定义良好的行为.
// 当一个 unsigned long long 类型的值超出了其表示范围时, 
// 它会从最小值重新开始, 也就是说, 它会自动进行模运算, 将溢出的部分回卷到该类型的范围内.
// 
#include <iostream>
#include <string>

#define ull unsigned long long

ull BKDRHash(std::string& s) {
    ull P = 131, H = 0;
    for (auto ch : s)
        // a = 1, b = 2, c = 3, ..., z = 26
        H = H * P + ch - 'a' + 1;
    
    return H; // 不需要再取余
}

int main()
{
    std::string s("string");
    std::string s1("strinh");
    std::string s2("strinj");

    std::cout << BKDRHash(s) << std::endl;
    std::cout << BKDRHash(s1) << std::endl;
    std::cout << BKDRHash(s2) << std::endl;

    return 0;
}