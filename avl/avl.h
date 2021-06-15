#ifndef _AVL_H_
#define _AVL_H_

#include <stdint.h>

struct Node {
    int val;
    uint32_t height;
    struct Node *left;
    struct Node *right;
};

struct AVL {
    struct Node *root;
};

#endif
