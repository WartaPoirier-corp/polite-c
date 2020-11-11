#include <stdio.h>

typedef struct {
    int inner[256];
    int size;
} list;

int control_flow(int until) {
    int sum = 0;

    if (until < -1) {
        return -1;
    }

    for (int i = 0; i < until; i++) {
        sum += i;
        if (sum == 42) {
            printf("Sum is 42, no further addition needed");
            break;
        }
    }

    return sum;
}

/**
 * @param self [move] List to extract the inner from
 * @param dest [mutable] Array where inner should be copied
 */
void list_into_inner(list self, int* dest) {
    // Not implemented
}

int main() {
    list list = { .inner = { 1, 2, 3 }, .size = 3 };

    int dest1[256];
    list_into_inner(list, dest1);

    int dest2[256];
    list_into_inner(list, dest2);
}
