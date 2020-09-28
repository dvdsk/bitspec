#include <stdint.h>
#include <stdio.h> //dev only
#include "encoding.h"

static uint8_t div_up(uint8_t x, uint8_t y) {
    return (x + y - 1) / y;
}

uint32_t decode_value(uint8_t line[], const uint8_t bit_offset, const uint8_t length) {
    const int start_byte = (bit_offset / 8);
    const int stop_byte = ((bit_offset + length) / 8);

    const uint8_t start_mask = (uint8_t)~0 >> (bit_offset % 8);
    const uint8_t used_bits = bit_offset + length - (uint8_t)stop_byte * 8;
    const uint8_t stop_mask = ~((uint8_t)~0 >> used_bits);

    //decode first bit (never needs shifting (lowest part is used))
    uint32_t decoded = (uint32_t)(line[start_byte] & start_mask);
    uint8_t bits_read = 8 - (bit_offset % 8);

    //if we have more bits
    if(length > 8) {
        //decode middle bits, no masking needed
        for (int i=start_byte+1; i<stop_byte; i++){
            uint8_t byte = line[i];
            decoded |= (uint32_t)byte << bits_read;
            bits_read += 8;
        }
    }

    int stop_byte_upper = div_up(bit_offset + length, 8) - 1; //starts at 0 
    decoded |= ((line[stop_byte_upper] & (uint32_t)stop_mask)) << (bits_read - (8 - used_bits));
    return decoded;
}

///MUST get a zerod line.
void encode_value(const uint32_t to_encode, uint8_t line[], const uint8_t bit_offset, const uint8_t length) {
    //~ is the cpp bitwise NOT, when applied to any type it will turn it into
    //a int, even if int is not equal to uint8_t. thus we cast directly after
    //applying ~
    const uint8_t start_mask = (uint8_t)~0 >> (bit_offset % 8);

    const int start_byte = (bit_offset / 8);
    const int stop_byte = ((bit_offset + length) / 8);

    line[start_byte] |= (to_encode) & start_mask;
    uint8_t bits_written = 8 - (bit_offset % 8);

    if (length > 8) {
        //decode middle bits, no masking needed
        for (int i=start_byte+1; i<stop_byte; i++) {
            line[i] |= (uint8_t)(to_encode >> bits_written);
            bits_written += 8;
        }
    }

    const uint8_t used_bits = bit_offset + length - (uint8_t)stop_byte * 8;
    const uint8_t stop_mask = ~((uint8_t)~0 >> used_bits);
    const int stop_byte_upper = div_up(bit_offset + length, 8); //starts at 0
    line[stop_byte_upper - 1] |= (uint8_t)(to_encode >> (bits_written - (8 - used_bits))) & stop_mask;
}

float decode(const struct Field* self, uint8_t line[])
{
    const uint32_t int_repr = decode_value(line, self->offset, self->length);
    printf("int repr decoding: %u \n", int_repr);
    float decoded = (float)int_repr;

    decoded *= self->decode_scale;
    decoded += self->decode_add;

    return decoded;
}

void encode(const struct Field* self, float numb, uint8_t line[])
{
    numb -= (float)self->decode_add;
    numb /= (float)self->decode_scale;

    const uint32_t to_encode = (uint32_t)numb;
    printf("int repr encoding: %u \n", to_encode);
    encode_value(to_encode, line, self->offset, self->length);
}

/*const uint8_t byte_length(const struct Field field_list[]){
    uint8_t last = sizeof(field_list)/sizeof(struct Field) -1;
    uint16_t bits = field_list[last].offset+field_list[last].length;
    uint8_t bytes = (bits - 1) / 8 + 1;
    return bytes;
}*/