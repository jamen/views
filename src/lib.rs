//! Views into a buffer.
//!
//! See [`View`](./struct.View.html) and [`ViewMut`](./struct.View.html). There's also the aliases [`Bytes`](./struct.Bytes.html) and [`BytesMut`](./struct.BytesMut.html) for byte buffers.
//!
//! This is mostly inspired from [`bytes::Bytes`](https://docs.rs/bytes/1.0.1/bytes/struct.Bytes.html). Some notable dfferences with this crate are the buffer and element types are generic, there is bounds checking, and no dynamic dispatch.

/// An error for an out of bounds operation.
#[derive(Debug,PartialEq,Eq)]
pub struct OutOfBounds;

mod view;
mod view_mut;

pub use view::*;
pub use view_mut::*;