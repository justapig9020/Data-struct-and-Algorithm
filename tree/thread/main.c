#include "thread.h"

int main(void) {
    struct thread_tree *tree = new_tree();
    print_preorder(tree);
    print_inorder(tree);
    insert_value(tree, 10);
    insert_value(tree, 5);
    insert_value(tree, 20);
    insert_value(tree, 30);
    insert_value(tree, 1);
    insert_value(tree, 6);
    insert_value(tree, 15);
    print_preorder(tree);
    print_inorder(tree);
    free_tree(tree);
    return 0;
}
