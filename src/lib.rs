mod compression;
mod field;
mod spec;

pub use field::{Field, MetaField, FloatField, BoolField, FieldId};
pub use spec::{MetaDataSpec, MetaData};
pub use spec::write_template;
