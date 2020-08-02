#include <stdint.h>

struct Field {
    uint8_t offset; //bits
    uint8_t length; //bits (max 32 bit variables)

    float decode_scale;
    float decode_add;
};

float decode(struct Field* self, uint8_t line[])
{
    const uint32_t int_repr = decode_value(line, self->offset, self->length);
    float decoded = (float)int_repr;

    decoded *= (float)self->decode_scale;
    decoded += (float)self->decode_add;

    return decoded;
}

void encode(struct Field* self, float numb, uint8_t line[])
{
    numb -= (float)self->decode_add;
    numb /= (float)self->decode_scale;
    //println!("scale: {}, add: {}, numb: {}", self.decode_scale, self.decode_add, numb);

    const uint32_t to_encode = (uint32_t)numb;

    encode_value(to_encode, line, self->offset, self->length);
}