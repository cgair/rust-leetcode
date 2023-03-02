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
    
    for i in 1..=N {
        for w in  1..=W {
            if (w as i32).checked_sub(weight[i - 1] as i32).unwrap() < 0 {
                // 这种情况下只能选择不装入背包
                dp_table[i][w] = dp_table[i - 1][w];
            } else {
                // 装入或者不装入背包, 择优.
                // 
                // 如果放入背包, 则留给前 i-1 个物品将会减小, 当前总价值与前 i -1 个物品恰放入容量 w - weight[i] 的背包
                // 的总价值加上当前物品价值 value[i]
                // Note 索引 weight[] 和 value[] 时索引要减 1
                dp_table[i][w] = cmp::max(dp_table[i - 1][w.checked_sub(weight[i - 1] as usize).unwrap()] + value[i - 1], dp_table[i - 1][w]);
                // 我们将"给定了背包容量 W 的情况下, 前 i 个物品恰放入背包的最大总价值" 这个问题转化为了只和前 i-1 个物品相关的子问题
            }
        }
    }
    
    dp_table[N][W]
}
// //////////// //
// 空间复杂度优化 //
// /////////// //
// 上述问题的空间复杂度为 O(NW), 时间复杂度已经达到了最优.

// 对于状态转移方程, 考虑到每个 dp[i][w] 都仅和 dp[i-1][w] 或 dp[i-1][w-wi]有关. (第 i 个物品的状态往往只依赖于第 i-1 个物品的状态, 那么我们可以很轻松地想到只使用两行数组存储两个时刻的状态)
// 可以通过利用滚动数组的方式, 将 dp[N][W] 所需要空间压缩至 dp[2][W].
// 
// 举个例子, 我们将 dp[0][wi] 状态存在二维表的第 0 行, 
// 接着推出 dp[1][wi]对应的值, 并将 dp[1][wi] 的状态存放于二维表的第 1 行.
// 再接着推出 dp[2][Wi]的状态时, dp[0][wi] 的状态已经没有保存意义了, 因此直接将 dp[2][wi] 的状态保存到二维表的第 0 行.
// 以此类推, 使数组空间得到循环利用.
fn knapsack_space_opt(W: usize, weight: &[u32], value: &[u32]) -> u32 {
    let n = weight.len();

    let mut dp = vec![vec![0; W + 1]; 2];
    for i in 1..=n {
        // for w in 1..=W {
        //     if (w as i32) - (weight[i - 1] as i32) < 0 {
        //             dp[i % 2][w] = if i % 2 == 0 { dp[1][w] } else { dp[0][w] };
        //         } else {
        //                 dp[i % 2][w] = if i % 2 == 0 {
        //                         cmp::max(dp[1][w], dp[1][w.checked_sub(weight[i - 1] as usize).unwrap()] + value[i - 1])
        //                     } else {
        //                             cmp::max(dp[0][w], dp[0][w.checked_sub(weight[i - 1] as usize).unwrap()] + value[i - 1])
        //                         };
        //                     }
                            
        for w in 0..=W {
            let idx = i & 1;
            if (w as i32) - (weight[i - 1] as i32) < 0 {
                dp[idx][w] = dp[idx ^ 1][w];
            } else {
                dp[idx][w] = cmp::max(dp[idx ^ 1][w], dp[idx ^ 1][w.checked_sub(weight[i - 1] as usize).unwrap()] + value[i - 1]);
            }
        }
    }

    dp[n & 1 ^ 1][W]
}


// 理解滚动数组优化空间的基础之上, 可以将长度 2 * W 的二维表进一步压缩成一个一维数组,.
// 其状态转移方程为 dp[w]= max{ dp[w], dp[w-wi]+vi }
// 
// 先计算 dp[i-1][w] 并存入数组 B 当中. 
// 然后再计算 dp[i][w] 并覆盖数组 B 中的内容, 依次类推直至所有状态求出.
// 由于 dp[w]可由 dp[i-1][w-wi]推出, 因此在覆盖数组时必须采用倒序存储.
// 
// 举个例子：物品 0 的 weight[0] = 1, value[0] = 15
// 正序遍历:
// dp[1] = dp[1 - weight[0]] + value[0] => dp[1] = 15
// dp[2] = dp[2 - weight[0]] + value[0] => dp[2] = 30
// 此时 dp[2] 就已经是30了, 物品0已经被放入了两次, 所以不能正序遍历.
// 倒序遍历:
// dp[2] = dp[2 - weight[0]] + value[0] => dp[2] = 15
// dp[1] = dp[1 - weight[0]] + value[0] => dp[1] = 15
// 所以需要从后从后向前遍历, 每次取得状态不会和之前取得状态重合, 这样每种物品就只取一次了.
// 另外, 如果遍历背包容量放在外层, 那么每个 dp[w] 只会放入一个物品, 即背包里只放入了一个物品, 所以需要背包容量的遍历要放在内层.

fn knapsack_space_opt2(W: usize, weight: &[u32], value: &[u32]) -> u32 {
    let n = weight.len();
    // dp[w] 表示：容量为 w 的背包, 所背的物品价值最大为 dp[w].
    let mut dp = vec![0; W];
    
    for i in 0..n {
        if weight[i] > W as u32 { continue; }
        for w in (weight[i] as usize..W).rev() {
            dp[w] = cmp::max(dp[w], dp[w.checked_sub(weight[i] as usize).unwrap()] + value[i]);
        }
    }

    dp[W - 1]
}


fn main() {
    assert_eq!(
        knapsack(3, 4, &[2, 1, 3], &[4, 2, 3]),
        6
    );
    assert_eq!(
        knapsack_space_opt(4, &[2, 1, 3], &[4, 2, 3]),
        6
    );
    assert_eq!(
        knapsack_space_opt2(4, &[2, 1, 3], &[4, 2, 3]),
        6
    );
}