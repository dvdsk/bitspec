use log::error;
use rand::{FromEntropy, Rng};
use serde_derive::{Deserialize, Serialize};

use crate::{FieldId, MetaField};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FieldLength {
    pub name: String,
    pub min_value: f32,
    pub max_value: f32,
    pub numb_of_bits: u8, //bits (max 32 bit variables)
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FieldResolution {
    pub name: String,
    pub min_value: f32,
    pub max_value: f32,
    pub resolution: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FieldManual {
    pub name: String,
    pub length: u8,
    pub decode_scale: f32,
    pub decode_add: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FieldSpec {
    BitLength(FieldLength),
    Resolution(FieldResolution),
    Manual(FieldManual),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetaDataSpec {
    pub name: String,
    pub description: String,
    pub fields: Vec<FieldSpec>, //must be sorted lowest id to highest
}

impl Into<Vec<MetaField<f32>>> for MetaDataSpec {
    fn into(mut self) -> Vec<MetaField<f32>> {
        let mut fields = Vec::new();
        let mut start_bit = 0;
        //convert every field enum in the fields vector into a field
        for (id, field) in self.fields.drain(..).enumerate() {
            if id == u8::max_value as usize {
                error!("can only have {} fields", u8::max_value());
                break;
            }
            let (decode_scale, length, name, decode_add) = match field {
                FieldSpec::BitLength(field) => {
                    let max_storable = 2_u32.pow(field.numb_of_bits as u32) as f32;
                    let decode_scale = (field.max_value - field.min_value) / max_storable;

                    let length = field.numb_of_bits;
                    let name = field.name;
                    let decode_add = field.min_value;
                    (decode_scale, length, name, decode_add)
                }
                FieldSpec::Resolution(field) => {
                    let given_range = field.max_value - field.min_value;
                    let needed_range = given_range as f32 / field.resolution as f32;
                    let length = needed_range.log2().ceil() as u8;
                    let decode_scale = field.resolution;

                    let name = field.name;
                    let decode_add = field.min_value;
                    (decode_scale, length, name, decode_add)
                }
                FieldSpec::Manual(field) => {
                    let length = field.length;
                    let decode_scale = field.decode_scale;
                    let name = field.name;
                    let decode_add = field.decode_add;
                    (decode_scale, length, name, decode_add)
                }
            };
            fields.push(MetaField::<f32> {
                id: id as FieldId,
                name,
                offset: start_bit,
                length,
                decode_scale,
                decode_add,
            });
            start_bit += length;
        }
        fields //must be sorted lowest id to highest
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetaData {
    pub name: String,
    pub description: String,
    pub key: u64,
    pub fields: Vec<MetaField<f32>>, //must be sorted lowest id to highest
}

impl MetaData {
    pub fn fieldsum(&self) -> u16 {
        let field = self.fields.last().unwrap();
        let bits = field.offset as u16 + field.length as u16;
        divide_up(bits, 8)
    }
}

#[inline]
fn divide_up(t: u16, n: u16) -> u16 {
    (t + (n - 1)) / n
}

impl Into<MetaData> for MetaDataSpec {
    fn into(self) -> MetaData {
        //set the security key to a random value
        let mut rng = rand::StdRng::from_entropy();
        MetaData {
            name: self.name.clone(),
            description: self.description.clone(),
            key: rng.gen(),
            fields: self.into(), //must be sorted lowest id to highest
        }
    }
}
