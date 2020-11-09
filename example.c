typedef struct {
    int inner[256];
    int size;
} list;

/**
 * @param self [move] List to extract the inner from
 * @param dest [mutable] Array where inner should be copied
 */
void list_into_inner(list self, int* dest) {
    // Not implemented
}

int main() {
    list list = { { 1, 2, 3 }, 3 };

    int dest1[256];
    list_into_inner(list, dest1);

    int dest2[256];
    list_into_inner(list, dest2);
}
