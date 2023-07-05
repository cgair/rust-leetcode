// https://man7.org/linux/man-pages/man3/memcpy.3.html
#include <sys/types.h>
#include <iostream>

// The memcpy() function copies n bytes from memory area src to
// memory area dest.  The memory areas must not overlap.
void* memcpy(void* dest, const void* src, size_t n) {
    if (dest == nullptr || src == nullptr) return nullptr;
    // 因为是按 1 个字节拷贝, 转换成 char* 类型来操作:
    char* p_dest = static_cast<char*>(dest);
    const char* p_src = static_cast<const char*>(src);

    // 检查 源内存 和 目标内存 是否存在内存重叠
    if (p_dest > p_src && p_dest < p_src + n) {
        // 如果目标内存首地址在源内存的中间, 则要从后往前拷贝. 
        // (因为如果从前往后拷贝, 那从目标内存首地址开始的地方就会被覆盖掉)
        for (size_t i = n - 1; i >= 0; --i) {
            p_dest[i] = p_src[i];
        }
    } else { // 如果没有重叠, 或者源内存地址在目标内存的中间. 那可以从前往后拷贝.
        for (size_t i = 0; i < n; ++i) {
            p_dest[i] = p_src[i];
        }
    }

    return dest;
}

int main() {
    // 源数据
    char src[] = "Hello, memcpy!";

    // 目标数据
    char dest[20];

    // 使用自定义的memcpy函数进行拷贝
    memcpy(dest, src, sizeof(src));

    // 打印拷贝后的目标数据
    std::cout << "Copied string: " << dest << std::endl;

    // 检查是否与标准库函数memcpy的结果一致
    if (std::memcmp(dest, src, sizeof(src)) == 0) {
        std::cout << "memcpy successful!" << std::endl;
    } else {
        std::cout << "memcpy failed!" << std::endl;
    }

    return 0;
}