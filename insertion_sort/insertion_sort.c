#include <stdio.h>

int* insertion_sort(int A[], int n) {
    for (int i = 1; i < n; i++) {
        int key = A[i];
        int j = i - 1;
        while (j >= 0 && A[j] > key) {
            A[j+1] = A[j];
            j -= 1;
        }
        A[j+1] = key;
    }

    return 0;
}

int main() {
    int A[] = {10,3,4,1,2,6};
    int size = sizeof(A) / sizeof(A[0]);

    printf("Pre: [ ");
    for (int i = 0; i < size; i++) {
        printf("%d ",A[i]);
    }
    printf("]\n");

    insertion_sort(A, 6);

    printf("Post: [ ");
    for (int i = 0; i < 6; i++) {
        printf("%d ", A[i]);
    }
    printf("]\n");

    return 0;
}

// yaaaaaaaaay^^ we're so back
