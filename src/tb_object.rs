use crate::prelude::*;
use serde_json;
use std::collections::HashMap;

use crate::prelude::ServiceError::{RowDoesntExist, TooManyArgs, TypeDoesntMatch};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::string::String;
use csv::{ReaderBuilder, StringRecord};

#[derive(Clone, Debug)]
pub struct TableObject {
    pub db_name: String,
    pub table_name: String,
    pub db_path: String,
}

pub struct Record {
    from_table: String,
    to_table: String,
    field: String,
}

impl Record {
    fn new(from_table: String, to_table: String, field: String) -> Self {
        Record {
            from_table,
            to_table,
            field,
        }
    }
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

        let table_fields = ok_or_service_err!(self.read_table_info());
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
                    record: ok_or_err!(serde_json::to_string(&record)),
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

     fn _find_record_by_name(&self, csv_file: &str, value: &str) -> Result<Option<StringRecord>,
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

     fn _select(self) -> Result<(), HandlerError> {
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
    /// * `where_` - Hashmap with names of fields and its present values.
    /// * `changes` - Hashmap with names of fields and its future (new) values.
    pub fn edit_record(
        &self,
        where_: HashMap<String, String>,
        changes: HashMap<String, String>,
    ) -> Result<(), HandlerError> {
        let table_path = self.get_path();
        let table_fields = ok_or_service_err!(self.read_table_info());
        let headers = ok_or_err!(self.get_headers(table_fields.clone()));

        if headers.len() < changes.keys().count() {
            return Err(HandlerError::ServiceErrors(TooManyArgs));
        }

        let mut new_full_record: Vec<Vec<String>> = Vec::new();

        let buf_reader = BufReader::new(ok_or_service_err!(File::open(table_path.clone())));
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(buf_reader);

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

    pub fn delete_record(
        &self,
        where_: HashMap<String, String>
    ) -> Result<(), HandlerError> {
        let table_path = self.get_path();
        let table_fields = ok_or_service_err!(self.read_table_info());
        let headers = ok_or_err!(self.get_headers(table_fields.clone()));

        let buf_reader = BufReader::new(ok_or_service_err!(File::open(table_path.clone())));
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(buf_reader);

        let mut new_full_record: Vec<Vec<String>> = Vec::new();

        for records in reader.records() {
            let record = ok_or_service_err!(records);
            let mut need_to_delete = false;
            let mut all_fields_match = true;

            for (i, column) in headers.iter().enumerate() {
                let value = record.get(i).expect("Error wile getting a value");

                if let Some(where_value) = where_.get(column) {
                    if where_value != value {
                        all_fields_match = false;
                    }
                }
            }

            if all_fields_match {
                need_to_delete = true;
            }

            if !need_to_delete {
                new_full_record.push(record.clone().iter().map(|field| field.to_string()).collect());
            }

        }

        let mut writer = ok_or_service_err!(csv::Writer::from_path(table_path.clone()));
        for item in new_full_record {
            ok_or_service_err!(writer.write_record(item));
        }
        ok_or_service_err!(writer.flush());

        Ok(())
    }

    pub fn _read_relations_data(&self) -> Result<Vec<Record>, HandlerError> {
        let relations_file_path = format!("{}/{}/relations.csv", self.db_path, self
            .db_name);

        let buf_reader = ok_or_service_err!(File::open(relations_file_path.clone()));
        let reader = BufReader::new(buf_reader);

        let mut result: Vec<Record> = Vec::new();

        for records in reader.lines() {
            let record = ok_or_err!(records);

            let fields: Vec<&str> = record.split(',').collect();
            let line = Record::new(
                fields[0].to_string(),
                fields[1].to_string(),
                fields[2].to_string(),
            );
            result.push(line);
        }

        Ok(result)
    }

    pub fn delete_fks(
        &self,
        row_name: String
    ) -> Result<(), HandlerError> {
        let relations_file_path = format!("{}/{}/relations.csv", self.db_path, self.db_name);
        let buf_reader = ok_or_service_err!(File::open(relations_file_path.clone()));
        let reader = BufReader::new(buf_reader);

        let mut new_full_record: Vec<Record> = Vec::new();

        for records in reader.lines() {
            let record = ok_or_err!(records);

            let fields: Vec<&str> = record.split(',').collect();
            if (fields[0] == self.table_name && fields[2] == row_name)
                || (fields[1] == self.table_name && fields[2] == row_name) {
                println!("Row {} is a foreign key, it will be deleted.", row_name.clone());
                continue;
            } else {
                let line = Record::new(
                    fields[0].to_string(),
                    fields[1].to_string(),
                    fields[2].to_string(),
                );
                new_full_record.push(line);
            }
        }

        let mut writer = ok_or_service_err!(csv::Writer::from_path(relations_file_path.clone()));
        for item in new_full_record.into_iter() {
            ok_or_service_err!(writer.write_record([&item.from_table, &item.to_table, &item
                .field]));
        }
        ok_or_service_err!(writer.flush());

        Ok(())
    }

    //FIXME
    pub fn delete_row(
        &self,
        row_name: &str
    ) -> Result<(), HandlerError> {
        let table_path = self.get_path();
        let table_fields = ok_or_service_err!(self.read_table_info());
        let headers = ok_or_err!(self.get_headers(table_fields.clone()));
        if !headers.contains(&row_name.to_string()) {
            return Err(HandlerError::ServiceErrors(RowDoesntExist))
        }

        let buf_reader = BufReader::new(ok_or_service_err!(File::open(table_path.clone())));
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(buf_reader);

        self.delete_fks(row_name.to_string());

        let mut new_full_record: Vec<Vec<String>> = Vec::new();

        for row_result in reader.records()
        {
            let row = ok_or_err!(row_result);
            let mut row_fields: Vec<String> = row.iter().map(|s| s.to_string()).collect();

            if let Some(index) = headers.iter().position(|f| f == row_name) {
                row_fields.remove(index);
            }

            new_full_record.push(row_fields);
        }

        let mut writer = ok_or_service_err!(csv::Writer::from_path(table_path.clone()));
        for item in new_full_record {
            ok_or_service_err!(writer.write_record(item));
        }
        ok_or_service_err!(writer.flush());

        Ok(())
    }

    /// TODO: write
    pub fn delete_table(&self) -> Result<(), HandlerError> {
        unimplemented!()
    }
}
