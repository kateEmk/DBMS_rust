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
            return Err(OperationFailure {
                path: file!().to_string(),
                line: line!() as usize,
                msg: $msg,
            }
            .into());
        }
    };
}
pub use fail_inner;

#[macro_export]
macro_rules! ok_or_service_err {
    ($result:expr $(,)?) => {
        match $result {
            Ok(value) => value,
            Err(error) => {
                return Err(TableFailure {
                    record: file!().to_string(),
                    msg: error.to_string(),
                }
                .into());
            }
        }
    };
}
pub use ok_or_service_err;