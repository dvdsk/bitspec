#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <limits.h>
#include <assert.h>
#include <float.h>

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
        printf(" ");
    }
    printf("\n");
}

void encode_and_decode_multiple_edge_case() {
    uint8_t line[] = {0, 0, 0, 0, 0, 0, 0, 0};
    
    encode_value(1, line, 0, 8);
    //print_array_binairy(line, sizeof(line));
    encode_value(2, line, 8, 8);
    //print_array_binairy(line, sizeof(line));

    uint32_t decoded1 = decode_value(line, 0, 8);
    uint32_t decoded2 = decode_value(line, 8, 8);

    //printf("0-10 %d ", decoded1); binprintf_32(decoded1); printf("\n");
    //printf("0-20 %d ", decoded2); binprintf_32(decoded2); printf("\n");

    assert(decoded1 == 1);
    assert(decoded2 == 2);
}

void encode_and_decode_600() {
    uint8_t line[] = {0, 0, 0, 0, 0, 0, 0, 0};
    encode_value(600, line, 14, 10);
    //print_array_binairy(line, sizeof(line));

    uint32_t decoded1 = decode_value(line, 14, 10);
    //printf("0-10 %d ", decoded1); binprintf_32(decoded1); printf("\n");

    assert(decoded1 == 600);
}

void field_encode_decode() {
    const union Field fields[] = {
        {.F32 = { // Sine
            .decode_add = -5000,
            .decode_scale = 1,
            .length = 14,
            .offset = 0},
        },
        {.F32 = { // Triangle
            .decode_add = -10,
            .decode_scale = 0.05,
            .length = 10,
            .offset = 14},
        },
        {.Bool = { // Boolean
            .offset = 24},
        }
    };

    /*for (int i=0; i<100; i++){
        float sine = -5000.0 + (float)i*(5000.0*2.0)/100.0;
        float triangle = 20.0 - ((float)i)*(20.0+10.0)/100.0;

        uint8_t line[3] = {0, 0, 0};
        encode(&fields[0], sine, line);
        encode(&fields[1], triangle, line);

        float decoded_sine = decode(&fields[0], line);
        float decoded_triangle = decode(&fields[1], line);

        assert(sine-decoded_sine <= 1+0.001);
        assert(triangle-decoded_triangle <= 0.05+0.001 );
    }*/

    uint8_t line[4] = {0, 0, 0, 0};
    encode_bool(&fields[2], true, line);

    encode_f32(&fields[1], 2.81, line);
    printf("%#04x %#04x %#04x %#04x\n",line[0],line[1],line[2],line[3]);
    float decoded = decode_f32(&fields[1], line);
    printf("%.2f \n", decoded);
}

int main(){
    /* printf("encode_and_decode_multiple_edge_case\n"); */
    /* encode_and_decode_multiple_edge_case(); */

    /* printf("encode_and_decode_600\n"); */
    /* encode_and_decode_600(); */

    printf("field_encode_decode\n");    
    field_encode_decode();
}
