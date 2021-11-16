#pragma once

#include <stdbool.h>

struct thread_node;
struct thread_tree {
    struct thread_node *root;
};

struct thread_tree *new_tree();

void free_tree(struct thread_tree *tree);

bool insert_value(struct thread_tree *tree, int val);

void print_inorder(struct thread_tree *tree);

void print_preorder(struct thread_tree *tree);
