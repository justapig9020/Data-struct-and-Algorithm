#include "avl.h"
#include <stdlib.h>
#include <stdbool.h>

static struct Node *new_node(int val) {
    struct Node *node = malloc(sizeof(struct Node));
    if (!node)
        return NULL;
    node->height = 1;
    node->left = node->right = NULL;
    node->val = val;
    return node;
}

bool insert_node(struct AVL *tree, int val) {
    struct Node **curr = &tree->root;
    struct Node *node = new_node(val);
    if (!node)
        return false;
    while (*curr) {
        int curr_val = (*curr)->val;
        if (curr_val > val)
            curr = &(*curr)->left;
        else
            curr = &(*curr)->right;
    }
    *curr = node;
    return true;
}
