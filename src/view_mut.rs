use core::slice;

use crate::BadPos;

pub trait ViewMut<T: Copy>: AsMut<[T]> {
    fn put(&mut self, val: &[T]) -> Result<(), BadPos>;
}

impl<T: Copy> ViewMut<T> for &mut [T] {
    fn put(&mut self, val: &[T]) -> Result<(), BadPos> {
        let n = val.len();
        let len = self.len();
        if n > len { return Err(BadPos) }
        let write = unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), n) };
        write.copy_from_slice(val);
        *self = unsafe { slice::from_raw_parts_mut(self.as_mut_ptr().add(n), len - n) };
        Ok(())
    }
}

pub trait BytesMut: ViewMut<u8> {
    fn put_u8(&mut self, val: u8) -> Result<(), BadPos> {
        self.put(&[val])
    }

    fn put_i8(&mut self, val: i8) -> Result<(), BadPos> {
        self.put(&[val as u8])
    }

    fn put_u16_le(&mut self, val: u16) -> Result<(), BadPos> {
        self.put(&u16::to_le_bytes(val))
    }

    fn put_u16_be(&mut self, val: u16) -> Result<(), BadPos> {
        self.put(&u16::to_be_bytes(val))
    }

    fn put_u16_ne(&mut self, val: u16) -> Result<(), BadPos> {
        self.put(&u16::to_ne_bytes(val))
    }

    fn put_i16_le(&mut self, val: i16) -> Result<(), BadPos> {
        self.put(&i16::to_le_bytes(val))
    }

    fn put_i16_be(&mut self, val: i16) -> Result<(), BadPos> {
        self.put(&i16::to_be_bytes(val))
    }

    fn put_i16_ne(&mut self, val: i16) -> Result<(), BadPos> {
        self.put(&i16::to_ne_bytes(val))
    }

    fn put_u32_le(&mut self, val: u32) -> Result<(), BadPos> {
        self.put(&u32::to_le_bytes(val))
    }

    fn put_u32_be(&mut self, val: u32) -> Result<(), BadPos> {
        self.put(&u32::to_be_bytes(val))
    }

    fn put_u32_ne(&mut self, val: u32) -> Result<(), BadPos> {
        self.put(&u32::to_ne_bytes(val))
    }

    fn put_i32_le(&mut self, val: i32) -> Result<(), BadPos> {
        self.put(&i32::to_le_bytes(val))
    }

    fn put_i32_be(&mut self, val: i32) -> Result<(), BadPos> {
        self.put(&i32::to_be_bytes(val))
    }

    fn put_i32_ne(&mut self, val: i32) -> Result<(), BadPos> {
        self.put(&i32::to_ne_bytes(val))
    }

    fn put_u64_le(&mut self, val: u64) -> Result<(), BadPos> {
        self.put(&u64::to_le_bytes(val))
    }

    fn put_u64_be(&mut self, val: u64) -> Result<(), BadPos> {
        self.put(&u64::to_be_bytes(val))
    }

    fn put_u64_ne(&mut self, val: u64) -> Result<(), BadPos> {
        self.put(&u64::to_ne_bytes(val))
    }

    fn put_i64_le(&mut self, val: i64) -> Result<(), BadPos> {
        self.put(&i64::to_le_bytes(val))
    }

    fn put_i64_be(&mut self, val: i64) -> Result<(), BadPos> {
        self.put(&i64::to_be_bytes(val))
    }

    fn put_i64_ne(&mut self, val: i64) -> Result<(), BadPos> {
        self.put(&i64::to_ne_bytes(val))
    }

    fn put_u128_le(&mut self, val: u128) -> Result<(), BadPos> {
        self.put(&u128::to_le_bytes(val))
    }

    fn put_u128_be(&mut self, val: u128) -> Result<(), BadPos> {
        self.put(&u128::to_be_bytes(val))
    }

    fn put_u128_ne(&mut self, val: u128) -> Result<(), BadPos> {
        self.put(&u128::to_ne_bytes(val))
    }

    fn put_i128_le(&mut self, val: i128) -> Result<(), BadPos> {
        self.put(&i128::to_le_bytes(val))
    }

    fn put_i128_be(&mut self, val: i128) -> Result<(), BadPos> {
        self.put(&i128::to_be_bytes(val))
    }

    fn put_i128_ne(&mut self, val: i128) -> Result<(), BadPos> {
        self.put(&i128::to_ne_bytes(val))
    }

    fn put_f32_le(&mut self, val: f32) -> Result<(), BadPos> {
        self.put(&f32::to_le_bytes(val))
    }

    fn put_f32_be(&mut self, val: f32) -> Result<(), BadPos> {
        self.put(&f32::to_be_bytes(val))
    }

    fn put_f32_ne(&mut self, val: f32) -> Result<(), BadPos> {
        self.put(&f32::to_ne_bytes(val))
    }

    fn put_f64_le(&mut self, val: f64) -> Result<(), BadPos> {
        self.put(&f64::to_le_bytes(val))
    }

    fn put_f64_be(&mut self, val: f64) -> Result<(), BadPos> {
        self.put(&f64::to_be_bytes(val))
    }

    fn put_f64_ne(&mut self, val: f64) -> Result<(), BadPos> {
        self.put(&f64::to_ne_bytes(val))
    }
}

impl BytesMut for &mut [u8] {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Look,BytesMut};

    #[test]
    fn test_put() {
        let src = &mut [2,0,0,0,b'H',b'i',b'\0'][..];
        let mut a = Look::new(src);

        let out = a.put(&[]);
        assert!(out.is_ok());
        assert!(a.as_ref() == &[2,0,0,0,b'H',b'i',b'\0'][..]);
        // assert!(src == &[2,0,0,0,b'H',b'i',b'\0'][..]);

        let four = a.put(&[0xFF,0xFF,0xFF,0xFF]);
        assert!(four.is_ok(), "put 4");
        assert!(a.as_ref() == &[b'H',b'i',b'\0'][..]);
        // assert!(src == &[0xFF,0xFF,0xFF,0xFF,b'H',b'i',b'\0'][..]);

        let four_err = a.put(&[0xFF,0xFF,0xFF,0xFF]);
        assert!(four_err.is_err(), "put 4 out of bounds");
        assert!(a.as_ref() == &[b'H',b'i',b'\0'][..]);
        // assert!(src == &[0xFF,0xFF,0xFF,0xFF,b'H',b'i',b'\0'][..]);

        let rest = a.put(&[b'O',b'k',b'\0']);
        assert!(rest.is_ok(), "put rest");
        assert!(a.as_ref() == &[]);
        // assert!(src == &[0xFF,0xFF,0xFF,0xFF,b'O',b'k',b'\0'][..]);

        let one_err = a.put(&[0xFF]);
        assert!(one_err.is_err(), "put 1 out of bounds");
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();

        assert!(src == &[0xFF,0xFF,0xFF,0xFF,b'O',b'k',b'\0'][..]);
    }

    #[test]
    fn test_put_u8() {
        let src: &mut [u8] = &mut [0,1][..];
        let mut a = Look::new(src);

        let b = a.put_u8(0xFF);
        assert!(b.is_ok());
        assert!(a.as_ref() == &[1]);

        let c = a.put_u8(0xFE);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let d = a.put_u8(0xFD);
        assert!(d.is_err());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[0xFF,0xFE]);
    }

    #[test]
    fn test_put_i8() {
        let src: &mut [u8] = &mut [0,1,-1i8 as u8];
        let mut a = Look::new(src);

        let b = a.put_i8(-1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &[1,-1i8 as u8]);

        let c = a.put_i8(0);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[-1i8 as u8]);

        let d = a.put_i8(1);
        assert!(d.is_ok());
        assert!(a.as_ref() == &[]);

        let e = a.put_i8(-2);
        assert!(e.is_err());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[-1i8 as u8,0,1]);
    }

    #[test]
    fn test_put_u16() {
        let src = &mut [1u16.to_le_bytes(),u16::MAX.to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u16_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u16::MAX.to_le_bytes()[..]);

        let c = a.put_u16_le(u16::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u16.to_le_bytes(),u16::MAX.to_le_bytes()].concat()[..]);

        let src = &mut [1u16.to_be_bytes(),u16::MAX.to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u16_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u16::MAX.to_be_bytes()[..]);

        let c = a.put_u16_be(u16::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u16.to_be_bytes(),u16::MAX.to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_i16() {
        let src: &mut [u8] = &mut [1i16.to_le_bytes(),(-1i16).to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i16_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i16).to_le_bytes()[..]);

        let c = a.put_i16_le(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i16.to_le_bytes(),(-1i16).to_le_bytes()].concat()[..]);

        let src: &mut [u8] = &mut [1i16.to_be_bytes(),(-1i16).to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i16_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i16).to_be_bytes()[..]);

        let c = a.put_i16_be(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i16.to_be_bytes(),(-1i16).to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_u32() {
        let src = &mut [1u32.to_le_bytes(),u32::MAX.to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u32_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u32::MAX.to_le_bytes()[..]);

        let c = a.put_u32_le(u32::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u32.to_le_bytes(),u32::MAX.to_le_bytes()].concat()[..]);

        let src = &mut [1u32.to_be_bytes(),u32::MAX.to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u32_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u32::MAX.to_be_bytes()[..]);

        let c = a.put_u32_be(u32::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u32.to_be_bytes(),u32::MAX.to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_i32() {
        let src: &mut [u8] = &mut [1i32.to_le_bytes(),(-1i32).to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i32_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i32).to_le_bytes()[..]);

        let c = a.put_i32_le(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i32.to_le_bytes(),(-1i32).to_le_bytes()].concat()[..]);

        let src: &mut [u8] = &mut [1i32.to_be_bytes(),(-1i32).to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i32_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i32).to_be_bytes()[..]);

        let c = a.put_i32_be(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i32.to_be_bytes(),(-1i32).to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_u64() {
        let src = &mut [1u64.to_le_bytes(),u64::MAX.to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u64_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u64::MAX.to_le_bytes()[..]);

        let c = a.put_u64_le(u64::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u64.to_le_bytes(),u64::MAX.to_le_bytes()].concat()[..]);

        let src = &mut [1u64.to_be_bytes(),u64::MAX.to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u64_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u64::MAX.to_be_bytes()[..]);

        let c = a.put_u64_be(u64::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u64.to_be_bytes(),u64::MAX.to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_i64() {
        let src: &mut [u8] = &mut [1i64.to_le_bytes(),(-1i64).to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i64_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i64).to_le_bytes()[..]);

        let c = a.put_i64_le(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i64.to_le_bytes(),(-1i64).to_le_bytes()].concat()[..]);

        let src: &mut [u8] = &mut [1i64.to_be_bytes(),(-1i64).to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i64_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i64).to_be_bytes()[..]);

        let c = a.put_i64_be(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i64.to_be_bytes(),(-1i64).to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_u128() {
        let src = &mut [1u128.to_le_bytes(),u128::MAX.to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u128_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u128::MAX.to_le_bytes()[..]);

        let c = a.put_u128_le(u128::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u128.to_le_bytes(),u128::MAX.to_le_bytes()].concat()[..]);

        let src = &mut [1u128.to_be_bytes(),u128::MAX.to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u128_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u128::MAX.to_be_bytes()[..]);

        let c = a.put_u128_be(u128::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u128.to_be_bytes(),u128::MAX.to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_i128() {
        let src: &mut [u8] = &mut [1i128.to_le_bytes(),(-1i128).to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i128_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i128).to_le_bytes()[..]);

        let c = a.put_i128_le(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i128.to_le_bytes(),(-1i128).to_le_bytes()].concat()[..]);

        let src: &mut [u8] = &mut [1i128.to_be_bytes(),(-1i128).to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i128_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i128).to_be_bytes()[..]);

        let c = a.put_i128_be(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i128.to_be_bytes(),(-1i128).to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_f32() {
        let src = &mut [1f32.to_le_bytes(),(-1f32).to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_f32_le(1f32);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1f32).to_le_bytes()[..]);

        let c = a.put_f32_le(-1f32);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1f32.to_le_bytes(),(-1f32).to_le_bytes()].concat()[..]);

        let src = &mut [1f32.to_be_bytes(),(-1f32).to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_f32_be(1f32);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1f32).to_be_bytes()[..]);

        let c = a.put_f32_be(-1f32);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1f32.to_be_bytes(),(-1f32).to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_f64() {
        let src = &mut [1f64.to_le_bytes(),(-1f64).to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_f64_le(1f64);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1f64).to_le_bytes()[..]);

        let c = a.put_f64_le(-1f64);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1f64.to_le_bytes(),(-1f64).to_le_bytes()].concat()[..]);

        let src = &mut [1f64.to_be_bytes(),(-1f64).to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_f64_be(1f64);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1f64).to_be_bytes()[..]);

        let c = a.put_f64_be(-1f64);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1f64.to_be_bytes(),(-1f64).to_be_bytes()].concat()[..]);
    }
}