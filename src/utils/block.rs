use std::any;
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::Read;
use std::mem;
use std::ops::{Index, IndexMut};
use std::slice;

/// Block: an array of u8's that can be interpreted as
/// any abstract data type
#[derive(Debug)]
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
    #[inline]
    pub const fn empty() -> Block {
        Block { data: vec![] }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data = vec![];
    }

    #[inline]
    pub fn resize(&mut self, size: usize) {
        self.data.resize(size, 0u8);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub unsafe fn get<T: Copy>(&self, offset: usize) -> Result<&T, BlockError> {
        let len: usize = mem::size_of::<T>();

        if offset + len > self.data.len() {
            return Err(BlockError::Overflow(offset + len - self.data.len()));
        }

        let arr = &self.data[offset];

        let ret = mem::transmute::<&u8, &T>(arr);

        Ok(ret)
    }

    pub unsafe fn get_mut<T: Copy>(&mut self, offset: usize) -> Result<&mut T, BlockError> {
        let len: usize = mem::size_of::<T>();

        if offset + len > self.data.len() {
            return Err(BlockError::Overflow(offset + len - self.data.len()));
        }

        let arr = &mut self.data[offset];

        let ret = mem::transmute::<&mut u8, &mut T>(arr);

        Ok(ret)
    }

    /// Offset is the offset in bytes
    pub fn set<T: Copy>(&mut self, offset: usize, val: T) -> Result<(), BlockError> {
        let len: usize = mem::size_of::<T>();
        let data = unsafe {
            let ptr = &val as * const T;
            slice::from_raw_parts(ptr as * const u8, len)
        };

        if offset + len > self.data.len() {
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
            let ext = slice::from_raw_parts(ptr as * mut u8, len);

            self.data.extend(ext);
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

    pub fn push_vec<T: Copy>(&mut self, vec: Vec<T>) {
        for e in vec {
            self.push::<T>(e);
        }
    }

    pub fn from_vec<T: Copy>(vec: &Vec<T>) -> Block {
        let mut ret: Block = Block::empty();
        ret.push_vec(vec.clone());

        ret
    }

    pub fn remove_bytes(&mut self, start: usize, len: usize) -> Result<(), BlockError> {
        if start + len > self.len() {
            Err(BlockError::Overflow(start + len - self.len()))
        } else {
            // there's gotta be a better way to do this
            for i in start..start+len {
                self.data.remove(i);
            }
            Ok(())
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn from_file(f: &mut File) -> Result<Block, std::io::Error> {
        let mut ret: Block = Block::empty();
        f.read_to_end(&mut ret.data)?;

        Ok(ret)
    }

    pub fn print<T: Copy + Display>(&self) {
        // header
        println!("Data block at heap address {:p} as {} {{", self.as_ptr(), any::type_name::<T>());
        let d_len = self.data.len();
        let e_len = mem::size_of::<T>();

        let count = (d_len / e_len) as usize;

        // data
        for i in 0..count {
            unsafe {
                println!("    {},", self.get::<T>(i * e_len).expect("this shouldn't happen"));
            }
        }

        // footer
        println!("}}");
    }
}

impl Index<usize> for Block {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl IndexMut<usize> for Block {
    fn index_mut(&mut self, i: usize) -> &mut u8 {
        &mut self.data[i]
    }
}

