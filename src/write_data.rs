use crate::field_type::FieldType;

pub fn validate_type(input_type: String) {
    FieldType::from_str(input_type.as_str()).unwrap();
}