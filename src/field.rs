use crate::compression;
use serde_derive::{Deserialize, Serialize};

pub trait FloatMinMax {
    fn float_min(&mut self) -> f64;
    fn float_max(&mut self) -> f64;
}

impl<T> FloatMinMax for T
where
    T: Iterator<Item = f64>,
{
    fn float_max(&mut self) -> f64 {
        self.fold(f64::NAN, f64::max)
    }
    fn float_min(&mut self) -> f64 {
        self.fold(f64::NAN, f64::min)
    }
}

pub type FieldId = u8;
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetaField<T> {
    pub id: FieldId,
    pub name: String,

    pub offset: u8, //bits
    pub length: u8, //bits (max 32 bit variables)

    pub decode_scale: T,
    pub decode_add: T,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Field<T> {
    pub offset: u8, //bits
    pub length: u8, //bits (max 32 bit variables)

    pub decode_scale: T,
    pub decode_add: T,
}

impl<T> Into<Field<T>> for MetaField<T> {
    fn into(self) -> Field<T> {
        Field {
            offset: self.offset,
            length: self.length,
            decode_scale: self.decode_scale,
            decode_add: self.decode_add,
        }
    }
}

impl<T> MetaField<T>
where
    T: num::cast::NumCast
        + std::fmt::Display
        + std::ops::Add
        + std::ops::SubAssign
        + std::ops::DivAssign
        + std::ops::MulAssign
        + std::marker::Copy,
{
    pub fn decode<D>(&self, line: &[u8]) -> D
    where
        D: num::cast::NumCast
            + std::fmt::Display
            + std::ops::Add
            + std::ops::SubAssign
            + std::ops::MulAssign
            + std::ops::AddAssign,
    {
        let int_repr: u32 = compression::decode(line, self.offset, self.length);
        let mut decoded: D = num::cast(int_repr).unwrap();

        decoded *= num::cast(self.decode_scale).unwrap(); //FIXME flip decode scale / and *
        decoded += num::cast(self.decode_add).unwrap();

        decoded
    }
    #[allow(dead_code)]
    pub fn encode<D>(&self, mut numb: T, line: &mut [u8])
    where
        D: num::cast::NumCast
            + std::fmt::Display
            + std::ops::Add
            + std::ops::SubAssign
            + std::ops::AddAssign
            + std::ops::DivAssign,
    {
        numb -= num::cast(self.decode_add).unwrap();
        numb /= num::cast(self.decode_scale).unwrap();

        let to_encode: u32 = num::cast(numb).unwrap();

        compression::encode(to_encode, line, self.offset, self.length);
    }
}

#[allow(dead_code)]
impl<T> Field<T>
where
    T: num::cast::NumCast
        + std::fmt::Display
        + std::ops::Add
        + std::ops::SubAssign
        + std::ops::DivAssign
        + std::ops::MulAssign
        + std::marker::Copy,
{
    pub fn decode<D>(&self, line: &[u8]) -> D
    where
        D: num::cast::NumCast
            + std::fmt::Display
            + std::ops::Add
            + std::ops::SubAssign
            + std::ops::MulAssign
            + std::ops::AddAssign,
    {
        let int_repr: u32 = compression::decode(line, self.offset, self.length);
        let mut decoded: D = num::cast(int_repr).unwrap();

        decoded *= num::cast(self.decode_scale).unwrap(); //FIXME flip decode scale / and *
        decoded += num::cast(self.decode_add).unwrap();

        decoded
    }
    pub fn encode(&self, mut numb: T, line: &mut [u8])
    where
        T: num::cast::NumCast
            + std::fmt::Display
            + std::ops::Add
            + std::ops::SubAssign
            + std::ops::AddAssign
            + std::ops::DivAssign,
    {
        //println!("org: {}",numb);
        numb -= num::cast(self.decode_add).unwrap();
        numb /= num::cast(self.decode_scale).unwrap();
        //println!("scale: {}, add: {}, numb: {}", self.decode_scale, self.decode_add, numb);

        let to_encode: u32 = num::cast(numb).unwrap();

        compression::encode(to_encode, line, self.offset, self.length);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let fields = &[
            // Ble_reliability_testing_dataset
            Field::<f32> {
                // Sine
                decode_add: -5000.0000000000,
                decode_scale: 1.0000000000,
                length: 14,
                offset: 0,
            },
            Field::<f32> {
                // Triangle
                decode_add: -10.0000000000,
                decode_scale: 0.0500000007,
                length: 10,
                offset: 14,
            },
        ];

        for i in 0..100 {
            let sine = -5000.0 + i as f32 * (5000.0 * 2.0) / 100.0;
            let triangle = 20.0 - i as f32 * (20.0 + 10.0) / 100.0;

            let mut line = [0u8, 0, 0];
            fields[0].encode(sine, &mut line);
            fields[1].encode(triangle, &mut line);

            let decoded_sine: f32 = fields[0].decode(&line);
            let decoded_triangle: f32 = fields[1].decode(&line);

            assert!(sine - decoded_sine <= 1. + 0.001);
            assert!(triangle - decoded_triangle <= 0.05 + 0.001);
        }
    }
}
