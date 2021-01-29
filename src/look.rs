use core::marker::PhantomData;

use crate::{View,ViewMut,Bytes,BytesMut,BadPos};

/// Temporarily look into a buffer
///
/// Similar to a slice except you're able to restore the original buffer after slicing it. This can
/// be useful for knowing your position in the buffer and looking back. This can be done with two
/// slices but they need to be bounds checked.
pub struct Look<T, B> {
    buf: B,
    pos: usize,
    phantom: PhantomData<T>,
}

impl<T, B: AsRef<[T]>> Look<T, B> {
    pub fn new(buf: B) -> Look<T, B> {
        Look { buf, pos: 0, phantom: Default::default() }
    }

    pub fn new_with_pos(buf: B, pos: usize) -> Result<Look<T, B>, BadPos> {
        let buf_ref = buf.as_ref();
        if pos > buf_ref.len() { return Err(BadPos) }
        Ok(Look { buf, pos, phantom: Default::default() })
    }

    pub fn from_slice<'a, N: AsRef<[T]> + 'a>(buf: B, slice: &'a [T]) -> Result<Look<T, B>, BadPos> {
        let buf_ref = buf.as_ref();
        let buf_start_ptr = buf_ref.as_ptr();
        let buf_end_ptr = unsafe { buf_start_ptr.add(buf_ref.len()) };
        let slice_start_ptr = slice.as_ptr();
        let slice_end_ptr = unsafe { slice_start_ptr.add(slice.len()) };

        if
            slice_start_ptr < buf_start_ptr ||
            slice_start_ptr > buf_end_ptr ||
            slice_end_ptr > buf_end_ptr
        {
            return Err(BadPos)
        }

        let pos = buf_end_ptr as usize - slice_start_ptr as usize;

        Ok(Look { buf, pos, phantom: Default::default() })
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn into_inner(self) -> B {
        self.buf
    }
}

impl<T, B: AsRef<[T]>> AsRef<[T]> for Look<T, B> {
    fn as_ref(&self) -> &[T] {
        &self.buf.as_ref()[self.pos ..]
    }
}

impl<T, B: AsMut<[T]>> AsMut<[T]> for Look<T, B> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.buf.as_mut()[self.pos ..]
    }
}

impl<T, B: AsRef<[T]>> View<T> for Look<T, B> {
    fn take(&mut self, n: usize) -> Result<&[T], BadPos> {
        let buf = self.buf.as_ref();
        let len = buf.len();
        if n > len { return Err(BadPos) }
        let out = &buf[..n];
        self.pos += n;
        Ok(out)
    }
}

impl<T: Copy, B: AsMut<[T]>> ViewMut<T> for Look<T, B> {
    fn put(&mut self, val: &[T]) -> Result<(), BadPos> {
        let buf = self.as_mut();
        let n = val.len();
        let len = buf.len();
        if n > len { return Err(BadPos) }
        let write = &mut buf[..n];
        write.copy_from_slice(val);
        self.pos += n;
        Ok(())
    }
}

impl<B: AsRef<[u8]>> Bytes for Look<u8, B> {}

impl<B: AsMut<[u8]>> BytesMut for Look<u8, B> {}