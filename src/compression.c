#include <stdint.h>

uint32_t decode_value(uint8_t line[], const uint8_t bit_offset, const uint8_t length) {
    const int start_byte = (bit_offset / 8);
    const int stop_byte = ((bit_offset + length) / 8);

    const uint8_t start_mask = !0 >> (bit_offset % 8);
    const uint8_t used_bits = bit_offset + length - (uint8_t)stop_byte * 8;
    const uint8_t stop_mask = !(!0 >> used_bits);

    //decode first bit (never needs shifting (lowest part is used))
    uint32_t decoded = (uint32_t)(line[start_byte] & start_mask);
    uint8_t bits_read = 8 - (bit_offset % 8);
    //if we have more bits
    if(length > 8) {
        //decode middle bits, no masking needed
        for (int i=0; i<stop_byte+1-start_byte; i++){
            uint8_t byte = line[start_byte+i];
            decoded |= (uint32_t)byte << (8 - (bit_offset % 8) + (uint8_t)i * 8);
            bits_read += 8;
        }
    }

    int stop_byte = div_up(bit_offset + length, 8); //starts at 0
    decoded |= ((line[stop_byte - 1] & (uint32_t)stop_mask)) << (bits_read - (8 - used_bits));
    return decoded;
}

uint8_t div_up(uint8_t x, uint8_t y) {
    return (x + y - 1) / y;
}

void encode_value(const uint32_t to_encode, uint8_t line[], const uint8_t bit_offset, const uint8_t length) {
    const uint8_t start_mask = !0 >> (bit_offset % 8);

    const int start_byte = (bit_offset / 8);
    const int stop_byte = ((bit_offset + length) / 8);

    line[start_byte] |= ((uint8_t)to_encode) & start_mask;
    uint8_t bits_written = 8 - (bit_offset % 8);

    if (length > 8) {
        //decode middle bits, no masking needed
        for (int i=start_byte+1; i<stop_byte; i++) {
            line[i] |= (uint8_t)(to_encode >> bits_written);
            bits_written += 8;
        }
    }

    const uint8_t used_bits = bit_offset + length - (uint8_t)stop_byte * 8;
    const uint8_t stop_mask = !(!0 >> used_bits);
    const int stop_byte = div_up(bit_offset + length, 8); //starts at 0
    line[stop_byte - 1] |= (uint8_t)(to_encode >> (bits_written - (8 - used_bits))) & stop_mask;
}