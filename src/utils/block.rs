use std::error::Error;
use std::fmt::{self, Display};
use std::mem;
use std::slice;

/// Block: an array of u8's that can be interpreted as
/// any abstract data type
pub struct Block {
    data: Vec<u8>,
}

#[derive(Debug)]
pub enum BlockError {
    Overflow(usize),
    Underflow(usize),
}

impl Error for BlockError {}

impl Display for BlockError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        match &self {
            BlockError::Overflow(amt) => write!(fmt, "block overflow by {} bytes", amt),
            BlockError::Underflow(amt) => write!(fmt, "block underflow by {} bytes", amt),
        }
    }
}

impl Block {
    pub fn empty() -> Block {
        Block { data: vec![] }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub unsafe fn get<T: Copy>(&self, offset: usize) -> Result<&mut T, BlockError> {
        let len: usize = mem::size_of::<T>();

        if offset + len >= self.data.len() {
            return Err(BlockError::Overflow(offset + len - self.data.len()));
        }

        let mut val: Vec<u8> = Vec::new();
        for i in 0..len {
            val.push(self.data[offset + i]);
        }

        let arr = &val;
        let arr_c: &mut [u8] = &mut [];
        arr_c.clone_from_slice(arr);

        Ok(&mut *mem::transmute::<* const u8, * mut T>(arr_c.as_ptr()))
    }

    pub fn set<T: Copy>(&mut self, offset: usize, val: T) -> Result<(), BlockError> {
        let len: usize = mem::size_of::<T>();
        let data = unsafe {
            let ptr = &val as * const T;
            slice::from_raw_parts(ptr as * const u8, len)
        };

        if offset + len >= self.data.len() {
            return Err(BlockError::Overflow(offset + len - self.data.len()));
        } else {
            for i in 0..len {
                self.data[offset + i] = data[i];
            }
            Ok(())
        }
    }

    pub fn push<T: Copy>(&mut self, val: T) {
        let len: usize = mem::size_of::<T>();
        let ptr = &val as * const T;

        unsafe {
            self.data.extend(slice::from_raw_parts(ptr as * const u8, len));
        }
    }

    pub unsafe fn pop<T: Copy>(&mut self) -> Result<T, BlockError> {
        let len: usize = mem::size_of::<T>();

        if self.data.len() < len {
            return Err(BlockError::Underflow(len - self.data.len()));
        }

        let ptr = &(self.data[self.data.len() - len]) as * const u8;

        let arr = slice::from_raw_parts(ptr, len);
        let mut arr_c: Vec<u8> = vec![0; len];
        arr_c.clone_from_slice(arr);

        self.data.truncate(self.data.len() - len);

        Ok(*mem::transmute::<* const u8, * const T>(arr_c.as_ptr()))
    }
}

