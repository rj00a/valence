//! Support for serializing and deserializing compounds in Java edition's binary
//! format.
//!
//! # Examples
//!
//! ```
//! use valence_nbt::{compound, to_binary, Compound, List};
//!
//! let c = compound! {
//!     "byte" => 5_i8,
//!     "string" => "hello",
//!     "list_of_float" => List::Float(vec![
//!         3.1415,
//!         2.7182,
//!         1.4142
//!     ]),
//! };
//!
//! let mut buf = vec![];
//!
//! to_binary(&mut buf, "", &c).unwrap();
//! ```
//!
//! Decode NBT data from its binary form.
//!
//! ```
//! use valence_nbt::{compound, from_binary, Compound, Value};
//!
//! let some_bytes = [10, 0, 0, 3, 0, 3, 105, 110, 116, 0, 0, 222, 173, 0];
//!
//! let expected_value = compound! {
//!     "int" => 0xdead
//! };
//!
//! let (root_name, nbt) = from_binary(&mut some_bytes.as_slice()).unwrap().unwrap();
//!
//! assert_eq!(nbt, Value::from(expected_value));
//! assert_eq!(root_name, "");
//! ```

mod decode;
mod encode;
mod modified_utf8;
#[cfg(test)]
mod tests;

pub use decode::*;
pub use encode::*;

use crate::Tag;

impl Tag {
    /// Returns the name of this tag for error reporting purposes.
    const fn name(self) -> &'static str {
        match self {
            Tag::End => "end",
            Tag::Byte => "byte",
            Tag::Short => "short",
            Tag::Int => "int",
            Tag::Long => "long",
            Tag::Float => "float",
            Tag::Double => "double",
            Tag::ByteArray => "byte array",
            Tag::String => "string",
            Tag::List => "list",
            Tag::Compound => "compound",
            Tag::IntArray => "int array",
            Tag::LongArray => "long array",
        }
    }
}
