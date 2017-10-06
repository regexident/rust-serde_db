//! Support for deserializing database resultsets, or individual rows, or individual values,
//! into rust types.
//!
//! Implementing DB drivers just need to implement DeserializableResultset,
//! DeserializableRow, and -- more effort -- DbValue.
//!
//! We further recommend to implement the method <code>into_typed()</code> directly on the
//! driver's classes for Resultset and Row with a plain delegation to the provided methods
//! <code>DeserializableResultset::into_typed()</code> and <code>DeserializableRow::into_typed()</code>.
//!
//! By this extension of the driver's API, the functionality of serde_db canbe provided
//! to the users of a DB driver without the need to
//! import DeserializableResultset or DeserializableRow.
//!
//! It depends on the dimension of the resultset what target data structure you can
//! choose for deserialization:
//!
//! * You can always use a <code>Vec&lt;line_struct&gt;</code>, where
//!   <code>line_struct</code> matches the field list of the resultset.
//!
//! * If the resultset contains only a single line (e.g. because you specified
//!   TOP 1 in your select),
//!   then you can optionally choose to deserialize into a plain <code>line_struct</code>.
//!
//! * If the resultset contains only a single column, then you can optionally choose to
//!   deserialize into a <code>Vec&lt;plain_field&gt;</code>.
//!
//! * If the resultset contains only a single value (one row with one column),
//!   then you can optionally choose to deserialize into a plain <code>line_struct</code>,
//!   or a <code>Vec&lt;plain_field&gt;</code>, or a plain variable.
//!
//! # Examples
//!
//! Convert a n&#215;m resultset into a Vec of structs which implement serde::de::Deserialize:
//!
//! ```ignore
//! #[macro_use]
//! extern crate serde_derive;
//! ...
//! #[derive(Deserialize)]
//! struct MyStruct {...}
//! ...
//! let resultset = ...;
//! let data: Vec<MyStruct> = resultset.into_typed().unwrap();
//! ```
//!
//! Convert a n&#215;1 resultset into a Vec of fields:
//!
//! ```ignore
//! let vec_s: Vec<String> = resultset.into_typed().unwrap();
//! ```
//!
//! Convert a 1&#215;1 resultset into a single field:
//!
//! ```ignore
//! let s: String = resultset.into_typed().unwrap();
//! ```
//!
//! Loop over rows, convert each row individually into a struct
//! (for better streaming support with large result sets):
//!
//! ```ignore
//! for row in resultset {
//!     let data: MyStruct = row.into_typed().unwrap();
//! }
//! ```
//!
//! Or convert the rows into tuples (no need to derive serde::de::Deserialize):
//!
//! ```ignore
//! for row in resultset {
//!     let t: (String, NaiveDateTime, i32, Option<i32>) = row.into_typed().unwrap();
//! }
//! ```
//!
//! FIXME Add example for single field evaluation
//!

mod db_value;
mod conversion_error;
mod deserializable_resultset;
mod deserializable_row;
mod deserialization_error;
mod field_deserializer;
pub mod row;
mod row_deserializer;
mod rs_deserializer;

pub use de::conversion_error::ConversionError;
pub use self::deserialization_error::{DeserError, DeserResult};

pub use self::db_value::{DbValue, DbValueInto};
pub use self::deserializable_resultset::DeserializableResultset;
pub use self::deserializable_row::DeserializableRow;
