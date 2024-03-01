use std::path::PathBuf;

use crate::api::schema::private as schema;

pub mod mutation;
pub mod query;

cynic::impl_scalar!(PathBuf, schema::StorageItemPath);
