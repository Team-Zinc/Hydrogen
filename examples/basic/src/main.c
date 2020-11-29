#include <stdio.h>
#include <stdlib.h>

#include "util/add.h"

int main(void) {
    int a = 5;
    int b = 3;

    printf("%d + %d = %d\n", a, b, add(a, b));

    return EXIT_SUCCESS;
}