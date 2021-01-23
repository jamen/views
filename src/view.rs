use std::ops::{RangeBounds,Bound,Deref};
use std::slice;
use std::fmt::{self,Debug,Formatter};
use std::mem;
use std::sync::Arc;

use crate::OutOfBounds;

#[derive(Clone)]
pub struct View<T, B: AsRef<[T]> + Clone> {
    buf: B,
    ptr: *const T,
    len: usize,
}
pub type Bytes = View<u8, Arc<[u8]>>;

impl<T, B: AsRef<[T]> + Clone> AsRef<[T]> for View<T, B> {
    fn as_ref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T, B: AsRef<[T]> + Clone> Deref for View<T, B> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: Debug, B: AsRef<[T]> + Clone> Debug for View<T, B> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        self.as_ref().fmt(fmt)
    }
}

impl<T, B: AsRef<[T]> + Clone> View<T, B> {
    pub fn new(buf: B) -> Self {
        let slice = buf.as_ref();
        let ptr = slice.as_ptr();
        let len = slice.len();
        Self { buf, ptr, len }
    }

    pub fn slice(&self, range: impl RangeBounds<usize>) -> Result<Self, OutOfBounds> {
        let begin = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n.checked_add(1).ok_or(OutOfBounds)?,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&n) => n.checked_add(1).ok_or(OutOfBounds)?,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => self.len,
        };

        if begin > end || end > self.len {
            return Err(OutOfBounds)
        }

        let buf = self.buf.clone();
        let len = end - begin;
        let ptr = unsafe { self.ptr.add(begin) };

        Ok(View { buf, ptr, len })
    }

    pub fn take(&mut self, len: usize) -> Result<Self, OutOfBounds> {
        let buf = self.buf.clone();
        let ptr = self.ptr;

        if len > self.len {
            return Err(OutOfBounds)
        }

        self.len = self.len - len;
        self.ptr = unsafe { self.ptr.add(len) };

        Ok(View { buf, ptr, len })
    }
}

macro_rules! take_int {
    ($self:ident, $typ:tt::$conv:tt) => {
        {
            const SIZE: usize = mem::size_of::<$typ>();
            let bytes = $self.take(SIZE)?;
            let arr = unsafe { *(bytes.as_ptr() as *const [u8; SIZE]) };
            Ok($typ::$conv(arr))
        }
    }
}

impl<B: AsRef<[u8]> + Clone> View<u8, B> {
    pub fn take_u8(&mut self) -> Result<u8, OutOfBounds> {
        Ok(self.take(1)?[0])
    }

    pub fn take_i8(&mut self) -> Result<i8, OutOfBounds> {
        Ok(self.take_u8()? as i8)
    }

    pub fn take_u16_le(&mut self) -> Result<u16, OutOfBounds> {
        take_int!(self, u16::from_le_bytes)
    }

    pub fn take_u16_be(&mut self) -> Result<u16, OutOfBounds> {
        take_int!(self, u16::from_be_bytes)
    }

    pub fn take_u16_ne(&mut self) -> Result<u16, OutOfBounds> {
        take_int!(self, u16::from_ne_bytes)
    }

    pub fn take_i16_le(&mut self) -> Result<i16, OutOfBounds> {
        take_int!(self, i16::from_le_bytes)
    }

    pub fn take_i16_be(&mut self) -> Result<i16, OutOfBounds> {
        take_int!(self, i16::from_be_bytes)
    }

    pub fn take_i16_ne(&mut self) -> Result<i16, OutOfBounds> {
        take_int!(self, i16::from_ne_bytes)
    }

    pub fn take_u32_le(&mut self) -> Result<u32, OutOfBounds> {
        take_int!(self, u32::from_le_bytes)
    }

    pub fn take_u32_be(&mut self) -> Result<u32, OutOfBounds> {
        take_int!(self, u32::from_be_bytes)
    }

    pub fn take_u32_ne(&mut self) -> Result<u32, OutOfBounds> {
        take_int!(self, u32::from_ne_bytes)
    }

    pub fn take_i32_le(&mut self) -> Result<i32, OutOfBounds> {
        take_int!(self, i32::from_le_bytes)
    }

    pub fn take_i32_be(&mut self) -> Result<i32, OutOfBounds> {
        take_int!(self, i32::from_be_bytes)
    }

    pub fn take_i32_ne(&mut self) -> Result<i32, OutOfBounds> {
        take_int!(self, i32::from_ne_bytes)
    }

    pub fn take_u64_le(&mut self) -> Result<u64, OutOfBounds> {
        take_int!(self, u64::from_le_bytes)
    }

    pub fn take_u64_be(&mut self) -> Result<u64, OutOfBounds> {
        take_int!(self, u64::from_be_bytes)
    }

    pub fn take_u64_ne(&mut self) -> Result<u64, OutOfBounds> {
        take_int!(self, u64::from_ne_bytes)
    }

    pub fn take_i64_le(&mut self) -> Result<i64, OutOfBounds> {
        take_int!(self, i64::from_le_bytes)
    }

    pub fn take_i64_be(&mut self) -> Result<i64, OutOfBounds> {
        take_int!(self, i64::from_be_bytes)
    }

    pub fn take_i64_ne(&mut self) -> Result<i64, OutOfBounds> {
        take_int!(self, i64::from_ne_bytes)
    }

    pub fn take_u128_le(&mut self) -> Result<u128, OutOfBounds> {
        take_int!(self, u128::from_le_bytes)
    }

    pub fn take_u128_be(&mut self) -> Result<u128, OutOfBounds> {
        take_int!(self, u128::from_be_bytes)
    }

    pub fn take_u128_ne(&mut self) -> Result<u128, OutOfBounds> {
        take_int!(self, u128::from_ne_bytes)
    }

    pub fn take_i128_le(&mut self) -> Result<i128, OutOfBounds> {
        take_int!(self, i128::from_le_bytes)
    }

    pub fn take_i128_be(&mut self) -> Result<i128, OutOfBounds> {
        take_int!(self, i128::from_be_bytes)
    }

    pub fn take_i128_ne(&mut self) -> Result<i128, OutOfBounds> {
        take_int!(self, i128::from_ne_bytes)
    }

    pub fn take_f32_le(&mut self) -> Result<f32, OutOfBounds> {
        take_int!(self, f32::from_le_bytes)
    }

    pub fn take_f64_le(&mut self) -> Result<f64, OutOfBounds> {
        take_int!(self, f64::from_le_bytes)
    }

    pub fn take_until_nul(&mut self) -> Result<Self, OutOfBounds> {
        let len = self.iter().take_while(|x| **x != b'\0').count();
        let out = self.take(len);
        let _ = self.take(1)?;
        out
    }
}