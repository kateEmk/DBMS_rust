use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use serde_json::json;
use csv;


pub fn create_database(db_path: String, db_name: String, fields: HashMap<String, String>) -> Result<(), String>{
    let dir_path = format!("{}/{}", db_path, db_name).trim().to_string();

    match fs::create_dir_all(&dir_path) {
        Ok(_) => {
            File::create(db_path.as_str()).map_err(|e| format!("Failed to write to CSV file: {}", e))?;
            create_db_info_file(&db_path, &db_name, fields);

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

pub fn create_db_info_file(db_path: &str, db_name: &str, fields: HashMap<String, String>) {
    let info_path = format!("{}/db_info.json", db_path);

    let mut info_file = File::create(&info_path).expect("Failed to create db_info.json");

    let db_info = json!({
        "db_name": db_name,
        "fields": fields,
    });

    write!(info_file, "{}", db_info.to_string()).expect("Failed to write to db_info.json");
}

pub fn delete_database() {
    unimplemented!()
}