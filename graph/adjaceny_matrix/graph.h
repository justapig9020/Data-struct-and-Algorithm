#ifndef _GRAPH_H_
#define _GRAPH_H_

#include <iostream>
#include <vector>

class Edge {
    public:
        Edge(int u, int v);
        friend std::ostream& operator << (std::ostream& stream, const Edge& edge);
    private:
        int u;
        int v;
};

class Graph {
    public:
        Graph(int v_num);
        ~Graph();
        void new_edge(int v, int u);
        std::vector<Edge> list_edge();
        std::vector<Edge> dfs();
    private:
        bool **edge;
        int v;
        int e;
        void do_dfs(int curr, bool *visited, std::vector<Edge>& history);
};

#endif
