use crate::prelude::*;
use serde_json;
use std::borrow::Borrow;
use std::collections::HashMap;

use crate::prelude::ServiceError::TooManyArgs;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};
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
    pub fn read_table_info(&self, db_path: String) -> Result<Vec<FieldInfo>, HandlerError> {
        let table_info_path = format!("{}/{}/{}_info", db_path, self.db_name, self.table_name);
        let mut info_file = ok_or_err!(File::open(table_info_path.clone().trim()));
        let fields_info_result: std::result::Result<Vec<FieldInfo>, bincode::Error> =
            bincode::deserialize_from(&mut info_file);
        Ok(ok_or_err!(fields_info_result))
    }

    pub fn get_headers(&self, field_info: Vec<FieldInfo>) -> Result<Vec<String>, HandlerError> {
        Ok(field_info
            .iter()
            .map(|field| field.field_name.clone())
            .collect())
    }

    /// This function add record to the table. NOT TESTED YET.
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
        let table_path = format!("{}/{}/{}.csv", db_path, self.db_name, self.table_name);

        let table_fields = self.read_table_info(db_path).unwrap();
        let mut fields: HashMap<String, FieldType> = HashMap::new();

        for field in &table_fields {
            fields.insert(field.clone().field_name, field.clone().field.field_type);
        }

        let headers = ok_or_err!(self.get_headers(table_fields));

        let mut line_record = vec![];

        for column in headers.iter() {
            if FieldType::convert_value_type_from_str(record.get(column.as_str()).expect
            ("Error while getting a type"))
                == *fields.get(column.as_str()).expect("Error while getting a type")
            {
                if let Some(value) = record.get(column.as_str()) {
                    line_record.push(value);
                }
            } else {
                return Err(HandlerError::TableError(TableFailure {
                    record: serde_json::to_string(&record).unwrap(),
                    msg: "Failed to add record".to_string(),
                }));
            }
        }
        println!("{:?}", line_record);

        let mut writer = ok_or_err!(csv::Writer::from_path(&table_path));
        ok_or_service_err!(writer.write_record(&line_record));
        ok_or_service_err!(writer.into_inner());
        Ok(())
    }

    /// This function edit record in the table.
    /// # Arguments
    ///
    /// * `db_path` - Path to database, where table was created.
    /// * `fields_to_change` - Hashmap with names of fields and its present values.
    /// * `changes` - Hashmap with names of fields and its future (new) values.
    ///
    pub fn edit_record(
        &self,
        db_path: String,
        fields_to_change: HashMap<String, String>,
        new_values: HashMap<String, String>,
    ) -> Result<(), HandlerError> {
        if fields_to_change.len() < new_values.keys().count() {
            return Err(HandlerError::ServiceErrors(TooManyArgs));
        }

        let table = ok_or_err!(OpenOptions::new()
            .write(true)
            .truncate(false)
            .open(format!("{}/{}/{}.csv", db_path.clone(), self.db_name, self.table_name).trim()));
        let mut writer = csv::Writer::from_writer(BufWriter::new(table));
        // let mut reader =
        //     ok_or_err!(csv::ReaderBuilder::new()
        //         .has_headers(true)
        //         .from_path(format!(
        //             "{}/{}/{}.csv",
        //             db_path, self.db_name, self.table_name
        //         )));
        let table_fields = ok_or_err!(self.read_table_info(db_path.clone()));
        let mut fields = ok_or_err!(self.get_headers(table_fields));

        let mut line_record: Vec<String> = vec![];

        for field in fields.iter().clone() {
            let future_field = match new_values.get(field.as_str()) {
                Some(field) => field,
                None => {
                    return Err(HandlerError::TableError(TableFailure {
                        record: serde_json::to_string(
                            &new_values
                                .values()
                                .map(|value| value.to_owned())
                                .collect::<Vec<String>>(),
                        )
                        .unwrap(),
                        msg: format!("Field {} not found in changes", field),
                    }))
                }
            };

            let field_to_change = match fields_to_change.get(field.as_str()) {
                Some(field) => field,
                None => {
                    return Err(HandlerError::TableError(TableFailure {
                        record: serde_json::to_string(
                            &new_values
                                .values()
                                .map(|value| value.to_owned())
                                .collect::<Vec<String>>(),
                        )
                        .unwrap(),
                        msg: format!("Field {} not found in fields_to_change", field),
                    }))
                }
            };

            if FieldType::convert_value_type_from_str(future_field.as_str()) == FieldType::convert_value_type_from_str(field_to_change.as_str()) {
                if let Some(value) = new_values.get(field.as_str()) {
                    line_record.push(value.to_string());
                }
            } else {
                return Err(HandlerError::TableError(TableFailure {
                    record: serde_json::to_string(
                        &new_values
                            .values()
                            .map(|value| value.to_owned())
                            .collect::<Vec<String>>(),
                    )
                    .unwrap(),
                    msg: format!(
                        "FieldType mismatch for field {}: future field type is {:?}, field_to_change type is {:?}",
                        field,
                        FieldType::convert_value_type_from_str(future_field.as_str()),
                        FieldType::convert_value_type_from_str(field_to_change.as_str())
                    ),
                }));
            }
        }

        // let mut records = reader.records().enumerate();
        // let row_index = records
        //     .find(|(_, record)| {
        //         let mut matched = true;
        //         for (field, value) in fields_to_change.iter() {
        //             if let Ok(record) = record {
        //                 if record.get((*field).parse().unwrap()).map_or(false, |v| v == value) == false {
        //                     matched = false;
        //                     break;
        //                 }
        //             }
        //         }
        //         matched
        //     })
        //     .map(|(index, _)| index)
        //     .expect("Error while finding a row index.");

        // let old_record = reader.records().nth(row_index).expect("Error while reading old record.");
        // let old_record_vec = ok_or_err!(old_record);
        // let mut old_record_string = String::new();
        // for (index, field) in old_record_vec.iter().enumerate() {
        //     old_record_string.push_str(field);
        //     if index < old_record_vec.len() - 1 {
        //         old_record_string.push(',');
        //     }
        // }

        writer.write_record(line_record);
        writer.flush();
        Ok(())
    }

    /// TODO: decide keep this function or not
    pub fn edit_field(&self) -> Result<(), HandlerError> {
        unimplemented!()
    }

    /// TODO: write
    pub fn delete_record(&self) -> Result<(), HandlerError> {
        unimplemented!()
    }

    /// TODO: write
    pub fn delete_row(&self) -> Result<(), HandlerError> {
        unimplemented!()
    }

    /// TODO: write
    pub fn delete_table(&self) -> Result<(), HandlerError> {
        unimplemented!()
    }
}
