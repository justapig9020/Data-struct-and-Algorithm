#include "avl.h"
#include <stdio.h>

static void tree_remove_val(struct AVL *tree, int val) {
    printf("Remove %d\n", val);
    if (!remove_val(tree, val))
        printf("Remove %d failed\n", val);
}

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

    tree_insert_val(tree, 105);
    tree_insert_val(tree, 110);

    tree_insert_val(tree, 20);
    tree_insert_val(tree, 25);

    tree_insert_val(tree, 15);
    tree_insert_val(tree, 23);
    tree_insert_val(tree, 16);
    tree_insert_val(tree, 14);
    tree_insert_val(tree, 13);

    tree_remove_val(tree, 105);
    tree_remove_val(tree, 25);
    tree_remove_val(tree, 23);
    print_tree(tree);

    del_tree(tree);

    return 0;
}
