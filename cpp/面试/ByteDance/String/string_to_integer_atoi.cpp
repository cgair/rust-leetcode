// 问题描述:
// 字符串转数字, 及边界条件
// 函数 myAtoi(string s) 的算法如下:
// 1. 读入字符串并丢弃无用的前导空格
// 2. 检查下一个字符 (假设还未到字符末尾) 为正还是负号, 读取该字符 (如果有). 确定最终结果是负数还是正数, 如果两者都不存在, 则假定结果为正.
// 3. 读入下一个字符, 直到到达下一个非数字字符或到达输入的结尾. 字符串的其余部分将被忽略.
// 4. 将前面步骤读入的这些数字转换为整数 (即, "123" -> 123, "0032" -> 32). 如果没有读入数字, 则整数为 0. 必要时更改符号 (从步骤 2 开始)
// 5. 如果整数数超过 32 位有符号整数范围 [−2^31,  2^31 − 1], 需要截断这个整数, 使其保持在这个范围内. 具体来说, 小于 −2^31 的整数应该被固定为 −2^31, 大于 2^31 − 1 的整数应该被固定为 2^31 − 1
// 6. 返回整数作为最终结果。
// 注意:
// 
// 本题中的空白字符只包括空格字符 ' ' .
// 除前导空格或数字后的其余字符串外, 请勿忽略任何其他字符
// 
// problem: https://leetcode.cn/problems/string-to-integer-atoi/
#include <string>
#include <cmath>

int my_atoi(std::string s) {
    if (s.empty()) return 0;
    int n = s.length();
    int pos = 0;
    bool negative = false;
    std::string number;
    while (pos < n && s[pos] == ' ') pos ++;
    if (s[pos] == '-') {
        negative = true;
        pos ++;
    } else if (s[pos] == '+') {
        pos ++;
    }

    while (pos < n && s[pos] >= '0' && s[pos] <= '9') {
        number.push_back(s[pos]);
        pos ++;
    }

    int len = number.length();
    long long ret = 0, nn = 0;
    for (int i = 0; i < len; ++i) {
        int digit = number[i] - '0';
        // 乘以 10 操作和累加操作都可能造成溢出.
        // 由 ret * 10 + digit ? INT_MAX
        if (ret > (INT_MAX - digit) / 10) {
            return !negative ? INT_MAX : INT_MIN;
        }

        ret = ret * 10 + digit;
    }

    return negative ? -ret : ret;
}


int main()
{

    return 0;
}