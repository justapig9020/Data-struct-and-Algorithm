#include "bst.h"
#include <stdlib.h>
#include <stdio.h>
#include <sys/cdefs.h>

struct BST *new_bst() {
    struct BST *tree = malloc(sizeof(struct BST));
    if (tree) {
        tree->root = NULL;
    }
    return tree;
}

static struct Node *new_node(int val) {
    struct Node *node = malloc(sizeof(struct Node));
    if (node) {
        node->val = val;
        node->left = node->right = NULL;
    }
    return node;
}

void insert(struct BST* tree, int val) {
    struct Node *new = new_node(val);
    if (!new)
        return ;

    struct Node **curr = &tree->root;
    while (*curr) {
        int curr_val = (*curr)->val;
        if (curr_val >= val) {
            curr = &(*curr)->left;
        } else {
            curr = &(*curr)->right;
        }
    }
    *curr = new;
}

void _preorder(struct Node *node) {
    if (!node)
        return;
    printf("%d, ", node->val);
    _preorder(node->left);
    _preorder(node->right);
}

void preorder(struct BST* tree) {
    printf("Pre: ");
    _preorder(tree->root);
    printf("\n");
}

void _inorder(struct Node *node) {
    if (!node)
        return;
    _inorder(node->left);
    printf("%d, ", node->val);
    _inorder(node->right);
}

void inorder(struct BST* tree) {
    printf("In: ");
    _inorder(tree->root);
    printf("\n");
}

void _postorder(struct Node *node) {
    if (!node)
        return;
    _postorder(node->left);
    _postorder(node->right);
    printf("%d, ", node->val);
}

void postorder(struct BST* tree) {
    printf("Post: ");
    _postorder(tree->root);
    printf("\n");
}

static struct Node **find_smallest(struct Node **root) {
    struct Node **ptr = root;
    while ((*ptr)->left) {
        ptr = &(*ptr)->left;
    }
    return ptr;
}

static void free_node(struct Node *node) {
    node->left = NULL;
    node->right = NULL;
    free(node);
}

static void remove_node(struct Node **target) {
    struct Node *right, *left;
    right = (*target)->right;
    left = (*target)->left;
    if (right) {
        struct Node **sptr = find_smallest(&(*target)->right);
        struct Node *s = *sptr;
        *sptr = s->right;
        s->left = left;
        /* Due to the smallest node maybe the right child of the target node.
         * Therefore, we should make sure the right node pointer has been updated to the latest version.
         */
        s->right = (*target)->right;
        free_node(*target);
        *target = s;
    } else {
        free_node(*target);
        *target = left;
    }
}

int remove_val(struct BST *tree, int val) {
    struct Node **curr = &tree->root;
    while (*curr) {
        int curr_val = (*curr)->val;
        if (curr_val < val) {
            curr = &(*curr)->right;
        } else if (curr_val > val) {
            curr = &(*curr)->left;
        } else {
            remove_node(curr);
            return val;
        }
    }
    return -1;
}
