#include "avl.h"
#include <stdlib.h>
#include <stdbool.h>
#include <printf.h>

struct Stack {
    struct Node ***buf;
    uint32_t len;
    uint32_t capacity;
};

static bool stack_with_capacity(struct Stack *s, uint32_t c) {
    if (!s)
        return NULL;
    s->buf = malloc(sizeof(struct Node ***) * c);
    if (!s->buf) {
        free(s);
        return NULL;
    }
    s->len = 0;
    s->capacity = c;
    return s;
}

static bool push_stack(struct Stack *s, struct Node **ptr) {
    uint32_t len = s->len;
    if (len == s->capacity)
        return false;
    s->buf[len] = ptr;
    s->len += 1;
    return true;
}

static struct Node **pop_stack(struct Stack *s) {
    if (0 == s->len)
        return NULL;

    s->len -= 1;
    struct Node **buf = s->buf[s->len];
    s->buf[s->len] = NULL;
    return buf;
}

static void free_stack(struct Stack *s) {
    free(s->buf);
}

static struct Node *new_node(int val) {
    struct Node *node = malloc(sizeof(struct Node));
    if (!node)
        return NULL;
    node->height = 0;
    node->left = node->right = NULL;
    node->val = val;
    return node;
}

static void print_node(struct Node *node) {
    printf("==== Node @ %p ====\n", node);
    printf("Val: %d\n", node->val);
    printf("Height: %u\n", node->height);
    printf("left: %p\tright: %p\n", node->left, node->right);
    printf("\n");
}

static uint32_t get_height(struct Node *node) {
    if (node)
        return node->height;
    else
        return -1;
}

static void update_height(struct Node *node) {
    int r_hi = get_height(node->right);
    int l_hi = get_height(node->left);
    if (r_hi > l_hi)
        node->height = r_hi + 1;
    else
        node->height = l_hi + 1;
}

static struct Node *left_rotation(struct Node *root) {
    struct Node *new_root = root->right;
    root->right = new_root->left;
    new_root->left = root;
    update_height(new_root->left);
    update_height(new_root);
    return new_root;
}

static struct Node *right_rotation(struct Node *root) {
    struct Node *new_root = root->left;
    root->left = new_root->right;
    new_root->right = root;
    update_height(new_root->right);
    update_height(new_root);
    return new_root;
}

static void balence(struct Stack *s) {
    struct Node **ptr;
    while ((ptr = pop_stack(s))) {
        struct Node *left = (*ptr)->left;
        struct Node *right = (*ptr)->right;
        int diff =
            get_height(left) - get_height(right);
        if (1 < diff) {
            // right rotation
            int r_hi = get_height(left->right);
            int l_hi = get_height(left->left);
            if (r_hi > l_hi)
                // lr, make an additional left rotation
                (*ptr)->left = left_rotation(left);
            *ptr = right_rotation(*ptr);
        } else if (-1 > diff) {
            // left rotation
            int r_hi = get_height(right->right);
            int l_hi = get_height(right->left);
            if (r_hi < l_hi)
                // rl, make an additional right rotation
                (*ptr)->right = right_rotation(right);
            *ptr = left_rotation(*ptr);
        }
        update_height(*ptr);
    }
}

bool insert_val(struct AVL *tree, int val) {
    if (!tree)
        return false;
    struct Node **curr = &tree->root;
    struct Node *node = new_node(val);
    if (!node)
        return false;
    /* The height of the tree is start from 0,
     * therefore the buffer size
     * (the possible amount of node from root to leaf)
     * should be the height of the tree + 1.
     */
    struct Stack stack;
    if (!stack_with_capacity(&stack, tree->height + 1))
        goto free_node;

    while (*curr) {
        if (!push_stack(&stack, curr))
            goto free_stack;
        int curr_val = (*curr)->val;
        if (curr_val > val)
            curr = &(*curr)->left;
        else
            curr = &(*curr)->right;
    }
    *curr = node;
    balence(&stack);
    free_stack(&stack);

    tree->height = tree->root->height;
    return true;

free_stack:
    free_stack(&stack);
free_node:
    free(node);
    return false;
}

static void _inorder(struct Node *node) {
    if (!node)
        return;
    _inorder(node->left);
    print_node(node);
    _inorder(node->right);
}

void inorder(struct AVL *tree) {
    printf("Inorder: \n");
    _inorder(tree->root);
}

struct AVL *new_tree() {
    struct AVL *tree = malloc(sizeof(struct AVL));
    if (!tree)
        return NULL;
    tree->root = NULL;
    tree->height = 0;
    return tree;
}

static void del_subtree(struct Node *root) {
    if (!root)
        return;
    del_subtree(root->left);
    del_subtree(root->right);
    free(root);
}

void del_tree(struct AVL *tree) {
    del_subtree(tree->root);
    free(tree);
}
