#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
pub mod db_object;
pub mod db_operations;
pub mod utils;
pub mod binary_storage;
pub mod field_type;
pub mod errors;

pub mod prelude {
    pub use crate::db_object::{TableObject, DbObject};
    pub use crate::db_operations::*;
    pub use crate::utils::asserts::*;
    pub use crate::binary_storage::*;
    pub use crate::field_type::FieldType;
    pub use crate::errors::errors::ServiceError;
}