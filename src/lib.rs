#![feature(map_try_insert)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
pub mod db_object;
pub mod db_operations;
pub mod utils;
pub mod binary_storage;
pub mod field_type;
pub mod errors;
pub mod tb_object;

extern crate blob;
extern crate bincode;

pub mod prelude {
    pub use crate::db_object::DbObject;
    pub use crate::db_operations::*;
    pub use crate::utils::macroses::*;
    pub use crate::binary_storage::*;
    pub use crate::field_type::FieldType;
    pub use crate::errors::error::*;
    pub use crate::errors::macros_errors::*;
    pub use crate::tb_object::TableObject;
}