#include "sort.h"
#include <stdio.h>

static void swap(int *a, int *b) {
    *a ^= *b;
    *b ^= *a;
    *a ^= *b;
}

void shell_sort(int arr[], size_t n)
{
    int gap = n / 2;
    while (gap > 0) {
        for (int i=gap; i<n; i++) {
            int curr = i;
            int last = curr - gap;
            while (0 <= last &&
                    arr[curr] < arr[last]) {
                swap(&arr[curr], &arr[last]);
                curr = last;
                last = curr - gap;
            }
        }
        gap /= 2;
    }
}
