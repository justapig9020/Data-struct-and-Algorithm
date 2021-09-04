#include "disjoint_set.h"
#include <stdlib.h>
#include <stdio.h>

struct Disjoint {
    int *node;
    int size;
};

struct Disjoint *new_set(int n) {
    struct Disjoint *ret = malloc(sizeof(struct Disjoint));
    ret->node = malloc(sizeof(int) * n);
    for (int i=0; i<n; i++)
        ret->node[i] = -1;
    ret->size = n;
    return ret;
}

static int get_height(struct Disjoint *set, int p) {
    int height = set->node[p];
    if (height < 0) {
        return abs(height);
    }
    return 0;
}

void union_sets(struct Disjoint *set, int a, int b) {
    if (a >= set->size || b >= set->size)
        return;

    int pa = find_partition(set, a);
    int pb = find_partition(set, b);
    if (pa == pb)
        return;

    int ha = get_height(set, pa);
    int hb = get_height(set, pb);
    if (ha > hb) {
        set->node[pb] = pa;
    } else if (ha < hb) {
        set->node[pa] = pb;
    } else {
        set->node[pb] = pa;
        set->node[pa] -= 1;
    }
}

int find_partition(struct Disjoint *set, int item) {
    if (item >= set->size)
        return -1;

    int ptr = item;
    while (set->node[ptr] > 0) {
        ptr = set->node[ptr];
    }
    return ptr;
}

void print_set(struct Disjoint *set) {
    for (int i=0; i<set->size; i++)
        printf("%d: %d\n", i, set->node[i]);
}
