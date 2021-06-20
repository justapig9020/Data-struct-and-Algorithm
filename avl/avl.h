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
};

bool insert_val(struct AVL *tree, int val);
bool remove_val(struct AVL *tree, int val);
struct AVL *new_tree();
void inorder(struct AVL *tree);
void del_tree(struct AVL *tree);

#endif
