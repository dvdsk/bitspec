use crate::compression;
use serde_derive::{Deserialize, Serialize};

trait FloatIterExt {
    fn float_min(&mut self) -> f64;
    fn float_max(&mut self) -> f64;
}

impl<T> FloatIterExt for T
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
pub struct Meta {
    pub id: FieldId,
    pub name: String,
    pub field: Field,
}

impl Meta {
    pub fn length(&self) -> u8 {
        self.field.length()
    }
    pub fn offset(&self) -> u8 {
        self.field.offset()
    }
    pub fn set_offset(&mut self, offset: u8) {
        self.field.set_offset(offset);
    }
    pub fn encode(&self, value: FieldValue, line: &mut [u8]) {
        self.field.encode(value, line);
    }
    pub fn decode(&self, line: &[u8]) -> FieldValue {
        self.field.decode(line)
    }
}

impl Into<Field> for Meta {
    fn into(self) -> Field {
        self.field
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FieldValue {
    Bool(bool),
    F32(f32),
    F64(f64),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Field {
    Bool(BoolField),
    F32(FloatField<f32>),
    F64(FloatField<f64>),
}

impl Field {
    pub fn length(&self) -> u8 {
        match &self {
            Self::Bool(_) => 1,
            Self::F32(f) => f.length,
            Self::F64(f) => f.length,
        }
    }
    pub fn offset(&self) -> u8 {
        match &self {
            Self::Bool(b) => b.offset,
            Self::F32(f) => f.offset,
            Self::F64(f) => f.offset,
        }
    }
    pub fn set_offset(&mut self, offset: u8) {
        match self {
            Field::Bool(ref mut f) => f.offset = offset,
            Field::F32(ref mut f) => f.offset = offset,
            Field::F64(ref mut f) => f.offset = offset,
        }
    }
    pub fn encode(&self, value: FieldValue, line: &mut [u8]) {
        match (&self, value) {
            (Self::Bool(f), FieldValue::Bool(value)) => f.encode(value, line),
            (Self::F32(f), FieldValue::F32(value)) => f.encode(value, line),
            (Self::F64(f), FieldValue::F64(value)) => f.encode(value, line),
            (_, value) => panic!("field: {:?}, value: {:?}", &self, value),
        }
    }
    pub fn decode(&self, line: &[u8]) -> FieldValue {
        match &self {
            Self::Bool(f) => FieldValue::Bool(f.decode(line)),
            Self::F32(f) => FieldValue::F32(f.decode(line)),
            Self::F64(f) => FieldValue::F64(f.decode(line)),
        }
    }
}

impl Into<f32> for FieldValue {
    fn into(self) -> f32 {
        match self {
            Self::Bool(b) => b as isize as f32,
            Self::F32(f) => f,
            Self::F64(f) => f as f32,
        }
    }
}

impl Into<f64> for FieldValue {
    fn into(self) -> f64 {
        match self {
            Self::Bool(b) => b as isize as f64,
            Self::F32(f) => f as f64,
            Self::F64(f) => f,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BoolField {
    pub offset: u8,
}

impl BoolField {
    pub fn decode(&self, line: &[u8]) -> bool {
        let idx = (self.offset/8) as usize;
        let bitmask = 0b00000001 << (self.offset % 8);
        let bit = line[idx] & bitmask;
        if bit == 0 {
            false
        } else {
            true
        }
    }
    pub fn encode(&self, event: bool, line: &mut [u8]) {
        let idx = (self.offset/8) as usize;
        let bitmask = 0b00000001 << (self.offset % 8);
        if event == false {
            line[idx] &= !bitmask;
        } else {
            line[idx] &= bitmask;
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatField<T> {
    pub offset: u8, //bits
    pub length: u8, //bits (max 32 bit variables)

    pub decode_scale: T,
    pub decode_add: T,
}

#[allow(dead_code)]
impl<T> FloatField<T>
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
        //where D: From<T>+From<u32>+From<u16>+std::ops::Add+std::ops::SubAssign+std::ops::DivAssign+std::ops::AddAssign{
        let int_repr: u32 = compression::decode(line, self.offset, self.length);
        //println!("int regr: {}", int_repr);
        let mut decoded: D = num::cast(int_repr).unwrap();

        //println!("add: {}", self.decode_add);
        //println!("scale: {}", self.decode_scale);

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
    fn test(){
        let fields = &[ // Ble_reliability_testing_dataset
            FloatField::<f32> { // Sine
                    decode_add: -5000.0000000000,
                    decode_scale: 1.0000000000,
                    length: 14,
                    offset: 0},
            FloatField::<f32> { // Triangle
                    decode_add: -10.0000000000,
                    decode_scale: 0.0500000007,
                    length: 10,
                    offset: 14},
        ];

        for i in 0..100 {
            let sine = -5000.0 + i as f32*(5000.0*2.0)/100.0;
            let triangle = 20.0 - i as f32*(20.0+10.0)/100.0;
    
            let mut line = [0u8, 0, 0];
            fields[0].encode(sine, &mut line);
            fields[1].encode(triangle, &mut line);
    
            let decoded_sine: f32 = fields[0].decode(&line);
            let decoded_triangle: f32 = fields[1].decode(&line);
            
            assert!(sine-decoded_sine <= 1.+0.001);
            assert!(triangle-decoded_triangle <= 0.05+0.001 );
        }

        //<debug> app:  99 4C 00               |.L.     
        //<info> app: sine: 21.81

        let line = [0x9du8, 0x4c, 0x00];
        let decoded_sine: f32 = fields[0].decode(&line);
        dbg!(decoded_sine);

        let mut line = [0u8,0,0];
        fields[0].encode(21.81f32, &mut line);
        println!("{:#02x} {:#02x} {:#02x}",line[0],line[1],line[2]);
    }

}
