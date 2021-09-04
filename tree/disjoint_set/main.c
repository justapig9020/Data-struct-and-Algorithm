#include "disjoint_set.h"
#include <stdio.h>

int main(void) {
    struct Disjoint *set = new_set(10);
    union_sets(set, 1, 5);
    union_sets(set, 1, 9);
    union_sets(set, 8, 5);
    union_sets(set, 7, 6);
    union_sets(set, 7, 3);
    union_sets(set, 2, 7);
    print_set(set);
    return 0;
}
