#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <limits.h>
#include <assert.h>

#include "encoding.h"

void binprintf_32(uint32_t v) {
    uint32_t mask = 1 << sizeof(uint32_t)*CHAR_BIT - 1;    
    while(mask) {
        printf("%d", (v&mask ? 1 : 0));
        mask >>= 1;
    }
}

void binprintf_8(uint8_t v) {
    uint32_t mask = 1 << sizeof(uint8_t)*CHAR_BIT - 1;    
    while(mask) {
        printf("%d", (v&mask ? 1 : 0));
        mask >>= 1;
    }
}

void print_array_binairy(const uint8_t array[], const int length){
    printf("binairy array: ");
    for (int i = 0; i < length; i++) { 
        binprintf_8(array[i]);
    }
    printf("\n");
}

void encode_and_decode_multiple_edge_case() {
    uint8_t line[] = {0, 0, 0, 0, 0, 0, 0, 0};
    
    encode_value(1, line, 0, 8);
    print_array_binairy(line, sizeof(line));

    encode_value(2, line, 8, 8);
    print_array_binairy(line, sizeof(line));

    uint32_t decoded1 = decode_value(line, 0, 8);
    uint32_t decoded2 = decode_value(line, 8, 8);

    printf("0-10 %d ", decoded1); binprintf_32(decoded1); printf("\n");
    printf("0-20 %d ", decoded2); binprintf_32(decoded2); printf("\n");

    assert(decoded1 == 1);
    assert(decoded2 == 2);
}


int main(){
    printf("encode_and_decode_multiple_edge_case\n");
    encode_and_decode_multiple_edge_case();
}