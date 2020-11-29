#include <stdio.h>
#include <stdlib.h>

extern void test(void);

int main(void) {
    printf("Hello, world!\n");
    test();

    return EXIT_SUCCESS;
}
