#[derive(Debug, Clone)]
pub struct OperationFailure {
    pub path: String,
    pub line: usize,
    pub msg: String,
}

impl std::error::Error for OperationFailure {}

impl std::fmt::Display for OperationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "File {}:{}.\nMessage:\n{}",
            self.path, self.line, self.msg
        )
    }
}

impl OperationFailure {
    pub fn pretty(self) -> String {
        format!("{}:{}.\n{}", self.path, self.line, self.msg)
    }
}

#[derive(Debug, Clone)]
pub struct TableFailure {
    pub record: String,
    pub msg: String,
}

impl std::error::Error for TableFailure {}

impl std::fmt::Display for TableFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Where error occurred :{}.\nMessage:\n{}",
            self.record, self.msg
        )
    }
}

impl TableFailure {
    pub fn pretty(self) -> String {
        format!("{}.\n{}", self.record, self.msg)
    }
}