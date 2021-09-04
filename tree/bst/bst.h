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
void preorder(struct BST *tree);
void inorder(struct BST *tree);
void postorder(struct BST *tree);
int remove_val(struct BST *tree, int val);

#endif
