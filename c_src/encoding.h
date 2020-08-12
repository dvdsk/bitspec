#include <stdint.h>

struct Field {
    uint8_t offset; //bits
    uint8_t length; //bits (max 32 bit variables)

    float decode_scale;
    float decode_add;
};

uint32_t decode_value(uint8_t line[], const uint8_t bit_offset, const uint8_t length);
void encode_value(const uint32_t to_encode, uint8_t line[], const uint8_t bit_offset, const uint8_t length);

float decode(const struct Field* self, uint8_t line[]);
void encode(const struct Field* self, float numb, uint8_t line[]);
const uint8_t byte_length(const struct Field field_list[]);