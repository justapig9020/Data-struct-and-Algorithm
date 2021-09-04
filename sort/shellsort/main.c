#include <stdio.h>
#include "sort.h"
#include "vector.h"

#define DATA "numbers.txt"

static bool is_sorted(int arr[], size_t n)
{
    for (int i=1; i<n; i++) {
        if (arr[i-1] > arr[i])
            return false;
    }
    return true;
}

static void print_array(int arr[], size_t n) {
    for (int i=0; i<n; i++)
        printf("%d, ", arr[i]);
    printf("\n");
}

int main(void)
{
    FILE *f = fopen(DATA, "r");
    if (!f) {
        perror("Error: ");
        return 1;
    }
    vec_t vec = new_vec(sizeof(int));
    int val;
    while (fscanf(f, "%d", &val) != EOF) {
        push(vec, &val);
    }
    fclose(f);

    size_t n;
    int *arr = into_array(vec, &n);
    printf("Before: ");
    print_array(arr, n);

    shell_sort(arr, n);

    printf("After: ");
    print_array(arr, n);

    char *msg = is_sorted(arr, n)? "PASS" : "FAIL";
    printf("Sort: %s\n", msg);
    free(arr);

    return 0;
}
