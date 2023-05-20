// [1352] Product of the Last K Numbers
// Design an algorithm that accepts a stream of integers and retrieves the product of the last k integers of the stream.
// 
// Implement the ProductOfNumbers class:
// 
// ProductOfNumbers() Initializes the object with an empty stream.
// void add(int num) Appends the integer num to the stream.
// int getProduct(int k) Returns the product of the last k numbers in the current list. 
//                       You can assume that always the current list has at least k numbers.
// The test cases are generated so that, at any time, 
// the product of any contiguous sequence of numbers 
// will fit into a single 32-bit integer without overflowing.
// 
// Example:
// 
// 
// Input
// ["ProductOfNumbers","add","add","add","add","add","getProduct","getProduct","getProduct","add","getProduct"]
// [[],[3],[0],[2],[5],[4],[2],[3],[4],[8],[2]]
// 
// 
// Output
// [null,null,null,null,null,null,20,40,0,null,32]
// 
// 
// Explanation
// ProductOfNumbers productOfNumbers = new ProductOfNumbers();
// productOfNumbers.add(3);        // [3]
// productOfNumbers.add(0);        // [3,0]
// productOfNumbers.add(2);        // [3,0,2]
// productOfNumbers.add(5);        // [3,0,2,5]
// productOfNumbers.add(4);        // [3,0,2,5,4]
// productOfNumbers.getProduct(2); // return 20. The product of the last 2 numbers is 5 * 4 = 20
// productOfNumbers.getProduct(3); // return 40. The product of the last 3 numbers is 2 * 5 * 4 = 40
// productOfNumbers.getProduct(4); // return 0. The product of the last 4 numbers is 0 * 2 * 5 * 4 = 0
// productOfNumbers.add(8);        // [3,0,2,5,4,8]
// productOfNumbers.getProduct(2); // return 32. The product of the last 2 numbers is 4 * 8 = 32 
#include <iostream>
#include <vector>

class ProductOfNumbers {
public:
    ProductOfNumbers() { 
        pre_product.push_back(1);
    }
    
    // 一定要注意 add(0) 的问题!
    void add(int num) {
        if (num == 0) {
            pre_product.clear();
            pre_product.push_back(1);
            return;
        }
        int sz = pre_product.size();
        pre_product.push_back(pre_product[sz - 1] * num);
    }
    
    int getProduct(int k) {
        int sz = pre_product.size();
        if (k > sz - 1) { return 0; }

        return pre_product[sz - 1] / pre_product[sz - 1 - k];
    }

    void printPre() {
        std::cout << "[ ";
        for (auto &v : pre_product) {
            std::cout << v << ", ";
        }

        std::cout << "]\n";
    }
private: 
    std::vector<int> pre_product;
    std::vector<int> nums;
};


int main()
{   
    std::vector<int> v = {1, 2, 3, 4, 5};
    v.clear();
    if (v.empty()) std::cout << "Empty vector\n";

    ProductOfNumbers pon;
    pon.add(3);
    pon.printPre();
    pon.add(0);
    pon.printPre();
    pon.add(2);
    pon.add(5);
    pon.add(4);
    pon.printPre();
    int gp1 = pon.getProduct(2);
    int gp2 = pon.getProduct(3);
    int gp3 = pon.getProduct(4);
    pon.add(8);
    int gp4 = pon.getProduct(2);

    std::cout << gp1 << " "
              << gp2 << " "
              << gp3 << " "
              << gp4 << "\n";

    return 0;
}