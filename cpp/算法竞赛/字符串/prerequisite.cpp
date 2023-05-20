#include <string>
#include <iostream>
#include <stdexcept>

int main()
{
    // 字符串的基本操作
    // 
    // 1. 构造
    // 
    // 1.1 我想要一个空字符串?
    std::string str1;
    std::cout << str1 << std::endl;

    // 1.2 我想复制已有的一个字符串
    std::string str("already a string");
    std::string str2(str);
    std::cout << str2 << std::endl;

    // initialization by part of another string
    std::string str3(str, 8, 8); // from 9th index (second parameter) 注意❗️ index 从 0 开始
                                // 8 characters (third parameter)
    std::cout << str3 << std::endl;

    // 1.4 initialization by character with number of occurrence
    std::string str4(5, '#');
    std::cout << str4 << std::endl;


    // 2. 查找
    // 
    std::string str5("This is a examples"), str6("This");
    size_t found = str5.find(str6);
    if (found != std::string::npos) {
        std::cout << str6 << " found at: " << found << '\n'; // 注意❗️从 0 开始
    }

    // 3. 替换
    std::string str7(str5);
    str7.replace(2, 7, "ese are test"); // replace(a, b, str) replaces b characters from a index by str
    std::cout << str7 << std::endl;

    // 4. 截取
    std::string str8("a string");
    std::string str88 = str8.substr(0, 1); // substr(a, b) function **returns a substring**
                                           // of b length starting from index a
    std::cout << str88 <<std::endl;
    std::string str9("a string");
    std::string str99 = str9.substr(2);
    std::cout << str99 <<std::endl;

    // 5. 删除
    std::string str10("Day day up");
    // erase(a, b) deletes b characters at index a
    str10.erase(0, 3);
    std::cout << str10 << std::endl;
    //  iterator version of erase
    str10.erase(str10.begin(), str10.end() - 2); // 头 0 开始 尾 -1 开始
    std::cout << str10 << std::endl;

    // 6. 数字和字符串的转换
    // Conversion of number into string using to_string()
    int i_val = 20;
    float f_val = 30.5;
    std::string stri2 = std::to_string(i_val);
    std::string strf2 = std::to_string(f_val);
    // Displaying the converted strings
    std::cout << "The integer in string is : "
              << stri2 << std::endl;
    std::cout << "The float in string is : "
              << strf2 << std::endl;


    // string to number
    // stox() function:
    std::string s_int1 = "123";
    int inum = std::stoi(s_int1); // This function takes a string as an argument and returns the converted integer. 
                                 // If the string contains non-numeric characters or is outside the range of an integer, 
                                 // it will throw an invalid_argument or out_of_range exception.
    std::cout << inum << std::endl;
    // It's important to note that std::stoi stops reading the string 
    // at the first character that it can't recognize as part of a number. 
    try {
        std::string s_int2 = "123xxx";
        std::size_t pos;
        int num = std::stoi(s_int2, &pos);
        if (pos < str.size()) {
            // pos is the position where a character not part of the integer was found
            std::cout << "String contains characters that couldn't be converted to an integer.\n";
        } else {
            std::cout << "The integer is " << num << "\n";
        }
    } catch (std::invalid_argument const &e) {
        std::cout << "Bad input: std::invalid_argument thrown.\n";
    } catch (std::out_of_range const &e) {
        std::cout << "Integer overflow: std::out_of_range thrown.\n";
    }


    // PS. 
    // 反转字符串
    // Reverse str[begin..end]
    std::string str11("skeegrofskeeg");
    reverse(str11.begin(), str11.end());
    assert(str11 == "geeksforgeeks");
 


    return 0;
}