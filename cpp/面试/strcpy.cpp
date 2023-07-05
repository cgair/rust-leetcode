// https://man7.org/linux/man-pages/man3/strcpy.3.html
#include <cassert>
#include <iostream>

char* strcpy(char* dest, const char* src) {
    assert(dest != nullptr && src != nullptr);
    char* address = dest;
    while ((*dest++ = *src++) != '\0');
    
    return address;
}
// 源字符串的长度超出目标字符串时, 
// 导致把数据写入到无法控制的地址中去
// 所以就有了 strncpy


int main()
{
    const char* source = "Hello, World!";
    char destination[20];
    strcpy(destination, source);
    std::cout << "Copied string: " << destination << std::endl;
    
    return 0;
}