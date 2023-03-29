use std::collections::HashMap;
use crate::state::FieldType;

pub struct BinaryStorage {
    pub table_name: String,
    pub fields: HashMap<String, Field>,
}

pub struct Field {
    pub field_type: FieldType,
    pub is_null: bool,
}

impl Field {
    pub fn is_null(&self) -> bool {
        self.is_null
    }
}

impl BinaryStorage {
    pub fn get_type(&self, name: String) -> FieldType {
        self.fields.get(&name).unwrap().field_type
    }

    pub fn get_fields(&self) {
        for (name, field) in &self.fields {
            println!("Field name - {}, field type - {:?}, field null - {}", name, field.field_type,
                     field.is_null )
        }
    }
}

