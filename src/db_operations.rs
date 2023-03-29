use std::fs::{self, File};
use std::io::prelude::*;
use serde_json::json;
use csv;


pub fn create_database(db_path: String, db_name: String) -> Result<(), String> {
    let dir_path = format!("{}/{}", db_path, db_name).trim().to_string();

    match fs::create_dir_all(dir_path.as_str()) {
        Ok(_) => {
            let csv_path = format!("{}/relations.csv", &db_path);
            let mut csv_relations = File::create(&csv_path).map_err(|e| format!("Failed \
            to create CSV file: {}", e))?;
            writeln!(csv_relations, "from_table,to_table,field,type").map_err(|e| format!("Failed to write to CSV file: {}", e))?;

            println!("Successfully created database.");
            Ok(())
        },
        Err(e) => Err(format!("Failed to create database: {}", e)),
    }
}

pub fn delete_database(db_path: String) {
    match fs::remove_dir_all(db_path.as_str()) {
        Ok(_) => println!("Folder deleted successfully"),
        Err(e) => println!("Error deleting folder: {:?}", e),
    }
}
