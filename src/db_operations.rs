use crate::prelude::DbObject;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

pub fn create_db(db_path: String, db_name: String) -> Result<DbObject, String> {
    let dir_path = format!("{}/{}", db_path, db_name).trim().to_string();

    match fs::create_dir_all(dir_path.as_str()) {
        Ok(_) => {
            let csv_path = format!("{}/relations.csv", &dir_path);
            let mut csv_relations =
                File::create(&csv_path).map_err(|e| format!("Failed to create CSV file: {}", e))?;
            writeln!(csv_relations, "from_table,to_table,field")
                .map_err(|e| format!("Failed to write to CSV file: {}", e))?;

            println!("Successfully created database.");
            Ok(DbObject {
                name: db_name,
                path: db_path,
            })
        }
        Err(e) => Err(format!("Failed to create database: {}", e)),
    }
}

pub fn get_db(db_path: String, db_name: String) -> Option<DbObject> {
    let db_folder = PathBuf::from(db_path.clone()).join(db_name.clone());

    if db_folder.exists() && db_folder.is_dir() {
        let db_object = DbObject {
            name: db_name,
            path: db_path,
        };
        Some(db_object)
    } else {
        None
    }
}

pub fn delete_db(db_path: String, db_name: String) {
    let dir_path = format!("{}/{}", db_path, db_name).trim().to_string();

    match fs::remove_dir_all(dir_path.as_str()) {
        Ok(_) => println!("Folder deleted successfully"),
        Err(e) => println!("Error deleting folder: {:?}", e),
    }
}
