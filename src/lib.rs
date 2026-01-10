#![doc = include_str!("../README.md")]

mod boundaries;
mod error;
mod message;
mod try_parse;

pub use message::{Message, Part};
pub use mime;

#[cfg(test)]
mod test;
