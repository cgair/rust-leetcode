// 图的存储
// 
#include <vector>
#include <iostream>

#define N 10

// 1. 邻接矩阵 graph[N][N], N 为节点总数
// 能表示有向图和无向图
// 无向图: 边 i - j 的权值 = graph[i][j] = graph[j][i]
// 有向图: 边 i -> j 的权值 = graph[i][j]; 
//        边 j -> i 的权值 = graph[j][i].
// 表示稀疏图时十分浪费空间

// 2. 邻接表
struct edge {
    int from, to, weight;
    edge(int _from, int _to, int _weight = 0) {
        from = _from; to = _to; weight = _weight;
    }
};

int main()
{
    std::vector<edge> graph[N];
    for (int i = 0; i < 10; ++i) {
        graph[i].clear();
    }
    // 存边
    graph[0].push_back(edge(0, 1));

    // 遍历节点 0 的所有邻居
    for (int i = 0; i < graph[0].size(); ++i) {
        std::cout << graph[0][i].from
                  << " -> "
                  << graph[0][i].to
                  << std::endl;
    }

    std::vector<int> graph_simple[N];

    return 0;
}