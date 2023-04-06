extern crate bincode;

use csv::{Writer, WriterBuilder};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::string::String;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct DbObject {
    pub name: String,
    pub path: String,
}

impl DbObject {
    /// This function create table in the database path from db object
    /// # Arguments
    ///
    /// * `table_name` - Name of the table to create
    /// * `fields` - Vector of fields (table header)
    /// * `foreign_keys` - Vector of foreign keys, it also can be empty
    ///
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use dbms_rust::prelude::{Field, FieldType};
    /// let mut fields_first: HashMap<String, Field> = HashMap::from([
    /// ("id".to_string(), Field { field_type: FieldType::Int, is_null: false }),
    /// ("name".to_string(), Field { field_type: FieldType::Text, is_null: false})]);
    /// let table_obj_first = db_object.create_table(table_name, fields, vec![]);
    /// ```
    ///
    pub fn create_table(
        &self,
        table_name: String,
        fields: HashMap<String, Field>,
        foreign_keys: Vec<ForeignKey>,
    ) -> std::result::Result<TableObject, OperationFailure> {
        let table_path = format!("{}/{}/{}.csv", self.path, self.name, table_name)
            .trim()
            .to_string();
        let table_file = ok_or_err!(File::create(&table_path));

        let mut csv_writer = WriterBuilder::new()
            .delimiter(b',')
            .quote_style(csv::QuoteStyle::Always)
            .from_writer(table_file);
        ok_or_err!(csv_writer.write_record(fields.keys().clone().collect::<Vec<_>>()));

        ok_or_err!(self.create_table_info(&table_name, fields));

        let table_object = TableObject {
            db_name: self.name.clone(),
            table_name,
        };

        // FIXME: add remark to the type to mention that it is fk
        if !foreign_keys.is_empty() {
            ok_or_err!(self.add_fks(table_object.clone(), foreign_keys));
        };

        ok_or_err!(csv_writer.flush());

        Ok(table_object)
    }

    /// This function create binary file with info about table fields
    /// # Arguments
    ///
    /// * `table_name` - Name of the table to create
    /// * `fields` - Vector of fields (table header)
    ///
    pub fn create_table_info(
        &self,
        table_name: &str,
        fields: HashMap<String, Field>,
    ) -> std::result::Result<(), OperationFailure> {
        let info_table_path = format!("{}/{}/{}_info", self.path, self.name, table_name)
            .trim()
            .to_string();

        let mut fields_info = vec![];
        for (field_name, field_obj) in &fields {
            fields_info.push(FieldInfo { field: field_obj.clone(), field_name: field_name.to_string() })
        }
        let encoded = ok_or_err!(bincode::serialize(&fields_info));

        let mut file = ok_or_err!(File::create(info_table_path.as_str()));
        ok_or_err!(file.write_all(&encoded));
        Ok(())
    }

    /// This function add relations between table. It make record to `relations.csv` file.
    /// # Arguments
    ///
    /// * `table_obj` - Table object. It has name of table and database.
    /// * `foreign_keys` - Array of foreign keys, that we pass into `create_table` function.
    ///
    pub fn add_fks(
        &self,
        table_obj: TableObject,
        foreign_keys: Vec<ForeignKey>,
    ) -> std::result::Result<(), OperationFailure> {
        let info_from_file: Vec<FieldInfo> =
            ok_or_err!(table_obj.read_table_info(self.path.clone()));
        let file = ok_or_err!(OpenOptions::new()
            .write(true)
            .truncate(false)
            .open(format!("{}/{}/relations.csv", self.path, self.name).trim()));
        let mut writer = csv::Writer::from_writer(BufWriter::new(file));

        for fk in &foreign_keys {
            let info_to_file: Vec<FieldInfo> =
                ok_or_err!(table_obj.read_table_info(self.path.clone()));

            // find the types of the fields in the foreign key
            let mut from_field_type: Option<FieldType> = None;
            let mut to_field_type: Option<FieldType> = None;

            for field in info_from_file.clone() {
                if field.field_name.trim() == fk.to_field_name.trim() {
                    from_field_type = Some(field.clone().field.field_type);
                }
            }
            for field in &info_to_file.clone() {
                if field.field_name.trim() == fk.to_field_name.trim() {
                    to_field_type = Some(field.clone().field.field_type);
                }
            }

            // check if the types match
            if let (Some(from_type), Some(to_type)) = (from_field_type, to_field_type) {
                if from_type != to_type {
                    return Err(OperationFailure {
                        path: file!().to_string(),
                        line: line!() as usize,
                        msg: format!(
                            "Field types don't match for foreign key from '{}' to \
                        '{}'",
                            fk.to_field_name, fk.to_table_name
                        ),
                    });
                }
            }

            ok_or_err!(writer.write_record(&[
                table_obj.clone().table_name,
                fk.to_table_name.to_string(),
                fk.to_field_name.to_string()
            ]));
        }
        ok_or_err!(writer.flush());

        Ok(())
    }
}
