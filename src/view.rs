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

impl<T: PartialEq<T>, B: AsRef<[T]> + Clone> PartialEq<View<T, B>> for View<T, B> {
    fn eq(&self, r: &Self) -> bool {
        self.as_ref() == r.as_ref()
    }
}

impl<T: Eq, B: AsRef<[T]> + Clone> Eq for View<T, B> {}

impl<'a, T> From<&'a [T]> for View<T, &'a [T]> {
    fn from(x: &'a [T]) -> Self {
        View::new(x)
    }
}

impl<'a, T> From<&'a Vec<T>> for View<T, &'a [T]> {
    fn from(x: &Vec<T>) -> View<T, &[T]> {
        View::new(&x[..])
    }
}

impl<T> From<Arc<[T]>> for View<T, Arc<[T]>> {
    fn from(x: Arc<[T]>) -> Self {
        View::new(x)
    }
}

impl<T, B: AsRef<[T]> + Clone> View<T, B> {
    pub fn new(buf: B) -> Self {
        let slice = buf.as_ref();
        let ptr = slice.as_ptr();
        let len = slice.len();
        Self { buf, ptr, len }
    }

    pub fn from<S: Into<B>>(src: S) -> Self {
        Self::new(src.into())
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
            Ok($typ::$conv(unsafe { *(bytes.as_ptr() as *const [u8; SIZE]) }))
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

    pub fn take_f32_be(&mut self) -> Result<f32, OutOfBounds> {
        take_int!(self, f32::from_be_bytes)
    }

    pub fn take_f32_ne(&mut self) -> Result<f32, OutOfBounds> {
        take_int!(self, f32::from_ne_bytes)
    }

    pub fn take_f64_le(&mut self) -> Result<f64, OutOfBounds> {
        take_int!(self, f64::from_le_bytes)
    }

    pub fn take_f64_be(&mut self) -> Result<f64, OutOfBounds> {
        take_int!(self, f64::from_be_bytes)
    }

    pub fn take_f64_ne(&mut self) -> Result<f64, OutOfBounds> {
        take_int!(self, f64::from_ne_bytes)
    }

    pub fn take_until_nul(&mut self) -> Result<Self, OutOfBounds> {
        let len = self.iter().take_while(|x| **x != b'\0').count();
        let out = self.take(len);
        let _ = self.take(1)?;
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let bytes = Bytes::from(vec![0,1,2]);
        assert!(bytes == bytes, "eq");
        assert!(bytes == bytes.clone(), "eq clone");
        assert!(Ok(bytes.clone()) == bytes.slice(..), "eq slice");
    }

    #[test]
    fn test_slice() {
        let bytes = Bytes::from(vec![0,1,2]);
        assert!(bytes.slice(4..5).is_err(), "all out of bounds");
        assert!(bytes.slice(1..4).is_err(), "overflow");
        assert!(bytes.slice(2..1).is_err(), "inverted");
        assert!(bytes.slice(4..1).is_err(), "inverted overflow");
        assert!(bytes.slice(4..).is_err(), "start out of bounds");
        assert!(bytes.slice(..4).is_err(), "end out of bounds");
        assert!(bytes.slice(1..2).is_ok(), "all in bounds");
        assert!(bytes.slice(1..).is_ok(), "start in bounds");
        assert!(bytes.slice(..2).is_ok(), "end in bounds");
        assert!(bytes.slice(..).is_ok(), "whole");
    }

    #[derive(Clone,PartialEq,Eq)]
    struct S (u32, u32);

    #[test]
    fn test_non_byte_data() {
        let a = vec![ S(0,1), S(1,2), S(2,3) ];
        let b = vec![ S(1,2), S(2,3) ];
        assert!(View::new(a).slice(1..) == Ok(View::new(b)), "slice eq")
    }

    #[test]
    fn test_take() {
        let mut a = Bytes::from(vec![2,0,0,0,b'H',b'i',b'\0']);

        let out = a.take(0);
        assert!(out == Ok(Bytes::from(vec![])), "take none");
        assert!(a == Bytes::from(vec![2,0,0,0,b'H',b'i',b'\0']), "took none");

        let four = a.take(4);
        assert!(four == Ok(Bytes::from(vec![2,0,0,0])), "take 4");
        assert!(a == Bytes::from(vec![b'H',b'i',b'\0']));

        let four_err = a.take(4);
        assert!(four_err.is_err(), "take 4 out of bounds");
        assert!(a == Bytes::from(vec![b'H',b'i',b'\0']), "took none");

        let rest = a.take(3);
        assert!(rest == Ok(Bytes::from(vec![b'H',b'i',b'\0'])), "take rest");
        assert!(a == Bytes::from(vec![]), "took rest");

        let one_err = a.take(1);
        assert!(one_err.is_err(), "take 1 out of bounds");
        assert!(a == Bytes::from(vec![]), "took none");
    }

    #[test]
    fn test_take_u8() {
        let mut a = Bytes::from(vec![0,1]);

        let b = a.take_u8();
        assert!(b == Ok(0));
        assert!(a == Bytes::from(vec![1]));

        let c = a.take_u8();
        assert!(c == Ok(1));
        assert!(a == Bytes::from(vec![]));

        let d = a.take_u8();
        assert!(d.is_err());
        assert!(a == Bytes::from(vec![]));
    }

    #[test]
    fn test_take_i8() {
        let mut a = Bytes::from(vec![0,1,-1i8 as u8]);

        let b = a.take_i8();
        assert!(b == Ok(0));
        assert!(a == Bytes::from(vec![1,-1i8 as u8]));

        let c = a.take_i8();
        assert!(c == Ok(1));
        assert!(a == Bytes::from(vec![-1i8 as u8]));

        let d = a.take_i8();
        assert!(d == Ok(-1));
        assert!(a == Bytes::from(vec![]));

        let e = a.take_i8();
        assert!(e.is_err());
        assert!(a == Bytes::from(vec![]));
    }

    #[test]
    fn test_take_u16() {
        let mut a = Bytes::from([1u16.to_le_bytes(),u16::MAX.to_le_bytes()].concat());

        let b = a.take_u16_le();
        assert!(b == Ok(1));
        assert!(a == Bytes::from(u16::MAX.to_le_bytes().to_vec()));

        let c = a.take_u16_le();
        assert!(c == Ok(u16::MAX));
        assert!(a == Bytes::from(vec![]));

        let mut a = Bytes::from([1u16.to_be_bytes(),u16::MAX.to_be_bytes()].concat());

        let b = a.take_u16_be();
        assert!(b == Ok(1));
        assert!(a == Bytes::from(u16::MAX.to_be_bytes().to_vec()));

        let c = a.take_u16_be();
        assert!(c == Ok(u16::MAX));
        assert!(a == Bytes::from(vec![]));
    }

    #[test]
    fn test_take_i16() {
        let mut a = Bytes::from([1i16.to_le_bytes(),(-1i16).to_le_bytes()].concat());

        let b = a.take_i16_le();
        assert!(b == Ok(1));
        assert!(a == Bytes::from((-1i16).to_le_bytes().to_vec()));

        let c = a.take_i16_le();
        assert!(c == Ok(-1));
        assert!(a == Bytes::from(vec![]));

        let mut a = Bytes::from([1i16.to_be_bytes(),(-1i16).to_be_bytes()].concat());

        let b = a.take_i16_be();
        assert!(b == Ok(1));
        assert!(a == Bytes::from((-1i16).to_be_bytes().to_vec()));

        let c = a.take_i16_be();
        assert!(c == Ok(-1));
        assert!(a == Bytes::from(vec![]));
    }

    #[test]
    fn test_take_u32() {
        let mut a = Bytes::from([1u32.to_le_bytes(),u32::MAX.to_le_bytes()].concat());

        let b = a.take_u32_le();
        assert!(b == Ok(1));
        assert!(a == Bytes::from(u32::MAX.to_le_bytes().to_vec()));

        let c = a.take_u32_le();
        assert!(c == Ok(u32::MAX));
        assert!(a == Bytes::from(vec![]));

        let mut a = Bytes::from([1u32.to_be_bytes(),u32::MAX.to_be_bytes()].concat());

        let b = a.take_u32_be();
        assert!(b == Ok(1));
        assert!(a == Bytes::from(u32::MAX.to_be_bytes().to_vec()));

        let c = a.take_u32_be();
        assert!(c == Ok(u32::MAX));
        assert!(a == Bytes::from(vec![]));
    }

    #[test]
    fn test_take_i32() {
        let mut a = Bytes::from([1i32.to_le_bytes(),(-1i32).to_le_bytes()].concat());

        let b = a.take_i32_le();
        assert!(b == Ok(1));
        assert!(a == Bytes::from((-1i32).to_le_bytes().to_vec()));

        let c = a.take_i32_le();
        assert!(c == Ok(-1));
        assert!(a == Bytes::from(vec![]));

        let mut a = Bytes::from([1i32.to_be_bytes(),(-1i32).to_be_bytes()].concat());

        let b = a.take_i32_be();
        assert!(b == Ok(1));
        assert!(a == Bytes::from((-1i32).to_be_bytes().to_vec()));

        let c = a.take_i32_be();
        assert!(c == Ok(-1));
        assert!(a == Bytes::from(vec![]));
    }

    #[test]
    fn test_take_u64() {
        let mut a = Bytes::from([1u64.to_le_bytes(),u64::MAX.to_le_bytes()].concat());

        let b = a.take_u64_le();
        assert!(b == Ok(1));
        assert!(a == Bytes::from(u64::MAX.to_le_bytes().to_vec()));

        let c = a.take_u64_le();
        assert!(c == Ok(u64::MAX));
        assert!(a == Bytes::from(vec![]));

        let mut a = Bytes::from([1u64.to_be_bytes(),u64::MAX.to_be_bytes()].concat());

        let b = a.take_u64_be();
        assert!(b == Ok(1));
        assert!(a == Bytes::from(u64::MAX.to_be_bytes().to_vec()));

        let c = a.take_u64_be();
        assert!(c == Ok(u64::MAX));
        assert!(a == Bytes::from(vec![]));
    }

    #[test]
    fn test_take_i64() {
        let mut a = Bytes::from([1i64.to_le_bytes(),(-1i64).to_le_bytes()].concat());

        let b = a.take_i64_le();
        assert!(b == Ok(1));
        assert!(a == Bytes::from((-1i64).to_le_bytes().to_vec()));

        let c = a.take_i64_le();
        assert!(c == Ok(-1));
        assert!(a == Bytes::from(vec![]));

        let mut a = Bytes::from([1i64.to_be_bytes(),(-1i64).to_be_bytes()].concat());

        let b = a.take_i64_be();
        assert!(b == Ok(1));
        assert!(a == Bytes::from((-1i64).to_be_bytes().to_vec()));

        let c = a.take_i64_be();
        assert!(c == Ok(-1));
        assert!(a == Bytes::from(vec![]));
    }

    #[test]
    fn test_take_u128() {
        let mut a = Bytes::from([1u128.to_le_bytes(),u128::MAX.to_le_bytes()].concat());

        let b = a.take_u128_le();
        assert!(b == Ok(1));
        assert!(a == Bytes::from(u128::MAX.to_le_bytes().to_vec()));

        let c = a.take_u128_le();
        assert!(c == Ok(u128::MAX));
        assert!(a == Bytes::from(vec![]));

        let mut a = Bytes::from([1u128.to_be_bytes(),u128::MAX.to_be_bytes()].concat());

        let b = a.take_u128_be();
        assert!(b == Ok(1));
        assert!(a == Bytes::from(u128::MAX.to_be_bytes().to_vec()));

        let c = a.take_u128_be();
        assert!(c == Ok(u128::MAX));
        assert!(a == Bytes::from(vec![]));
    }

    #[test]
    fn test_take_i128() {
        let mut a = Bytes::from([1i128.to_le_bytes(),(-1i128).to_le_bytes()].concat());

        let b = a.take_i128_le();
        assert!(b == Ok(1));
        assert!(a == Bytes::from((-1i128).to_le_bytes().to_vec()));

        let c = a.take_i128_le();
        assert!(c == Ok(-1));
        assert!(a == Bytes::from(vec![]));

        let mut a = Bytes::from([1i128.to_be_bytes(),(-1i128).to_be_bytes()].concat());

        let b = a.take_i128_be();
        assert!(b == Ok(1));
        assert!(a == Bytes::from((-1i128).to_be_bytes().to_vec()));

        let c = a.take_i128_be();
        assert!(c == Ok(-1));
        assert!(a == Bytes::from(vec![]));
    }

    #[test]
    fn test_take_f32() {
        let mut a = Bytes::from([1f32.to_le_bytes(),(-1f32).to_le_bytes()].concat());

        let b = a.take_f32_le();
        assert!(b == Ok(1f32));
        assert!(a == Bytes::from((-1f32).to_le_bytes().to_vec()));

        let c = a.take_f32_le();
        assert!(c == Ok(-1f32));
        assert!(a == Bytes::from(vec![]));

        let mut a = Bytes::from([1f32.to_be_bytes(),(-1f32).to_be_bytes()].concat());

        let b = a.take_f32_be();
        assert!(b == Ok(1f32));
        assert!(a == Bytes::from((-1f32).to_be_bytes().to_vec()));

        let c = a.take_f32_be();
        assert!(c == Ok(-1f32));
        assert!(a == Bytes::from(vec![]));
    }

    #[test]
    fn test_take_f64() {
        let mut a = Bytes::from([1f64.to_le_bytes(),(-1f64).to_le_bytes()].concat());

        let b = a.take_f64_le();
        assert!(b == Ok(1f64));
        assert!(a == Bytes::from((-1f64).to_le_bytes().to_vec()));

        let c = a.take_f64_le();
        assert!(c == Ok(-1f64));
        assert!(a == Bytes::from(vec![]));

        let mut a = Bytes::from([1f64.to_be_bytes(),(-1f64).to_be_bytes()].concat());

        let b = a.take_f64_be();
        assert!(b == Ok(1f64));
        assert!(a == Bytes::from((-1f64).to_be_bytes().to_vec()));

        let c = a.take_f64_be();
        assert!(c == Ok(-1f64));
        assert!(a == Bytes::from(vec![]));
    }

    #[test]
    fn test_take_until_nul() {
        let mut a = Bytes::from(vec![b'H',b'i',b'\0']);
        let b = a.take_until_nul();
        assert!(b == Ok(Bytes::from(vec![b'H',b'i'])));
        assert!(a == Bytes::from(vec![]));
    }
}