use crate::prelude::*;
use serde_json;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;

use crate::prelude::ServiceError::TooManyArgs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::string::String;
use csv::{ReaderBuilder, StringRecord};

#[derive(Clone, Debug)]
pub struct TableObject {
    pub db_name: String,
    pub table_name: String,
    pub db_path: String,
}

impl TableObject {
    /// This function reads file, which stores info about fields.
    /// # Arguments
    ///
    /// * `db_path` - Path to database, where table was created
    ///
    pub fn read_table_info(&self) -> Result<Vec<FieldInfo>, HandlerError> {
        let table_info_path = format!("{}/{}/{}_info", self.db_path, self.db_name, self.table_name);
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

    pub fn get_path(&self) -> String {
        return format!("{}/{}/{}.csv", self.db_path, self.db_name, self.table_name)
    }

    /// This function add record to the table. NOT TESTED YET.
    /// # Arguments
    ///
    /// * `db_path` - Path to database, where table was created.
    /// * `record` - Hashmap with value of fields and its types.
    ///
    pub fn add_record(
        &self,
        record: HashMap<String, String>,
    ) -> std::result::Result<(), HandlerError> {
        let table_path = self.get_path();

        let table_fields = self.read_table_info().unwrap();
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

        let mut file = ok_or_service_err!(OpenOptions::new()
            .append(true)
            .open(table_path));
        let mut writer = csv::Writer::from_writer(file);
        ok_or_service_err!(writer.write_record(&line_record));
        ok_or_service_err!(writer.flush());
        Ok(())
    }

    pub fn find_record_by_name(&self, csv_file: &str, value: &str) -> Result<Option<StringRecord>,
        HandlerError> {
        let file = ok_or_err!(File::open(csv_file));
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        ok_or_service_err!(buf_reader.read_to_string(&mut contents));

        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(contents.as_bytes());
        for result in reader.records() {
            let record = ok_or_service_err!(result);
            if let Some(field) = record.iter().find(|&f| f == value) {
                return Ok(Some(record));
            }
        }

        Ok(None)
    }

    pub fn select(self) -> Result<(), HandlerError> {
        let table_path = self.get_path();
        let reader = BufReader::new(File::open(table_path).unwrap());
        let mut csv_reader = csv::Reader::from_reader(reader);

        let table_fields = ok_or_err!(self.read_table_info());

        for table_field in table_fields.iter() {
            print!("{} ", table_field.field_name)
        }

        for result in csv_reader.records() {
            match result {
                Ok(record) => {
                    println!("{:?}", record);
                }
                Err(err) => {
                    eprintln!("Error reading CSV record: {}", err);
                }
            }

        }
        Ok(())
    }

    /// This function edit record in the table.
    /// # Arguments
    /// FIXME: rewrite this comments
    /// * `db_path` - Path to database, where table was created.
    /// * `fields_to_change` - Hashmap with names of fields and its present values.
    /// * `changes` - Hashmap with names of fields and its future (new) values.
    ///
    // pub fn edit_record(
    //     &self,
    //     db_path: String,
    //     fields_to_change: HashMap<String, String>,
    //     new_values: HashMap<String, String>,
    // ) -> Result<(), HandlerError> {
    //     let table_path = self.get_path();
    //     let table_fields = self.read_table_info(db_path.clone()).unwrap();
    //
    //     let headers = ok_or_err!(self.get_headers(table_fields.clone()));
    //     if headers.len() < new_values.keys().count() {
    //         return Err(HandlerError::ServiceErrors(TooManyArgs));
    //     }
    //
    //     let mut fields: HashMap<String, FieldType> = HashMap::new();
    //     for field in &table_fields.clone() {
    //         fields.insert(field.clone().field_name, field.clone().field.field_type);
    //     }
    //
    //     let mut reader = ok_or_err!(csv::Reader::from_path(&table_path));
    //     let mut writer = ok_or_err!(csv::Writer::from_path(&table_path));
    //
    //     let mut found_record = false;
    //
    //     let mut line_record: Vec<String> = vec![];

        //
        // for result in reader.records() {
        //     let record = ok_or_err!(result);
        //
        //     // record.iter().map(|| {
        //     //     if key == &fields_to_change[&0] { true } else { false }
        //     // }).collect();
        //
        //     if record.iter().map(|key| {
        //         if key == &fields_to_change[&0] { true } else { false }
        //     }) {
        //         let mut line_record: Vec<String> = vec![];
        //
        //         for column in headers.iter() {
        //             let future_field = match new_values.get(column.as_str()) {
        //                 Some(field) => field,
        //                 None => break,
        //             };
        //
        //             if column == &fields_to_change[&0] {
        //                 if FieldType::convert_value_type_from_str(future_field.as_str())
        //                     != fields[&fields_to_change[&0]]
        //                 {
        //                     return Err(HandlerError::TableError(TableFailure {
        //                         record: serde_json::to_string(
        //                             &new_values
        //                                 .values()
        //                                 .map(|value| value.to_owned())
        //                                 .collect::<Vec<String>>(),
        //                         )
        //                             .unwrap(),
        //                         msg: format!(
        //                             "FieldType mismatch for field {}: future field type is {:?}, field_to_change type is {:?}",
        //                             column,
        //                             FieldType::convert_value_type_from_str(future_field.as_str()),
        //                             fields[&fields_to_change[&0]]
        //                         ),
        //                     }));
        //                 }
        //                 line_record.push(future_field.to_string());
        //             } else {
        //                 let value = record[column.as_str()].to_owned();
        //                 line_record.push(value);
        //             }
        //         }
        //
        //         ok_or_service_err!(writer.write_record(&line_record));
        //         found_record = true;
        //     } else {
        //         ok_or_service_err!(writer.write_record(&record));
        //     }
        // }
    //     for column in headers.iter() {
    //         let future_field = match new_values.get(column.as_str()) {
    //             Some(field) => field,
    //             None => line_record.push(),
    //         };
    //
    //         if FieldType::convert_value_type_from_str(future_field.as_str())
    //             == FieldType::convert_value_type_from_str(record.expect("").column.as_str()) {
    //             if let Some(value) = new_values.get(column.as_str()) {
    //                 line_record.push(value.to_string());
    //             }
    //         } else {
    //             return Err(HandlerError::TableError(TableFailure {
    //                 record: serde_json::to_string(
    //                     &new_values
    //                         .values()
    //                         .map(|value| value.to_owned())
    //                         .collect::<Vec<String>>(),
    //                 )
    //                     .unwrap(),
    //                 msg: format!(
    //                     "FieldType mismatch for field {}: future field type is {:?}, need to be - {:?}",
    //                     column,
    //                     FieldType::convert_value_type_from_str(future_field.as_str()),
    //                     FieldType::convert_value_type_from_str(field_to_change.as_str())
    //                 ),
    //             }));
    //         }
    //     }
    //
    //     ok_or_service_err!(writer.write_record(&line_record));
    //     ok_or_service_err!(writer.into_inner());
    //
    //     if !found_record {
    //         return Err(HandlerError::TableError(TableFailure {
    //             record: serde_json::to_string(
    //                 &new_values
    //                     .values()
    //                     .map(|value| value.to_owned())
    //                     .collect::<Vec<String>>(),
    //             )
    //             .unwrap(),
    //             msg: format!("Record with field_to_change '{}' not found", fields_to_change[&0]),
    //         }));
    //     }
    //
    //     Ok(())
    // }

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
