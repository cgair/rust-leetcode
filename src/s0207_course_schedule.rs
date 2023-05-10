/**
 * [207] Course Schedule
 *
 * There are a total of n courses you have to take, labeled from 0 to n-1.
 *
 * Some courses may have prerequisites, for example to take course 0 you have to first take course 1, which is expressed as a pair: [0,1]
 *
 * Given the total number of courses and a list of prerequisite pairs, is it possible for you to finish all courses?
 *
 * Example 1:
 *
 *
 * Input: 2, [[1,0]]
 * Output: true
 * Explanation: There are a total of 2 courses to take.
 *              To take course 1 you should have finished course 0. So it is possible.
 *
 * Example 2:
 *
 *
 * Input: 2, [[1,0],[0,1]]
 * Output: false
 * Explanation: There are a total of 2 courses to take.
 *              To take course 1 you should have finished course 0, and to take course 0 you should
 *              also have finished course 1. So it is impossible.
 *
 *
 * Note:
 *
 * 
 * The input prerequisites is a graph represented by a list of edges, not adjacency matrices. 
 * Read more about [how a graph is represented.](https://www.khanacademy.org/computing/computer-science/algorithms/graph-representation/a/representing-graphs)
 * You may assume that there are no duplicate edges in the input prerequisites.
 *
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/course-schedule/

// submission codes start here

// 看到依赖问题, 首先想到的就是把问题转化成「有向图」这种数据结构.
// 只要图中存在环, 那就说明存在循环依赖.
// topology sort, BFS
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; num_courses as usize];
        let mut on_path = vec![false; num_courses as usize];
        let mut has_cycle = false;
        let graph = Solution::build_graph(num_courses as usize, prerequisites);
        for i in 0..num_courses as usize {
            Solution::traverse(&graph, i, &mut visited, &mut on_path, &mut has_cycle);
        }
        
        !has_cycle
    }
    fn traverse(graph: &Vec<Vec<i32>>, n: usize, visited: &mut [bool], on_path: &mut [bool], has_cycle: &mut bool) {
        if on_path[n] {
            // detect cycle
            *has_cycle = true;
            return;
        }
        if visited[n] { return; } 
        visited[n] = true;
        on_path[n] = true;
        for neighbor in &graph[n] {
            Solution::traverse(graph, *neighbor as usize, visited, on_path, has_cycle);
        }
        on_path[n] = false;
    }

    // 以刷题的经验常见的存储方式是使用邻接表
    fn build_graph(n: usize, prerequisites: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![Vec::new();n]; // 
        for v in prerequisites.iter() {
            graph[v[0] as usize].push(v[1]);
        }

        graph
    }
}


#[test]
fn test_207() {
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    assert_eq!(
        Solution::can_finish(
            8,
            vec![
                vec![1, 0],
                vec![2, 6],
                vec![1, 7],
                vec![6, 4],
                vec![7, 0],
                vec![0, 5]
            ]
        ),
        true
    );

    assert_eq!(
        Solution::can_finish(
            20,
            vec![
                vec![0,10],
                vec![3,18],
                vec![5,5],
                vec![6,11],
                vec![11,14],
                vec![13,1],
                vec![15,1],
                vec![17,4]
            ]    
        ),
        false
    );
}