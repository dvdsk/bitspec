use log::error;
use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use rand::{Rng, FromEntropy};

use std::fs;
use std::io;
use std::path::Path;

use crate::{Field, FloatField, BoolField, Meta, FieldId};

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
pub struct Bool {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FieldSpec {
    BitLength(FieldLength),
    Resolution(FieldResolution),
    Manual(FieldManual),
    Bool(Bool)
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetaDataSpec {
    pub name: String,
    pub description: String,
    pub fields: Vec<FieldSpec>, //must be sorted lowest id to highest
}

impl Into<Vec<Meta>> for MetaDataSpec {
    fn into(mut self) -> Vec<Meta> {
        let mut fields = Vec::new();
        let mut start_bit = 0;
        //convert every field enum in the fields vector into a field
        for (id, spec) in self.fields.drain(..).enumerate() {
            if id == u8::max_value as usize {
                error!("can only have {} fields", u8::max_value());
                break;
            }
            let name;
            let length;
            let field = match spec {
                FieldSpec::BitLength(field) => {
                    name = field.name;
                    length = field.numb_of_bits;
                    let max_storable = 2_u32.pow(field.numb_of_bits as u32) as f32;
                    Field::F32(FloatField::<f32> {
                        offset: start_bit,
                        length: field.numb_of_bits,
                        decode_scale: (field.max_value - field.min_value) / max_storable,
                        decode_add: field.min_value,
                    })
                }
                FieldSpec::Resolution(field) => {
                    name = field.name;
                    let given_range = field.max_value - field.min_value;
                    let needed_range = given_range as f32 / field.resolution as f32;
                    length = needed_range.log2().ceil() as u8;
                    Field::F32(FloatField::<f32> {
                        offset: start_bit,
                        length,
                        decode_scale: field.resolution,
                        decode_add: field.min_value,
                    })
                }
                FieldSpec::Manual(field) => {
                    name = field.name;
                    length = field.length;
                    Field::F32(FloatField::<f32> {
                        offset: start_bit,
                        length,
                        decode_scale: field.decode_scale,
                        decode_add: field.decode_add,
                    })
                }
                FieldSpec::Bool(field) => {
                    name = field.name;
                    length = 1;
                    Field::Bool(BoolField {
                        offset: start_bit,
                    })
                }
            };

            let metafield = Meta {
                id: id as FieldId,
                name,
                field,
            };

            fields.push(metafield);
            start_bit += length;
        }
        fields //must be sorted lowest id to highest
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedLine {
	pub name: String,
	pub description: String,
	pub key: u64,
	pub fields: Vec<Meta>,//must be sorted lowest id to highest
}

impl FixedLine {
	pub fn fieldsum(&self) -> u16 {
		let last_field = self.fields.last().unwrap();
		let bits = last_field.offset() as u16 + last_field.length() as u16;
		devide_up(bits, 8)
	}
}

#[inline] fn devide_up(t: u16, n: u16) -> u16 {
	(t + (n-1))/n
}

impl Into<FixedLine> for MetaDataSpec {
    fn into(self) -> FixedLine {
        //set the security key to a random value
        let mut rng = rand::StdRng::from_entropy();
        FixedLine {
			name: self.name.clone(),
			description: self.description.clone(),
			key: rng.gen(),
			fields:  self.into(),//must be sorted lowest id to highest
		}
    }
}

pub fn write_template() -> io::Result<()> {
    let template_field_1 = FieldSpec::BitLength(FieldLength {
        name: String::from("template field name1"),
        min_value: 0.01f32,
        max_value: 10f32,
        numb_of_bits: 10u8, //bits (max 32 bit variables)
    });
    let template_field_2 = FieldSpec::Resolution(FieldResolution {
        name: String::from("template field name2"),
        min_value: 0f32,
        max_value: 100f32,
        resolution: 0.1f32,
    });
    let template_field_3 = FieldSpec::Manual(FieldManual {
        name: String::from("template field name3"),
        length: 10,
        decode_scale: 0.1,
        decode_add: -40f32,
    });
    let template_field_4 = FieldSpec::Bool(Bool {
        name: String::from("template field name4"),
    });
    let metadata = MetaDataSpec {
		name: String::from("template dataset name"),
		description: String::from("This is a template it is not to be used for storing data, please copy this file and edit it. Then use the new file for creating new datasets"),
		fields: vec!(template_field_1, template_field_2, template_field_3, template_field_4),
	};

    if !Path::new("specs").exists() {
        fs::create_dir("specs")?
    }
    match fs::File::create("specs/template.yaml") {
        Ok(f) => {
            if serde_yaml::to_writer(f, &metadata).is_err() {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "could not parse specification",
                ))
            } else {
                Ok(())
            }
        }
        Err(error) => Err(error),
    }
}

/*pub fn write_template_for_test() -> io::Result<()> {
    let template_field_1 = FieldSpec::Manual( FieldManual {
        name: String::from("timestamps"),
        length: 32,
        decode_scale: 1.,
        decode_add: 0.,
    });
    let metadata = MetaDataSpec {
        name: String::from("template dataset name"),
        description: String::from("This is a test spec it is used for verifiying the timeseries interface"),
        fields: vec!(template_field_1),
    };

    if !Path::new("specs").exists() {fs::create_dir("specs")? }
    match fs::File::create("specs/template_for_test.yaml"){
        Ok(f) => {
            if serde_yaml::to_writer(f, &metadata).is_err() {
                Err(io::Error::new(io::ErrorKind::InvalidData, "could not parse specification"))
            } else { Ok(()) }
        },
        Err(error) => {
            println!("error while adding test template");
            Err(error)
        },
    }
}*/
