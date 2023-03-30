use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::format;
use crate::binary_storage::{Field, ForeignKey};
use std::fs::File;
use std::io::{self, Write, BufRead, BufReader, BufWriter};
use std::ops::Deref;
use serde::{Serialize};
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
        -> std::result::Result<TableObject, String>  {

        let table_path = format!("{}/{}/{}.csv", self.path, self.name, table_name).trim()
            .to_string();
        let mut table = File::create(&table_path).map_err(|e| format!("Failed to create CSV file: \
        {}", e))?;

        let info_table_path = format!("{}/{}/{}_info", self.path, self.name, table_name).trim()
            .to_string();
        let mut info_file = File::create(&info_table_path).map_err(|e| format!("Failed to create \
        CSV file: {}", e))?;

        let mut fields_name: Vec<String> = Vec::new();

        for (field_name, field_obj) in fields {
            fields_name.push(field_name.clone());
            writeln!(
                info_file,
                "{:?}, {:?}, {}",
                field_name, field_obj.field_type, field_obj.is_null
            ).map_err(|e| format!("Failed to create CSV file: {}", e))?;
        }

        if !fks.is_empty() { self.add_fks(info_table_path, fks); };

        writeln!(
            table,
            "{:?}",
            fields_name
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
        ).map_err(|e| format!("Failed to create CSV file: {}", e))?;

        Ok(TableObject {
            db_name: self.name.clone(),
            table_name,
        })
    }

    pub fn add_fks(&self, mut info_file: String, fks: Vec<ForeignKey>) -> std::result::Result<(),
        String>{
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