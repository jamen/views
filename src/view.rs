use core::slice;
use core::mem;

use crate::BadPos;

pub trait View<T>: AsRef<[T]> {
    fn take(&mut self, n: usize) -> Result<&[T], BadPos>;
}

impl<T> View<T> for &[T] {
    fn take(&mut self, n: usize) -> Result<&[T], BadPos> {
        let len = self.len();
        if n > len { return Err(BadPos) }
        let out = unsafe { slice::from_raw_parts(self.as_ptr(), n) };
        *self = unsafe { slice::from_raw_parts(self.as_ptr().add(n), len - n) };
        Ok(out)
    }
}

impl<T> View<T> for &mut [T] {
    fn take(&mut self, n: usize) -> Result<&[T], BadPos> {
        let len = self.len();
        if n > len { return Err(BadPos) }
        let out = unsafe { slice::from_raw_parts(self.as_ptr(), n) };
        *self = unsafe { slice::from_raw_parts_mut(self.as_mut_ptr().add(n), len - n) };
        Ok(out)
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

pub trait Bytes: View<u8> {
    fn take_u8(&mut self) -> Result<u8, BadPos> {
        Ok(self.take(1)?[0])
    }

    fn take_i8(&mut self) -> Result<i8, BadPos> {
        Ok(self.take_u8()? as i8)
    }

    fn take_u16_le(&mut self) -> Result<u16, BadPos> {
        take_int!(self, u16::from_le_bytes)
    }

    fn take_u16_be(&mut self) -> Result<u16, BadPos> {
        take_int!(self, u16::from_be_bytes)
    }

    fn take_u16_ne(&mut self) -> Result<u16, BadPos> {
        take_int!(self, u16::from_ne_bytes)
    }

    fn take_i16_le(&mut self) -> Result<i16, BadPos> {
        take_int!(self, i16::from_le_bytes)
    }

    fn take_i16_be(&mut self) -> Result<i16, BadPos> {
        take_int!(self, i16::from_be_bytes)
    }

    fn take_i16_ne(&mut self) -> Result<i16, BadPos> {
        take_int!(self, i16::from_ne_bytes)
    }

    fn take_u32_le(&mut self) -> Result<u32, BadPos> {
        take_int!(self, u32::from_le_bytes)
    }

    fn take_u32_be(&mut self) -> Result<u32, BadPos> {
        take_int!(self, u32::from_be_bytes)
    }

    fn take_u32_ne(&mut self) -> Result<u32, BadPos> {
        take_int!(self, u32::from_ne_bytes)
    }

    fn take_i32_le(&mut self) -> Result<i32, BadPos> {
        take_int!(self, i32::from_le_bytes)
    }

    fn take_i32_be(&mut self) -> Result<i32, BadPos> {
        take_int!(self, i32::from_be_bytes)
    }

    fn take_i32_ne(&mut self) -> Result<i32, BadPos> {
        take_int!(self, i32::from_ne_bytes)
    }

    fn take_u64_le(&mut self) -> Result<u64, BadPos> {
        take_int!(self, u64::from_le_bytes)
    }

    fn take_u64_be(&mut self) -> Result<u64, BadPos> {
        take_int!(self, u64::from_be_bytes)
    }

    fn take_u64_ne(&mut self) -> Result<u64, BadPos> {
        take_int!(self, u64::from_ne_bytes)
    }

    fn take_i64_le(&mut self) -> Result<i64, BadPos> {
        take_int!(self, i64::from_le_bytes)
    }

    fn take_i64_be(&mut self) -> Result<i64, BadPos> {
        take_int!(self, i64::from_be_bytes)
    }

    fn take_i64_ne(&mut self) -> Result<i64, BadPos> {
        take_int!(self, i64::from_ne_bytes)
    }

    fn take_u128_le(&mut self) -> Result<u128, BadPos> {
        take_int!(self, u128::from_le_bytes)
    }

    fn take_u128_be(&mut self) -> Result<u128, BadPos> {
        take_int!(self, u128::from_be_bytes)
    }

    fn take_u128_ne(&mut self) -> Result<u128, BadPos> {
        take_int!(self, u128::from_ne_bytes)
    }

    fn take_i128_le(&mut self) -> Result<i128, BadPos> {
        take_int!(self, i128::from_le_bytes)
    }

    fn take_i128_be(&mut self) -> Result<i128, BadPos> {
        take_int!(self, i128::from_be_bytes)
    }

    fn take_i128_ne(&mut self) -> Result<i128, BadPos> {
        take_int!(self, i128::from_ne_bytes)
    }

    fn take_f32_le(&mut self) -> Result<f32, BadPos> {
        take_int!(self, f32::from_le_bytes)
    }

    fn take_f32_be(&mut self) -> Result<f32, BadPos> {
        take_int!(self, f32::from_be_bytes)
    }

    fn take_f32_ne(&mut self) -> Result<f32, BadPos> {
        take_int!(self, f32::from_ne_bytes)
    }

    fn take_f64_le(&mut self) -> Result<f64, BadPos> {
        take_int!(self, f64::from_le_bytes)
    }

    fn take_f64_be(&mut self) -> Result<f64, BadPos> {
        take_int!(self, f64::from_be_bytes)
    }

    fn take_f64_ne(&mut self) -> Result<f64, BadPos> {
        take_int!(self, f64::from_ne_bytes)
    }

    /// Invalid UTF8 is considered an invalid position and out of bounds.
    fn take_as_str(&mut self, n: usize) -> Result<&str, BadPos> {
        core::str::from_utf8(self.take(n)?).map_err(|_| BadPos)
    }

    fn take_until_nul(&mut self) -> Result<&[u8], BadPos> {
        let len = self.as_ref().iter().take_while(|x| **x != b'\0').count();
        let out = self.take(len + 1)?;
        let out = &out[..len];
        Ok(out)
    }

    fn take_as_str_until_nul(&mut self) -> Result<&str, BadPos> {
        let out = self.take_until_nul()?;
        let out = core::str::from_utf8(out).map_err(|_| BadPos)?;
        Ok(out)
    }
}

impl Bytes for &[u8] {}
impl Bytes for &mut [u8] {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take() {
        let mut a = &[2,0,0,0,b'H',b'i',b'\0'][..];

        let out = a.take(0);
        assert!(out == Ok(&[]), "take none");
        assert!(a == &[2,0,0,0,b'H',b'i',b'\0'][..], "took none");

        let four = a.take(4);
        assert!(four == Ok(&[2,0,0,0]), "take 4");
        assert!(a == &[b'H',b'i',b'\0'][..]);

        let four_err = a.take(4);
        assert!(four_err.is_err(), "take 4 out of bounds");
        assert!(a == &[b'H',b'i',b'\0'][..], "took none");

        let rest = a.take(3);
        assert!(rest == Ok(&[b'H',b'i',b'\0']), "take rest");
        assert!(a == &[], "took rest");

        let one_err = a.take(1);
        assert!(one_err.is_err(), "take 1 out of bounds");
        assert!(a == &[], "took none");
    }

    #[test]
    fn test_take_u8() {
        let mut a: &[u8] = &[0,1];

        let b = a.take_u8();
        assert!(b == Ok(0));
        assert!(a == &[1]);

        let c = a.take_u8();
        assert!(c == Ok(1));
        assert!(a == &[]);

        let d = a.take_u8();
        assert!(d.is_err());
        assert!(a == &[]);
    }

    #[test]
    fn test_take_i8() {
        let mut a: &[u8] = &[0,1,-1i8 as u8];

        let b = a.take_i8();
        assert!(b == Ok(0));
        assert!(a == &[1,-1i8 as u8]);

        let c = a.take_i8();
        assert!(c == Ok(1));
        assert!(a == &[-1i8 as u8]);

        let d = a.take_i8();
        assert!(d == Ok(-1));
        assert!(a == &[]);

        let e = a.take_i8();
        assert!(e.is_err());
        assert!(a == &[]);
    }

    #[test]
    fn test_take_u16() {
        let mut a = &[1u16.to_le_bytes(),u16::MAX.to_le_bytes()].concat()[..];

        let b = a.take_u16_le();
        assert!(b == Ok(1));
        assert!(a == &u16::MAX.to_le_bytes()[..]);

        let c = a.take_u16_le();
        assert!(c == Ok(u16::MAX));
        assert!(a == &[]);

        let mut a = &[1u16.to_be_bytes(),u16::MAX.to_be_bytes()].concat()[..];

        let b = a.take_u16_be();
        assert!(b == Ok(1));
        assert!(a == &u16::MAX.to_be_bytes()[..]);

        let c = a.take_u16_be();
        assert!(c == Ok(u16::MAX));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_i16() {
        let mut a = &[1i16.to_le_bytes(),(-1i16).to_le_bytes()].concat()[..];

        let b = a.take_i16_le();
        assert!(b == Ok(1));
        assert!(a == &(-1i16).to_le_bytes()[..]);

        let c = a.take_i16_le();
        assert!(c == Ok(-1));
        assert!(a == &[]);

        let mut a = &[1i16.to_be_bytes(),(-1i16).to_be_bytes()].concat()[..];

        let b = a.take_i16_be();
        assert!(b == Ok(1));
        assert!(a == &(-1i16).to_be_bytes()[..]);

        let c = a.take_i16_be();
        assert!(c == Ok(-1));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_u32() {
        let mut a = &[1u32.to_le_bytes(),u32::MAX.to_le_bytes()].concat()[..];

        let b = a.take_u32_le();
        assert!(b == Ok(1));
        assert!(a == &u32::MAX.to_le_bytes()[..]);

        let c = a.take_u32_le();
        assert!(c == Ok(u32::MAX));
        assert!(a == &[]);

        let mut a = &[1u32.to_be_bytes(),u32::MAX.to_be_bytes()].concat()[..];

        let b = a.take_u32_be();
        assert!(b == Ok(1));
        assert!(a == &u32::MAX.to_be_bytes()[..]);

        let c = a.take_u32_be();
        assert!(c == Ok(u32::MAX));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_i32() {
        let mut a = &[1i32.to_le_bytes(),(-1i32).to_le_bytes()].concat()[..];

        let b = a.take_i32_le();
        assert!(b == Ok(1));
        assert!(a == &(-1i32).to_le_bytes()[..]);

        let c = a.take_i32_le();
        assert!(c == Ok(-1));
        assert!(a == &[]);

        let mut a = &[1i32.to_be_bytes(),(-1i32).to_be_bytes()].concat()[..];

        let b = a.take_i32_be();
        assert!(b == Ok(1));
        assert!(a == &(-1i32).to_be_bytes()[..]);

        let c = a.take_i32_be();
        assert!(c == Ok(-1));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_u64() {
        let mut a = &[1u64.to_le_bytes(),u64::MAX.to_le_bytes()].concat()[..];

        let b = a.take_u64_le();
        assert!(b == Ok(1));
        assert!(a == &u64::MAX.to_le_bytes()[..]);

        let c = a.take_u64_le();
        assert!(c == Ok(u64::MAX));
        assert!(a == &[]);

        let mut a = &[1u64.to_be_bytes(),u64::MAX.to_be_bytes()].concat()[..];

        let b = a.take_u64_be();
        assert!(b == Ok(1));
        assert!(a == &u64::MAX.to_be_bytes()[..]);

        let c = a.take_u64_be();
        assert!(c == Ok(u64::MAX));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_i64() {
        let mut a = &[1i64.to_le_bytes(),(-1i64).to_le_bytes()].concat()[..];

        let b = a.take_i64_le();
        assert!(b == Ok(1));
        assert!(a == &(-1i64).to_le_bytes()[..]);

        let c = a.take_i64_le();
        assert!(c == Ok(-1));
        assert!(a == &[]);

        let mut a = &[1i64.to_be_bytes(),(-1i64).to_be_bytes()].concat()[..];

        let b = a.take_i64_be();
        assert!(b == Ok(1));
        assert!(a == &(-1i64).to_be_bytes()[..]);

        let c = a.take_i64_be();
        assert!(c == Ok(-1));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_u128() {
        let mut a = &[1u128.to_le_bytes(),u128::MAX.to_le_bytes()].concat()[..];

        let b = a.take_u128_le();
        assert!(b == Ok(1));
        assert!(a == &u128::MAX.to_le_bytes()[..]);

        let c = a.take_u128_le();
        assert!(c == Ok(u128::MAX));
        assert!(a == &[]);

        let mut a = &[1u128.to_be_bytes(),u128::MAX.to_be_bytes()].concat()[..];

        let b = a.take_u128_be();
        assert!(b == Ok(1));
        assert!(a == &u128::MAX.to_be_bytes()[..]);

        let c = a.take_u128_be();
        assert!(c == Ok(u128::MAX));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_i128() {
        let mut a = &[1i128.to_le_bytes(),(-1i128).to_le_bytes()].concat()[..];

        let b = a.take_i128_le();
        assert!(b == Ok(1));
        assert!(a == &(-1i128).to_le_bytes()[..]);

        let c = a.take_i128_le();
        assert!(c == Ok(-1));
        assert!(a == &[]);

        let mut a = &[1i128.to_be_bytes(),(-1i128).to_be_bytes()].concat()[..];

        let b = a.take_i128_be();
        assert!(b == Ok(1));
        assert!(a == &(-1i128).to_be_bytes()[..]);

        let c = a.take_i128_be();
        assert!(c == Ok(-1));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_f32() {
        let mut a = &[1f32.to_le_bytes(),(-1f32).to_le_bytes()].concat()[..];

        let b = a.take_f32_le();
        assert!(b == Ok(1f32));
        assert!(a == &(-1f32).to_le_bytes()[..]);

        let c = a.take_f32_le();
        assert!(c == Ok(-1f32));
        assert!(a == &[]);

        let mut a = &[1f32.to_be_bytes(),(-1f32).to_be_bytes()].concat()[..];

        let b = a.take_f32_be();
        assert!(b == Ok(1f32));
        assert!(a == &(-1f32).to_be_bytes()[..]);

        let c = a.take_f32_be();
        assert!(c == Ok(-1f32));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_f64() {
        let mut a = &[1f64.to_le_bytes(),(-1f64).to_le_bytes()].concat()[..];

        let b = a.take_f64_le();
        assert!(b == Ok(1f64));
        assert!(a == &(-1f64).to_le_bytes()[..]);

        let c = a.take_f64_le();
        assert!(c == Ok(-1f64));
        assert!(a == &[]);

        let mut a = &[1f64.to_be_bytes(),(-1f64).to_be_bytes()].concat()[..];

        let b = a.take_f64_be();
        assert!(b == Ok(1f64));
        assert!(a == &(-1f64).to_be_bytes()[..]);

        let c = a.take_f64_be();
        assert!(c == Ok(-1f64));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_until_nul() {
        let mut a = &[b'H',b'i',b'\0'][..];
        let b = a.take_until_nul();
        assert!(b == Ok(&[b'H',b'i']));
        assert!(a == &[]);
    }
}