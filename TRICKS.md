# Some Tricks

## 类型转换
see http://shouce.jb51.net/rust-book-chinese/content/Casting%20Between%20Types%20%e7%b1%bb%e5%9e%8b%e8%bd%ac%e6%8d%a2.html

## 迭代器
### 为什么要使用迭代器？
```rust
    let vec = vec![1, 2, 3, 4, 5];

    // 额外的边界检查开销
    for i in 0..5 {
        eprint!("{}", vec[i]);  // Rust 的 Vec 类型下标访问每次都会做边界检查（越界会panic）
                                // 而循环本身也会做一次边界检查（0 到 5 每次循环都要检查）
    }

    for num in vec.iter() {
        eprint!("{}", num);
    }
```