extern crate bincode;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::format;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::ops::Deref;
use std::string::String;

use serde::Serialize;

use crate::binary_storage::{Field, FieldInfo, ForeignKey};
use crate::field_type::FieldType;

#[derive(Clone, Debug)]
pub struct DbObject {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug)]
pub struct TableObject {
    pub db_name: String,
    pub table_name: String,
}

impl DbObject {
    pub fn create_table(&self, table_name: String, fields: HashMap<String, Field>, fks: Vec<ForeignKey>)
                        -> std::result::Result<TableObject, String> {
        let table_path = format!("{}/{}/{}.csv", self.path, self.name, table_name).trim()
            .to_string();
        let mut table_file = File::create(&table_path);
        match table_file {
            Ok(mut table_file) => {
                let mut csv_writer = csv::Writer::from_writer(table_file);
                csv_writer.write_record(
                    fields.keys().clone().collect::<Vec<_>>()
                ).map_err(|e| format!("Failed to create table: {}", e));
                csv_writer.flush();

                self.create_table_info(&table_name.to_string(), fields);
                // TODO: rebuild fk building mechanism
                // if !fks.is_empty() { self.add_fks(info_table_path, fks); };

                Ok(TableObject {
                    db_name: self.name.clone(),
                    table_name,
                })
            }
            Err(e) => {
                Err("Failed to create table".to_string())
            }
        }
    }
    pub fn create_table_info(&self, table_name: &str, fields: HashMap<String, Field>) -> std::result::Result<(), String> {
        let info_table_path = format!("{}/{}/{}_info", self.path, self.name, table_name).trim()
            .to_string();
        let mut info_file = File::create(&info_table_path).map_err(|e| format!("Failed to create \
        CSV file: {}", e))?;

        let mut fields_info = vec![];
        for (field_name, field_obj) in &fields {
            fields_info.push(FieldInfo { field: field_obj.clone(), field_name: field_name.to_string() })
        }
        let encoded = bincode::serialize(&fields_info);
        match encoded {
            Ok(encoded_data) => {
                let mut file = File::create(info_table_path.as_str());
                match file {
                    Ok(mut file) => {
                        file.write_all(&encoded_data);
                    }
                    Err(e) => {
                        return Err("Failed to create table info".to_string());
                    }
                }
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
        Ok(())
    }
    pub fn read_table_info(&self, table_name: &str) -> Result<Vec<FieldInfo>, String> {
        let mut table_info_path = format!("{}/{}/{}_info", self.path, self.name, table_name).trim()
            .to_string();
        let mut info_file = File::open(table_info_path);
        match info_file {
            Ok(mut file) => {
                let fields_info_result: std::result::Result<Vec<FieldInfo>, bincode::Error> = bincode::deserialize_from(&mut file);
                match fields_info_result {
                    Ok(fields_info) => {
                        return Ok(fields_info);
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    }
                }
            }
            Err(e) => {
                Err("File not found".to_string())
            }
        }
    }

    pub fn add_fks(&self, mut info_file: String, fks: Vec<ForeignKey>) -> std::result::Result<(),
        String> {
        let info_from_file: HashMap<String, Option<Field>> = self.get_info_from_file(info_file.as_str
        ());
        let mut writer: BufWriter<File> = BufWriter::new(File::open(info_file.trim()).unwrap());

        for fk in &fks {
            let info_to_file: HashMap<String, Option<Field>> = self.get_info_from_file(format!(".{}",
                                                                                               fk.to_table_name).as_str());

            // find the types of the fields in the foreign key
            let mut from_field_type: Option<FieldType> = None;
            let mut to_field_type: Option<FieldType> = None;
            for (name, field) in &info_from_file {
                if name.trim() == fk.to_field_name.trim() {
                    from_field_type = field.as_ref().map(|f| f.field_type);
                }
            }
            for (name, field) in &info_to_file {
                if name.trim() == fk.to_field_name.trim() {
                    to_field_type = field.as_ref().map(|f| f.field_type);
                }
            }

            // check if the types match
            if let (Some(from_type), Some(to_type)) = (from_field_type, to_field_type) {
                if from_type != to_type {
                    return Err(format!("Field types don't match for foreign key from '{}' to '{}'", fk.to_field_name, fk.to_table_name));
                }
            }

            // write the foreign key info to the file
            writeln!(writer, "foreign key: {} -> {}.{}", info_file, fk.to_table_name, fk.to_field_name)
                .map_err(|e| format!("Failed to write to the info file: {}", e))?;
        }

        Ok(())
    }
    // TODO: deprecated code
    pub fn get_info_from_file(&self, file_path: &str) -> HashMap<String, Option<Field>> {
        let file = File::open(file_path.trim()).unwrap();
        let reader = BufReader::new(file);

        let mut fields_info = HashMap::new();
        for line in reader.lines() {
            let line = line.expect("Error while reading a line");
            let parts: Vec<_> = line.split(',').map(|part| part).collect();
            let parts_len = parts.clone().len();
            if parts_len == 3 {
                let field_name = parts[0].to_string();
                let field_type = parts[1].to_string();
                let mut is_null_str = parts[2].to_string();
                let is_null = is_null_str.parse::<bool>().unwrap();
                let field = Field {
                    field_type: FieldType::from_str(&field_type).unwrap(),
                    is_null: is_null,
                };
                fields_info.insert(field_name, Some(field));
            } else {
                fields_info.insert(parts[0].to_string(), None);
            }
        }

        fields_info
    }
}