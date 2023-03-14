#include <stdio.h>
#include <stdint.h>

uint32_t rfunc_add(uint32_t a, uint32_t b);

int main(int argc, char ** argv) {
    printf("1, 2 = %u\n", rfunc_add(1,2));
    printf("1, 100000 = ");
    fflush(stdout);
    printf("%u\n", rfunc_add(1,100000));
}
