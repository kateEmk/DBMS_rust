use crate::prelude::FieldType;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub field_type: FieldType,
    pub is_null: bool,
    pub is_pk: bool,
    pub is_fk: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FieldInfo {
    pub field: Field,
    pub field_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ForeignKey {
    pub to_table_name: String,
    pub to_field_name: String,
}

impl Field {
    pub fn is_null(&self) -> bool {
        self.is_null
    }
}
