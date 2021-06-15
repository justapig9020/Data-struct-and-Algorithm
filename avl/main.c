#include "avl.h"
#include <stdio.h>

static void tree_insert_val(struct AVL *tree, int val) {
    printf("Insert %d\n", val);
    if (!insert_val(tree, val))
        printf("Insert %d failed\n", val);
}

static void print_tree(struct AVL *tree) {
    printf("---- AVL ----\n");
    printf("Height: %u\n", tree->height);
    inorder(tree);
    printf("\n");
}

int main(void) {
    struct AVL *tree = new_tree();
    if (!tree)
        return 1;

    tree_insert_val(tree, 100);
    tree_insert_val(tree, 30);
    tree_insert_val(tree, 50);

    print_tree(tree);

    tree_insert_val(tree, 105);
    tree_insert_val(tree, 110);

    print_tree(tree);

    tree_insert_val(tree, 20);
    tree_insert_val(tree, 25);

    print_tree(tree);

    tree_insert_val(tree, 15);
    tree_insert_val(tree, 23);
    tree_insert_val(tree, 16);
    print_tree(tree);

    return 0;
}
