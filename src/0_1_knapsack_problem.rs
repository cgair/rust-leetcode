#![allow(non_snake_case)]
/**
 * 经典动态规划: [0-1背包问题](https://labuladong.github.io/algo/di-er-zhan-a01c6/bei-bao-le-34bd4/jing-dian--28f3c/)
 * 描述: 给你一个可装载重量为 W 的背包和 N 个物品, 每个物品有重量和价值两个属性.
 * 其中第 i 个物品的重量为 wt[i], 价值为 val[i].
 * 现在让你用这个背包装物品, 最多能装的价值是多少?
 * 
 * 例子:
 * 
 * 
 * 输入:
 * N = 3, W = 4
 * wt = [2, 1, 3]
 * val = [4, 2, 3]
 * 
 * 输出: 6
 * 解释: 选择前两件物品装进背包, 总重量 3 小于 W, 可以获得最大价值 6.
 * 
 */

// 明确 base case -> 明确「状态」-> 明确「选择」 -> 定义 dp 数组/函数的含义
// 状态有两个:「背包的容量」和「可选择的物品」
// 选择就是「装进背包」或者「不装进背包」
// 
// 明确 dp 数组的定义:「状态」有两个, 也就是说我们需要一个二维 dp 数组. 
// dp[i][w] 的定义: 对于前 i 个物品, 当前背包的容量为 w, 这种情况下可以装的最大价值是 dp[i][w].
// base case 就是 dp[0][..] = dp[..][0] = 0, 因为没有物品或者背包没有空间的时候, 能装的最大价值就是 0.
use std::cmp;

fn knapsack(N: usize, W: usize, weight: &[u32], value: &[u32]) -> u32 {
    assert!(N == weight.len());
    
    // base case
    let mut dp_table = vec![vec![0; W + 1]; N + 1];
    
    for y in 1..=N {
        for x in 1..=W {
            if (x as i32).checked_sub(weight[y - 1] as i32).unwrap() < 0 {
                // 不装入背包
                dp_table[y][x] = dp_table[y - 1][x];
            } else {
                dp_table[y][x] = cmp::max(
                    dp_table[y - 1][x],
                    dp_table[y - 1][x.checked_sub(weight[y - 1] as usize).unwrap()] + value[y - 1]
                );
            }
        }
    }
    
    dp_table[N][W]
}


fn main() {
    assert_eq!(
        knapsack(3, 4, &[2, 1, 3], &[4, 2, 3]),
        6
    );
}