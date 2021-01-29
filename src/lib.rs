//! Extension to slices for buffers
//!
//! See the traits [`Bytes`](./trait.Bytes.html), [`BytesMut`](./trait.BytesMut.html) (or the generic versions [`View`](./trait.View.html), [`ViewMut`](./trait.View.html)), and the struct [`Look`](./struct.Look.html).
//!
//! See also [`bytes`](https://docs.rs/bytes), [`byteorder`](https://docs.rs/byteorder), and [`nom`](https://docs.rs/nom).

#![no_std]

extern crate core;

/// When an operation goes to a bad position. E.g. out of bounds or invalid UTF8
#[derive(Debug,PartialEq,Eq)]
pub struct BadPos;

mod look;
mod view;
mod view_mut;

pub use look::*;
pub use view::*;
pub use view_mut::*;