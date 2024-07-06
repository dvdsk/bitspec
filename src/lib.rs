mod compression;
mod field;
mod spec;

pub use field::{Field, FieldId, MetaField};
pub use spec::{LengthWithOps, RangeWithRes, speclist_to_fields};
