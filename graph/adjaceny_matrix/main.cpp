#include "graph.h"
#include <vector>

using namespace std;
int main(void) {
    Graph g = Graph(10);
    g.new_edge(0, 1);
    g.new_edge(1, 2);
    g.new_edge(1, 5);
    g.new_edge(5, 7);
    g.new_edge(5, 8);
    g.new_edge(7, 8);
    vector<Edge> edges = g.dfs();
    vector<Edge>::iterator ptr;
    for (ptr = edges.begin(); ptr != edges.end(); ptr++)
        cout << *ptr << endl;
    return 0;
}
