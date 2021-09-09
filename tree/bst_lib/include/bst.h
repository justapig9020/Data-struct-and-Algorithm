#ifndef _BST_H_
#define _BST_H_

struct Node {
    int val;
    struct Node *left;
    struct Node *right;
};

struct BST {
    struct Node *root;
};

struct BST *new_bst();
void insert(struct BST *tree, int val);
void preorder(const struct BST *tree);
void inorder(const struct BST *tree);
void postorder(const struct BST *tree);
int remove_val(struct BST *tree, int val);
void free_bst(struct BST *tree);

#endif
