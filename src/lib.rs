mod compression;
mod field;
mod spec;

pub use field::{Field, Meta, FloatField, BoolField, FieldId};
pub use spec::{MetaDataSpec, FixedLine};
pub use spec::write_template;
