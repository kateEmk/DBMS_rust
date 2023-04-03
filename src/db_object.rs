extern crate bincode;

use csv::{Writer, WriterBuilder};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write};
use std::string::String;

use crate::prelude::*;

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
    pub fn create_table(
        &self,
        table_name: String,
        fields: HashMap<String, Field>,
        fks: Vec<ForeignKey>,
    ) -> std::result::Result<TableObject, AssertFailure> {
        let table_path = format!("{}/{}/{}.csv", self.path, self.name, table_name)
            .trim()
            .to_string();
        let mut table_file = ok_or_err!(File::create(&table_path));

        let mut csv_writer = WriterBuilder::new()
            .delimiter(b',')
            .quote_style(csv::QuoteStyle::Always)
            .from_writer(table_file);
        ok_or_err!(csv_writer.write_record(fields.keys().clone().collect::<Vec<_>>()));

        ok_or_err!(self.create_table_info(&table_name.to_string(), fields));

        // FIXME: rebuild fk building mechanism
        if !fks.is_empty() { self.add_fks(table_name.clone(), fks); };

        ok!(csv_writer.flush());

        Ok(TableObject {
            db_name: self.name.clone(),
            table_name,
        })
    }

    pub fn create_table_info(
        &self,
        table_name: &str,
        fields: HashMap<String, Field>,
    ) -> std::result::Result<(), AssertFailure> {
        let info_table_path = format!("{}/{}/{}_info", self.path, self.name, table_name)
            .trim()
            .to_string();

        let mut fields_info = vec![];
        for (field_name, field_obj) in &fields {
            fields_info.push(FieldInfo {
                field: field_obj.clone(),
                field_name: field_name.to_string(),
            })
        }
        let encoded = ok_or_err!(bincode::serialize(&fields_info));

        let mut file = ok_or_err!(File::create(info_table_path.as_str()));
        ok!(file.write_all(&encoded));

        Ok(())
    }

    pub fn read_table_info(&self, table_name: &str) -> Result<Vec<FieldInfo>, AssertFailure> {
        let mut table_info_path = format!("{}/{}/{}_info", self.path, self.name, table_name)
            .trim()
            .to_string();
        let mut info_file = ok_or_err!(File::open(table_info_path));
        let fields_info_result: std::result::Result<Vec<FieldInfo>, bincode::Error> =
            bincode::deserialize_from(&mut info_file);
        return match fields_info_result {
            Ok(fields_info) => {
                Ok(fields_info)
            }
            Err(e) => {
                Err(AssertFailure {
                    path: file!().to_string(),
                    line: line!() as usize,
                    msg: format!("Error while deserializing table info for table {}: {}",
                                 table_name, e.to_string()),
                })
            }
        }
    }

    pub fn add_fks(
        &self,
        mut table_name: String,
        fks: Vec<ForeignKey>,
    ) -> std::result::Result<csv::Result<()>, AssertFailure> {
        let info_from_file: Vec<FieldInfo> = ok_or_err!(self.read_table_info(table_name.as_str()));
        let file = ok_or_err!(File::open(format!("{}/{}/{}_info", self.path, self.name,
            table_name)));
        let mut writer = WriterBuilder::new()
            .delimiter(b',')
            .quote_style(csv::QuoteStyle::Always)
            .from_writer(file);

        let mut record: Vec<String> = Vec::new();

        for fk in &fks {
            let info_to_file: Vec<FieldInfo> = ok_or_err!(self.read_table_info(fk.to_table_name
                .as_str()));

            // find the types of the fields in the foreign key
            let mut from_field_type: Option<FieldType> = None;
            let mut to_field_type: Option<FieldType> = None;

            for field in info_from_file.clone() {
                if field.field_name.trim() == fk.to_field_name.trim() {
                    from_field_type = Some(field.field.field_type);
                }
            }
            for field in &info_to_file.clone() {
                if field.field_name.trim() == fk.to_field_name.trim() {
                    to_field_type = Some(field.field.field_type);
                }
            }

            // check if the types match
            if let (Some(from_type), Some(to_type)) = (from_field_type, to_field_type) {
                if from_type != to_type {
                    return Err(AssertFailure {
                        path: file!().to_string(),
                        line: line!() as usize,
                        msg:  format!("Field types don't match for foreign key from '{}' to \
                        '{}'", fk.to_field_name, fk.to_table_name).to_string()
                    })
                }
            }
            record.push(format!("foreign key: {} -> {}.{}", table_name, fk
                .to_table_name, fk.to_field_name));

        }
        for rec in record {
            ok_or_err!(writer.write_record(&[rec]));
        }
        ok_or_err!(writer.flush());

        Ok(Ok(()))
    }

}
