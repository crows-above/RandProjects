#include <stdio.h>
#include <stdlib.h>

int* range(int k, int n) {
    int i;
    int* a = (int*)malloc((n-k) * sizeof(int));
    for (i=k;i<n;i++) {
        a[i-k] = i;
    }
    return a;
}

int main() {
    int k = 2;
    int n = 10;
    int* meow = range(k,n);
    printf("Here are the the the arrayy elements^^\n[");

    for (int i = 0; i < (n-k); i++) {
        printf(" %d ", meow[i]);
    }
    printf("]\n");

    free(meow);

    return 0;
}

// We're so back^^
