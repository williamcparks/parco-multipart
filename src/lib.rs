#![doc = include_str!("../README.md")]

mod boundaries;
mod error;
mod message;
mod stream_message;
mod try_parse;

pub use error::MultiPartError;
pub use message::{Message, Part};
pub use mime;
pub use stream_message::StreamMessage;

#[cfg(test)]
mod test;
