use serde_derive::{Deserialize, Serialize};
extern crate blob;

/// An enumeration of the possible types for fields in a database.
///
/// # Variants
///
/// * `Int` - A signed integer value.
/// * `Float` - A 32-bit floating point value.
/// * `Double` - A 64-bit floating point value.
/// * `Varchar(max_length)` - A variable length string with a maximum length of `max_length`.
/// * `Text` - A text string of unlimited length.
/// * `Blob` - A binary large object.
///
/// # Examples
///
/// ```
/// use dbms_rust::prelude::FieldType;
///
/// let field_type = FieldType::from_str("varchar(255)").unwrap();
/// assert_eq!(field_type, FieldType::Varchar(255));
/// ```

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
    Int,
    Float,
    Double,
    Varchar(usize),
    // Varchar with a maximum length
    Text,
    Blob,
}

impl FieldType {
    pub fn from_str(s: &str) -> Option<FieldType> {
        match s.to_lowercase().as_str() {
            "int" => Some(FieldType::Int),
            "float" => Some(FieldType::Float),
            "double" => Some(FieldType::Double),
            s if s.starts_with("varchar") => {
                let max_length = s
                    .trim_start_matches("varchar")
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .parse::<usize>()
                    .ok()?;
                Some(FieldType::Varchar(max_length))
            }
            "text" => Some(FieldType::Text),
            "blob" => Some(FieldType::Blob),
            _ => None,
        }
    }
}
