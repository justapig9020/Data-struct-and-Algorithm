#include "thread.h"
#include <stdbool.h>
#include <stdlib.h>
#include <sys/resource.h>
#include <stdio.h>

#define HAS_CHILD(thread) (thread.is_thread == false)
#define CHILD_OF(thread) (thread.child)
#define THREAD_UPDATE(thread, ptr, type) do { \
    thread.child = ptr; \
    thread.is_thread = type; \
} while(0)

#define TYPE_THREAD true
#define TYPE_CHILD false

typedef void (*node_op_t)(struct thread_node *);

struct thread {
    bool is_thread;
    struct  thread_node *child;
};

struct thread_node {
    int value;
    struct thread right;
    struct thread left;
};

static struct thread default_thread() {
    struct thread def = {
        .is_thread = true,
        .child = NULL,
    };
    return def;
}

static struct thread_node *new_node(int value) {
    struct thread_node *node = malloc(sizeof(*node));
    node->value = value;
    node->left = default_thread();
    node->right = default_thread();
    return node;
}

static struct thread_node *leftest(struct thread_node *root) {
    if (!root)
        return NULL;
    struct thread_node *ptr = root;
    while (HAS_CHILD(ptr->left))
        ptr = CHILD_OF(ptr->left);
    return ptr;
}

static struct thread_node *inorder_next(struct thread_node *node) {
    if (!node)
        return NULL;
    if (!HAS_CHILD(node->right))
        // right thread is pointing to its inorder successor
        return CHILD_OF(node->right);
    struct thread_node *subroot = CHILD_OF(node->right);
    return leftest(subroot);
}

static void inorder_do(struct thread_node *root, node_op_t node_do) {
    struct thread_node *curr = leftest(root);
    while (curr) {
        struct thread_node *next = inorder_next(curr);
        node_do(curr);
        curr = next;
    }
}

static struct thread_node* preorder_next_wrong(struct thread_node *node) {
    if (!node)
        return NULL;

    struct thread_node *curr = node;
    if (HAS_CHILD(curr->left))
        return CHILD_OF(curr->left);
    if (HAS_CHILD(curr->right))
        return CHILD_OF(curr->right);

    curr = CHILD_OF(curr->right);
    while (!HAS_CHILD(curr->right))
        curr = CHILD_OF(curr->right);

    return CHILD_OF(curr->right);
}

static struct thread_node* preorder_next(struct thread_node *node) {
    if (!node)
        return NULL;

    struct thread_node *curr = node;
    if (HAS_CHILD(curr->left))
        return CHILD_OF(curr->left);
    if (HAS_CHILD(curr->right))
        return CHILD_OF(curr->right);

    while (!HAS_CHILD(curr->right) && CHILD_OF(curr->right))
        curr = CHILD_OF(curr->right);

    return CHILD_OF(curr->right);
}

static void preorder_do(struct thread_node *root, node_op_t node_do) {
    struct thread_node *curr = root;
    while (curr) {
    #ifdef WRONG
        struct thread_node *next = preorder_next_wrong(curr);
    #else
        struct thread_node *next = preorder_next(curr);
    #endif
        node_do(curr);
        curr = next;
    }
}

static void free_node(struct thread_node *node) {
    free(node);
}

static void free_subtree(struct thread_node *root) {
    inorder_do(root, free_node);
}

struct thread_tree *new_tree() {
    struct thread_tree *tree = malloc(sizeof(*tree));
    return tree;
}

void free_tree(struct thread_tree *tree) {
    free_subtree(tree->root);
    free(tree);
}

bool insert_value(struct thread_tree *tree, int val) {
    if (!tree)
        return false;
    struct thread_node *curr = tree->root;
    struct thread_node *new = new_node(val);
    if (!new)
        return false;

    if (!curr) {
        tree->root = new;
        return true;
    }
    while (1) {
        if (curr->value > val) {
            // go left
            if (HAS_CHILD(curr->left)) {
                curr = CHILD_OF(curr->left);
            } else {
                CHILD_OF(new->right) = curr;
                CHILD_OF(new->left) = CHILD_OF(curr->left);
                THREAD_UPDATE(curr->left, new, TYPE_CHILD);
                break;
            }
        } else {
            // go right
            if (HAS_CHILD(curr->right)) {
                curr = CHILD_OF(curr->right);
            } else {
                CHILD_OF(new->left) = curr;
                CHILD_OF(new->right) = CHILD_OF(curr->right);
                THREAD_UPDATE(curr->right, new, TYPE_CHILD);
                break;
            }
        }
    }
    return true;
}

static void print_value(struct thread_node *node) {
    printf("%d, ", node->value);
}

void print_inorder(struct thread_tree *tree) {
    printf("Inorder: ");
    inorder_do(tree->root, print_value);
    puts("");
}

void print_preorder(struct thread_tree *tree) {
    printf("Preorder: ");
    preorder_do(tree->root, print_value);
    puts("");
}
