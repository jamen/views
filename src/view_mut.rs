use std::ops::{Deref,DerefMut};
use std::fmt::{self,Debug,Formatter};
use std::mem;
use std::sync::Arc;
use std::marker::PhantomData;

use crate::OutOfBounds;

pub struct ViewMut<T, B: AsRef<[T]> + AsMut<[T]>> {
    buf: B,
    phantom: PhantomData<T>
}

pub type BytesMut = ViewMut<u8, Arc<Vec<u8>>>;

impl<T, B: AsRef<[T]> + AsMut<[T]>> AsRef<[T]> for ViewMut<T, B> {
    fn as_ref(&self) -> &[T] {
        self.buf.as_ref()
    }
}

impl<T, B: AsRef<[T]> + AsMut<[T]>> AsMut<[T]> for ViewMut<T, B> {
    fn as_mut(&mut self) -> &mut [T] {
        self.buf.as_mut()
    }
}

impl<T, B: AsRef<[T]> + AsMut<[T]>> Deref for ViewMut<T, B> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T, B: AsRef<[T]> + AsMut<[T]>> DerefMut for ViewMut<T, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: Debug, B: AsRef<[T]> + AsMut<[T]>> Debug for ViewMut<T, B> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        self.as_ref().fmt(fmt)
    }
}

impl<T, B: AsRef<[T]> + AsMut<[T]>> ViewMut<T, B> {
    pub fn new(buf: B) -> Self {
        Self { buf, phantom: PhantomData::default() }
    }
}

macro_rules! put_int {
    ($self:ident, $val:ident, $typ:tt::$conv:tt) => {
        {
            const SIZE: usize = mem::size_of::<$typ>();
            let bytes = $self.as_mut();
            if bytes.len() < SIZE { return Err(OutOfBounds) }
            let bytes = bytes.as_mut_ptr() as *mut [u8; SIZE];
            unsafe { *bytes = $typ::$conv($val); }
            Ok(())
        }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> ViewMut<u8, B> {
    pub fn put_u16_le(&mut self, val: u16) -> Result<(), OutOfBounds> {
        put_int!(self, val, u16::to_le_bytes)
    }

    pub fn put_u16_be(&mut self, val: u16) -> Result<(), OutOfBounds> {
        put_int!(self, val, u16::to_be_bytes)
    }

    pub fn put_u16_ne(&mut self, val: u16) -> Result<(), OutOfBounds> {
        put_int!(self, val, u16::to_ne_bytes)
    }

    pub fn put_i16_le(&mut self, val: i16) -> Result<(), OutOfBounds> {
        put_int!(self, val, i16::to_le_bytes)
    }

    pub fn put_i16_be(&mut self, val: i16) -> Result<(), OutOfBounds> {
        put_int!(self, val, i16::to_be_bytes)
    }

    pub fn put_i16_ne(&mut self, val: i16) -> Result<(), OutOfBounds> {
        put_int!(self, val, i16::to_ne_bytes)
    }

    pub fn put_u32_le(&mut self, val: u32) -> Result<(), OutOfBounds> {
        put_int!(self, val, u32::to_le_bytes)
    }

    pub fn put_u32_be(&mut self, val: u32) -> Result<(), OutOfBounds> {
        put_int!(self, val, u32::to_be_bytes)
    }

    pub fn put_u32_ne(&mut self, val: u32) -> Result<(), OutOfBounds> {
        put_int!(self, val, u32::to_ne_bytes)
    }

    pub fn put_i32_le(&mut self, val: i32) -> Result<(), OutOfBounds> {
        put_int!(self, val, i32::to_le_bytes)
    }

    pub fn put_i32_be(&mut self, val: i32) -> Result<(), OutOfBounds> {
        put_int!(self, val, i32::to_be_bytes)
    }

    pub fn put_i32_ne(&mut self, val: i32) -> Result<(), OutOfBounds> {
        put_int!(self, val, i32::to_ne_bytes)
    }

    pub fn put_u64_le(&mut self, val: u64) -> Result<(), OutOfBounds> {
        put_int!(self, val, u64::to_le_bytes)
    }

    pub fn put_u64_be(&mut self, val: u64) -> Result<(), OutOfBounds> {
        put_int!(self, val, u64::to_be_bytes)
    }

    pub fn put_u64_ne(&mut self, val: u64) -> Result<(), OutOfBounds> {
        put_int!(self, val, u64::to_ne_bytes)
    }

    pub fn put_i64_le(&mut self, val: i64) -> Result<(), OutOfBounds> {
        put_int!(self, val, i64::to_le_bytes)
    }

    pub fn put_i64_be(&mut self, val: i64) -> Result<(), OutOfBounds> {
        put_int!(self, val, i64::to_be_bytes)
    }

    pub fn put_i64_ne(&mut self, val: i64) -> Result<(), OutOfBounds> {
        put_int!(self, val, i64::to_ne_bytes)
    }

    pub fn put_u128_le(&mut self, val: u128) -> Result<(), OutOfBounds> {
        put_int!(self, val, u128::to_le_bytes)
    }

    pub fn put_u128_be(&mut self, val: u128) -> Result<(), OutOfBounds> {
        put_int!(self, val, u128::to_be_bytes)
    }

    pub fn put_u128_ne(&mut self, val: u128) -> Result<(), OutOfBounds> {
        put_int!(self, val, u128::to_ne_bytes)
    }

    pub fn put_i128_le(&mut self, val: i128) -> Result<(), OutOfBounds> {
        put_int!(self, val, i128::to_le_bytes)
    }

    pub fn put_i128_be(&mut self, val: i128) -> Result<(), OutOfBounds> {
        put_int!(self, val, i128::to_be_bytes)
    }

    pub fn put_i128_ne(&mut self, val: i128) -> Result<(), OutOfBounds> {
        put_int!(self, val, i128::to_ne_bytes)
    }

    pub fn put_f32_le(&mut self, val: f32) -> Result<(), OutOfBounds> {
        put_int!(self, val, f32::to_le_bytes)
    }

    pub fn put_f64_le(&mut self, val: f64) -> Result<(), OutOfBounds> {
        put_int!(self, val, f64::to_le_bytes)
    }
}