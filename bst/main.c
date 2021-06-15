#include "bst.h"
#include <stdio.h>

static void print_tree(struct BST *tree) {
    preorder(tree);
    inorder(tree);
    postorder(tree);
}

static void remove_node(struct BST *tree, int val) {
    int r = remove_val(tree, val);
    printf ("%d removed\n", r);
}
int main(void) {
    struct BST *tree = new_bst();

    insert(tree, 60);
    insert(tree, 20);
    insert(tree, 80);
    insert(tree, 10);
    insert(tree, 30);
    insert(tree, 70);
    insert(tree, 90);
    insert(tree, 13);
    insert(tree, 15);
    insert(tree, 11);

    print_tree(tree);

    remove_node(tree, 60);
    print_tree(tree);

    remove_node(tree, 20);
    print_tree(tree);

    remove_node(tree, 10);
    print_tree(tree);
    return 0;
}
