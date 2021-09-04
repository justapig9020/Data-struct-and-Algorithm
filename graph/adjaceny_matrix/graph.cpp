#include "graph.h"
#include <cstring>
#include <ostream>
#include <vector>

Edge::Edge(int u, int v) {
    this->u = u;
    this->v = v;
}

std::ostream& operator<< (std::ostream& stream, const Edge& edge) {
    stream << "( " << edge.u << ", " << edge.v << " )";
    return stream;
}

Graph::Graph(int v_num) {
    this->edge = new bool*[v_num];
    for (int i=0; i<v_num; i++)
        this->edge[i] = new bool[v_num]();
    this->v = v_num;
    this->e = 0;
}

void Graph::new_edge(int v, int u) {
    if (v < this->v && u < this->v)
        this->edge[v][u] = true;
}

std::vector<Edge> Graph::list_edge() {
    std::vector<Edge> v;
    for (int i=0; i<this->v; i++) {
        for (int j=0; j<this->v; j++) {
            if (this->edge[i][j]) {
                v.push_back(Edge(i, j));
            }
        }
    }
    return v;
}

Graph::~Graph() {
    for (int i=0; i<this->v; i++)
        delete this->edge[i];
    delete this->edge;
}

void Graph::do_dfs(int curr, bool *visited, std::vector<Edge>& history) {
    for (int i=0; i<this->v; i++) {
        if (this->edge[curr][i] && !visited[i]) {
            visited[i] = true;
            history.push_back(Edge(curr, i));
            do_dfs(i, visited, history);
        }
    }
}

std::vector<Edge> Graph::dfs() {
    std::vector<Edge> ret;
    bool *visited = new bool[this->v];
    for (int i=0; i<this->v; i++)
        visited[i] = false;
    do_dfs(0, visited, ret);
    delete[] visited;
    return ret;
}
