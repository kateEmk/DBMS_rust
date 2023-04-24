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
    Incorrect
}

impl FieldType {
    pub fn from_str(s: &String) -> Option<FieldType> {
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

    pub fn convert_value_type_from_str(v: &str) -> FieldType {
        let mut typ: Option<FieldType> = None;
        let len = v.len();

        if let Ok(v) = v.parse::<i32>() {
            typ = Some(FieldType::Int);
        } else if let Ok(v) = v.parse::<f32>() {
            typ = Some(FieldType::Float);
        } else if let Ok(v) = v.parse::<f64>() {
            typ = Some(FieldType::Double);
        } else if let Ok(v) = v.parse::<String>() {
            typ = Some(FieldType::Text);
        } else if len > 6 && &v[..6] == "VARCHAR" {
            if let Ok(max_len) = v[7..len - 1].parse::<usize>() {
                typ = Some(FieldType::Varchar(max_len));
            }
        } else if len == 4 && &v[..4] == "BLOB" {
            typ = Some(FieldType::Blob);
        }

        return match typ {
            Some(ty) => ty,
            None => FieldType::Incorrect,
        }
    }
}
