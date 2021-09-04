#ifndef _DISJOINT_SET_
#define _DISJOINT_SET_

struct Disjoint;

struct Disjoint *new_set(int n);

void union_sets(struct Disjoint *set, int a, int b);

int find_partition(struct Disjoint *set, int item);

void print_set(struct Disjoint *set);

#endif
