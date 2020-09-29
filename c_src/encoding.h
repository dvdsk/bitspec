#include <stdint.h>
#include <stdbool.h>

struct BoolField {
    uint8_t offset; //bits
};

struct Float32Field {
    uint8_t offset; //bits
    uint8_t length; //bits (max 32 bit variables)

    float decode_scale;
    float decode_add;
};

union FieldData {
    BoolField Bool;
    Float32Field F32;
    /* Float64Field F64; */
};

enum FieldVariant {
    Bool,
    F32,
};

struct Field {
    FieldVariant variant;
    FieldData data;
};

uint32_t decode_value(uint8_t line[], const uint8_t bit_offset, const uint8_t length);
void encode_value(const uint32_t to_encode, uint8_t line[], const uint8_t bit_offset, const uint8_t length);

float decode_f32(const struct Field* self, uint8_t line[]);
void encode_f32(const struct Field* self, float numb, uint8_t line[]);
bool decode_bool(const struct Field* self, uint8_t line[]);
void encode_bool(const struct Field* self, bool numb, uint8_t line[]);
const uint8_t byte_length(const struct field_list[]);
