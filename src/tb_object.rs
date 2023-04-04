use crate::prelude::*;
use std::collections::HashMap;

use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::string::String;

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
    pub fn read_table_info(&self, db_path: String) -> Result<Vec<FieldInfo>, AssertFailure> {
        let table_info_path = format!("{}/{}/{}_info", db_path, self.db_name, self.table_name)
            .trim()
            .to_string();
        let mut info_file = ok_or_err!(File::open(table_info_path));
        let fields_info_result: std::result::Result<Vec<FieldInfo>, bincode::Error> =
            bincode::deserialize_from(&mut info_file);
        return match fields_info_result {
            Ok(fields_info) => Ok(fields_info),
            Err(e) => Err(AssertFailure {
                path: file!().to_string(),
                line: line!() as usize,
                msg: format!(
                    "Error while deserializing table info for table {}: {}",
                    self.table_name, e
                ),
            }),
        };
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
        record: HashMap<String, FieldType>,
    ) -> std::result::Result<(), AssertFailure> {
        /// TODO: finish this function
        // - check the type and add record to the table
        let table = ok_or_err!(OpenOptions::new()
            .write(true)
            .truncate(false)
            .open(format!("{}/{}/{}.csv", db_path, self.db_name, self.table_name).trim()));
        let mut writer = csv::Writer::from_writer(BufWriter::new(table));
        unimplemented!()
    }
}
