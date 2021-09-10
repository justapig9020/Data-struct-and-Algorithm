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

static void balance_subtree(struct Node **root) {
    if (!*root)
        return;
    struct Node *left = (*root)->left;
    struct Node *right = (*root)->right;
    int diff =
        get_height(left) - get_height(right);
    const int threshold = 1;
    if (threshold < diff) {
        /* Left subtree is heigher, do right rotation. */
        int r_hi = get_height(left->right);
        int l_hi = get_height(left->left);
        if (r_hi > l_hi)
            /* LR, Make an additional left rotation. */
            (*root)->left = left_rotation(left);
        *root = right_rotation(*root);
    } else if ((-1 * threshold) > diff) {
        /* Right subtree is heigher, left rotation. */
        int r_hi = get_height(right->right);
        int l_hi = get_height(right->left);
        if (r_hi < l_hi)
            /* RL, make an additional right rotation. */
            (*root)->right = right_rotation(right);
        *root = left_rotation(*root);
    }
    update_height(*root);
}

static void balance(struct Stack *s) {
    struct Node **ptr;
    while ((ptr = pop_stack(s))) {
        balance_subtree(ptr);
    }
}

bool insert_val(struct AVL *tree, int val) {
    if (!tree)
        return false;
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

    /* Insert the node into the tree as a normal BST. */
    struct Node **curr = &tree->root;
    while (*curr) {
        /* We will balance the tree after insertion.
         * Push all go through nodes into stack, hence we can easily
         * check the height backwardly.
         */
        if (!push_stack(&stack, curr))
            goto free_stack;
        int curr_val = (*curr)->val;
        if (curr_val > val)
            curr = &(*curr)->left;
        else
            curr = &(*curr)->right;
    }
    *curr = node;

    /* Followed the rule of AVL to balance the tree */
    balance(&stack);
    free_stack(&stack);

    tree->height = tree->root->height;
    return true;

free_stack:
    free_stack(&stack);
free_node:
    free(node);
    return false;
}

static bool remove_node(struct Node **target) {
    if ((*target)->right) {
        struct Node **ptr = &(*target)->right;
        struct Stack stack;
        if (!stack_with_capacity(&stack, (*ptr)->height + 1))
            return false;

        /* Find the smallest node in the right subtree */
        while ((*ptr)->left) {
            ptr = &(*ptr)->left;
            push_stack(&stack, ptr);
        }
        pop_stack(&stack);
        /* Pop the smallest node from stack. Due to the node will soon be swap
         * to the root of the subtree and we are not going to balance it in
         * this function.
         */

        /* Remove target from the tree. */
        struct Node *new_root = *ptr;
        struct Node *buf = *target;
        *ptr = new_root->right;
        new_root->right = (*target)->right;
        new_root->left = (*target)->left;
        *target = new_root;

        balance(&stack);
        /* Balance from parent of "new_root" to left child of right child of "target". */

        balance_subtree(&new_root->right);
        /* Since the target has been replaced by new_root, we shouldn't balance &(*target)->right*/

        balance_subtree(target);
        /* Balance the subtree which rooted by *target (the subtree is now rooted by new_root) */

        free(buf);
        free_stack(&stack);
    } else {
        struct Node *buf = *target;
        *target = (*target)->left;
        free(buf);
    }
    return true;
}

bool remove_val(struct AVL *tree, int val) {
    if (!tree)
        return false;

    struct Stack stack;
    if (!stack_with_capacity(&stack, tree->height + 1))
        return false;

    bool ret = false;
    struct Node **cur = &tree->root;

    /* Find target node as normal BST. */
    while (*cur) {
        int curr_val = (*cur)->val;
        if (curr_val > val) {
            push_stack(&stack, cur);
            cur = &(*cur)->left;
        } else if (curr_val < val) {
            push_stack(&stack, cur);
            cur = &(*cur)->right;
        } else {
            if(remove_node(cur)) {
                balance(&stack);
                ret = true;
            }
            break;
        }
    }
    free_stack(&stack);
    return ret;
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
