// pub fn create_db_info_file(db_path: &str, db_name: &str) {
//     let info_path = format!("{}/db_info.json", db_path);
//
//     let mut info_file = File::create(&info_path).expect("Failed to create db_info.json");
//
//     let db_info = json!({
//         "db_name": db_name,
//         "fields": fields,
//     });
//
//     write!(info_file, "{}", db_info.to_string()).expect("Failed to write to db_info.json");
// }