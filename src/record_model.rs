pub struct Record {
    pub from_table: String,
    pub to_table: String,
    pub field: String,
}

impl Record {
    pub fn new(from_table: String, to_table: String, field: String) -> Self {
        Record {
            from_table,
            to_table,
            field,
        }
    }
}