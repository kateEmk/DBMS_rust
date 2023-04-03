#[derive(Debug, Clone)]
pub struct AssertFailure {
    pub path: String,
    pub line: usize,
    pub msg: String,
}

impl std::error::Error for AssertFailure {}

impl std::fmt::Display for AssertFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "File {}:{}.\nMessage:\n{}",
            self.path, self.line, self.msg
        )
    }
}

impl AssertFailure {
    pub fn pretty(self) -> String {
        format!("{}:{}.\n{}", self.path, self.line, self.msg)
    }
}

#[macro_export]
macro_rules! ok_or_err {
    ($result:expr $(,)?) => {
        match $result {
            Ok(value) => value,
            Err(error) => {
                let msg = format!(
                    "Operation failed. \nError message: {:?}", error
                );
                fail_inner!(msg);
            }
        }
    };
}
pub use ok_or_err;

#[macro_export]
macro_rules! ok {
    ($result:expr $(,)?) => {
        match $result {
            Ok(value) => value,
            Err(error) => {
                let msg = format!(
                    "Operation failed. \nError message: {:?}", error
                );
                fail_inner!(msg);
            }
        }
    };
}
pub use ok;

#[macro_export]
macro_rules! fail {
    ($tx:expr $(,)?) => {{
        let msg = format!("Operation failed.");
        fail_inner!(msg)
    }};
}
pub use fail;

#[macro_export]
macro_rules! fail_inner {
    ($msg: expr $(,)?) => {
        if cfg!(feature = "fire-panics-on-asserts") {
            panic!("{}", $msg)
        } else {
            return Err(AssertFailure {
                path: file!().to_string(),
                line: line!() as usize,
                msg: $msg,
            }
            .into());
        }
    };
}
pub use fail_inner;
