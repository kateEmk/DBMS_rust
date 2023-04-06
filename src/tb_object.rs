use crate::prelude::*;
use std::collections::HashMap;
use serde_json;

use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::string::String;
use crate::prelude::ServiceError::ErrorAddingToTheFile;

#[derive(Clone, Debug)]
pub struct TableObject {
    pub db_name: String,
    pub table_name: String,
}

impl TableObject {
    /// This function reads file, which stores info about fields.
    /// # Arguments
    ///
    /// * `db_path` - Path to database, where table was created
    ///
    pub fn read_table_info(&self, db_path: String) -> Result<Vec<FieldInfo>, HandlerError> {
        let table_info_path = format!("{}/{}/{}_info", db_path, self.db_name, self.table_name);
        let mut info_file = ok_or_err!(File::open(table_info_path.clone().trim()));
        let fields_info_result: std::result::Result<Vec<FieldInfo>, bincode::Error> =
            bincode::deserialize_from(&mut info_file);
        Ok(ok_or_err!(fields_info_result))
    }

    /// This function add record to the table. NOT IMPLEMENTED YET.
    /// # Arguments
    ///
    /// * `db_path` - Path to database, where table was created.
    /// * `record` - Hashmap with value of fields and its types.
    ///
    pub fn add_record(
        &self,
        db_path: String,
        record: HashMap<String, String>,
    ) -> std::result::Result<(), HandlerError> {
        let table = ok_or_err!(OpenOptions::new()
            .write(true)
            .truncate(false)
            .open(format!("{}/{}/{}.csv", db_path, self.db_name, self.table_name).trim()));
        let mut writer = csv::Writer::from_writer(BufWriter::new(table));

        let table_fields = self.read_table_info(db_path).unwrap();
        let mut fields: HashMap<String, FieldType> = HashMap::new();
        for field in table_fields {
            fields.insert(field.field_name, field.field.field_type);
        }

        let mut line_record = vec![];

        for (field_t_name, field_t_type) in fields {
            if FieldType::convert_value_type_from_str(record.get(field_t_name.as_str()).expect(""))
                == field_t_type {
                if let Some(value) = record.get(field_t_name.as_str()) {
                    line_record.push(value);
                }
            } else {
                return Err(HandlerError::TableError(TableFailure {
                    record: serde_json::to_string(&record).unwrap(),
                    msg: "Failed to add record".to_string()
                }));
            }
        }

        ok_or_service_err!(
            writer.write_record(&line_record.clone())
        );
        ok_or_service_err!(writer.flush());
        Ok(())
    }
}
