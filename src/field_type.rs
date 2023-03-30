use serde_derive::Deserialize;
use serde_derive::Serialize;

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
    pub(crate) fn from_str(s: &str) -> Option<FieldType> {
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