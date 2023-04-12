#![allow(non_snake_case)]
// **对于面试的话其实掌握01背包和完全背包就够用了**
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

// Why ?
// 每一件物品其实只有两个状态: 取或者不取. 
// 所以可以使用回溯法搜索出所有的情况, 那么时间复杂度就是O(2^n), 其中 n 表示物品数量.
// 所以暴力的解法是指数级别的时间复杂度, 进而才需要动态规划的解法来进行优化!
pub fn knapsack_violence(W: u32, weight: &[u32], value: &[u32]) -> u32 {
    let mut knapsack = Vec::new();
    let mut ret = 0;
    backtrack(W, weight, value, &mut knapsack, 0, &mut ret);
    ret
}

fn backtrack(W: u32, weight: &[u32], value: &[u32], knapsack: &mut Vec<(u32, u32)>, start: usize, max_value: &mut u32) {
    if start == weight.len() {
        let curr_value: u32 = knapsack.iter().map(|(_w, v)| v).sum();
        if curr_value > *max_value { 
            *max_value = curr_value;
            return;
        }
    }
    
    for idx in start..weight.len() {
        let curr_weight: u32 = knapsack.iter().map(|(w, _v)| w).sum();
        // println!("{:?}", knapsack);
        if curr_weight + weight[idx] <= W {
            knapsack.push((weight[idx], value[idx]));
            backtrack(W, weight, value, knapsack, idx + 1, max_value);
            knapsack.pop().unwrap();
        }
        // 在没有放入第 i 个物品的情况下遍历子树
        backtrack(W, weight, value, knapsack, idx + 1, max_value);
    }
}

// How ?
// 明确 base case -> 明确「状态」-> 明确「选择」 -> 定义 dp 数组/函数的含义
// 状态有两个:「背包的容量」和「可选择的物品」
// 选择就是「装进背包」或者「不装进背包」
// 
// 动态规划五部曲:
// 1. 确定dp数组以及下标的含义
// 2. 确定递推公式
// 3. dp数组如何初始化
// 4. 确定遍历顺序
// 5. 举例推导dp数组
// 
// 1. 明确 dp 数组的定义:「状态」有两个, 也就是说我们需要一个二维 dp 数组. 
//    dp[i][j] 的定义: 对于前 i 个物品, 当前背包的容量为 j, 这种情况下可以装的最大价值是 dp[i][j].
// 2. 有两个方向推出来dp[i][j]:
//    由 dp[i - 1][j] 推出, 背包容量为j, 里面不放物品 i 的最大价值, 此时 dp[i][j] = dp[i - 1][j]
//    由 dp[i - 1][j - weight[i]] 推出, dp[i - 1][j - weight[i]] 为背包容量为 j - weight[i] 的时候不放物品 i 的最大价值, 那么 dp[i - 1][j - weight[i]] + value[i] (物品i的价值), 就是背包放物品 i 得到的最大价值
//    所以递归公式: dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - weight[i]] + value[i]);
// 3. base case 就是 dp[0][..] = dp[..][0] = 0, 因为没有物品或者背包没有空间的时候, 能装的最大价值就是 0.
//
use std::cmp;

fn knapsack(N: usize, W: usize, weight: &[u32], value: &[u32]) -> u32 {
    assert!(N == weight.len());
    
    // base case
    let mut dp_table = vec![vec![0; W + 1]; N + 1];
    
    for i in 1..=N { // 遍历物品
        for w in  1..=W { // 遍历背包容量
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
    // println!("{dp_table:?}");
    
    dp_table[N][W]
}

// //////////// //
// 空间复杂度优化 //
// /////////// //
// 上述问题的空间复杂度为 O(NW), 时间复杂度已经达到了最优.
// 
// 对于状态转移方程, 考虑到每个 dp[i][j] 都仅和 dp[i-1][j] 或 dp[i-1][j-wi]有关. (第 i 个物品的状态往往只依赖于第 i-1 个物品的状态, 那么我们可以很轻松地想到只使用两行数组存储两个时刻的状态)
// 可以通过利用滚动数组的方式, 将 dp[N][W] 所需要空间压缩至 dp[2][W].
// 
// 举个例子, 我们将 dp[0][j] 状态存在二维表的第 0 行, 
// 接着推出 dp[1][j]对应的值, 并将 dp[1][j] 的状态存放于二维表的第 1 行.
// 再接着推出 dp[2][j]的状态时, dp[0][j] 的状态已经没有保存意义了, 因此直接将 dp[2][j] 的状态保存到二维表的第 0 行.
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


// 理解滚动数组优化空间的基础之上, 可以将长度 2 * W 的二维表进一步压缩成一个一维数组,
// 在使用二维数组的时候, 递推公式: 
//        dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - weight[i]] + value[i]);
// 发现如果把 dp[i - 1] 那一层拷贝到 dp[i]上, 表达式完全可以是: (line 67)
//        dp[i][j] = max(dp[i][j], dp[i][j - weight[i]] + value[i]);
// 
// 于其把 dp[i - 1] 这一层拷贝到 dp[i]上, 不如只用一个一维数组了, 只用dp[j]. 
// 1. 确定dp数组的定义:     dp[j] 为(当前)容量为 j 的背包所背的最大价值
// 2. 递推公式:            那么如何推导 dp[j] 呢? dp[j]有两个选择: 一个是取自己 dp[j], 一个是取 dp[j - weight[i]] + value[i], 取最大的: dp[j] = max(dp[j], dp[j - weight[i]] + value[i]);
// 3. dp 数组如何初始化:    dp[j] 表示容量为 j 的背包, 所背的物品价值可以最大为 dp[j], 那么dp[0]就应该是 0 (背包容量为 0 所背的物品的最大价值就是 0）
// 4. 一维 dp 数组遍历顺序: 二维 dp 遍历的时候, 背包容量是从小到大; 而一维dp遍历的时候, 背包是从大到小.

// 倒叙遍历是为了保证物品 i 只被放入一次!
// 举一个例子: 物品0的重量 weight[0] = 1, 价值value[0] = 15
// 如果正序遍历
// dp[1] = dp[1 - weight[0]] + value[0] = 15
// dp[2] = dp[2 - weight[0]] + value[0] = 30 此时 dp[2] 就已经是 30 了, 意味着物品 0 被放入了两次, 所以不能正序遍历.
// 
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

use std::time::Instant;

fn main() {
    let start = Instant::now();
    assert_eq!(
        knapsack_violence(4, &[2, 1, 3], &[4, 2, 3]),
        6
    );
    println!("          Backtrack: {:?}", start.elapsed());
    
    let start = Instant::now();
    assert_eq!(
        knapsack(3, 4, &[2, 1, 3], &[4, 2, 3]),
        6
    );
    println!("Dynamic Programming: {:?}", start.elapsed());

    assert_eq!(
        knapsack_space_opt(4, &[2, 1, 3], &[4, 2, 3]),
        6
    );
    assert_eq!(
        knapsack_space_opt2(4, &[2, 1, 3], &[4, 2, 3]),
        6
    );
}