use crate::prelude::*;
use serde_json;
use std::collections::HashMap;

use crate::prelude::ServiceError::{TooManyArgs, TypeDoesntMatch};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
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
    pub fn read_table_info(&self) -> Result<Vec<FieldInfo>, HandlerError> {
        let table_info_path = format!("{}/{}/{}_info", self.db_path, self.db_name, self.table_name);
        let mut info_file = ok_or_err!(File::open(table_info_path.trim()));
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
        format!("{}/{}/{}.csv", self.db_path, self.db_name, self.table_name)
    }

    /// This function add record to the table. NOT TESTED YET.
    /// # Arguments
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

        let file = ok_or_service_err!(OpenOptions::new()
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
    ///
    /// * `fields_to_change` - Hashmap with names of fields and its present values.
    /// * `changes` - Hashmap with names of fields and its future (new) values.
    pub fn edit_record(
        &self,
        where_: HashMap<String, String>,
        changes: HashMap<String, String>,
    ) -> Result<(), HandlerError> {
        let table_path = self.get_path();
        let table_fields = self.read_table_info().unwrap();
        let headers = ok_or_err!(self.get_headers(table_fields.clone()));

        if headers.len() < changes.keys().count() {
            return Err(HandlerError::ServiceErrors(TooManyArgs));
        }

        let mut fields: HashMap<String, FieldType> = HashMap::new();
        for field in &table_fields {
            fields.insert(field.clone().field_name, field.clone().field.field_type);
        }

        let mut new_full_record: Vec<Vec<String>> = Vec::new();

        let r_reader = BufReader::new(ok_or_service_err!(File::open(table_path.clone())));
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(r_reader);

        for records in reader.records() {
            let record = ok_or_service_err!(records);
            let mut line_record: Vec<String> = vec![];
            let mut need_to_update = false;

            for (i, column) in headers.iter().enumerate() {
                let value = record.get(i).expect("Error wile getting a value");

                if let Some(where_value) = where_.get(column) {
                    if where_value == value {
                        need_to_update = true;
                        break;
                    }
                }
            }

            if !need_to_update {
                new_full_record.push(record.clone().iter().map(|field| field.to_string()).collect());
            } else {
                for (i, column) in headers.iter().enumerate() {
                    let current_value = record.get(i).expect("Error wile getting a value");
                    if let Some(new_value) = changes.get(column) {
                        if FieldType::convert_value_type_from_str(new_value.as_str())
                            == FieldType::convert_value_type_from_str(current_value) {
                            line_record.push(new_value.to_string());
                        } else { return Err(HandlerError::ServiceErrors(TypeDoesntMatch)); }
                    } else {
                        line_record.push(current_value.to_string());
                    }
                }
                new_full_record.push(line_record.clone());
            }
        }

        // We need to rewrite full file.
        let mut writer = ok_or_service_err!(csv::Writer::from_path(table_path.clone()));
        for item in new_full_record {
            ok_or_service_err!(writer.write_record(item));
        }
        ok_or_service_err!(writer.flush());

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
