#ifndef _AVL_H_
#define _AVL_H_

#include <stdint.h>
#include <stdbool.h>
#include <sys/cdefs.h>

struct Node {
    int val;
    uint32_t height;
    struct Node *left;
    struct Node *right;
};

struct AVL {
    struct Node *root;
    uint32_t height;
    // Rocord the height of the tree.
    // Therefore, we can construct stack of tracing
    // backword at begining of insert/remove with
    // enough capacity.
};

bool insert_val(struct AVL *tree, int val);
bool remove_val(struct AVL *tree, int val);
struct AVL *new_tree();
void inorder(struct AVL *tree);
void del_tree(struct AVL *tree);

#endif
