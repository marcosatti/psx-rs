use std::ptr::NonNull;
use std::mem::size_of;
use std::convert::TryInto;
use std::fmt::{UpperHex, Debug};
use num_traits::{Unsigned, PrimInt};
use crate::types::bitfield::Bitfield;

#[derive(Clone, Copy, Debug)]
pub enum ReadError {
    Empty,
}

pub type ReadResult<T> = Result<T, ReadError>;

#[derive(Clone, Copy, Debug)]
pub enum WriteError {
    Full,
}

pub type WriteResult = Result<(), WriteError>;

pub trait B8MemoryMap {
    fn read_u8(&mut self, _offset: usize) -> ReadResult<u8> {
        panic!("Nothing implemented");
    }
    
    fn write_u8(&mut self, _offset: usize, _value: u8) -> WriteResult {
        panic!("Nothing implemented");
    }

    fn read_u16(&mut self, _offset: usize) -> ReadResult<u16> {
        panic!("Nothing implemented");
    }
    
    fn write_u16(&mut self, _offset: usize, _value: u16) -> WriteResult {
        panic!("Nothing implemented");
    }

    fn read_u32(&mut self, _offset: usize) -> ReadResult<u32> {
        panic!("Nothing implemented");
    }
    
    fn write_u32(&mut self, _offset: usize, _value: u32) -> WriteResult {
        panic!("Nothing implemented");
    }
}

pub struct B8MemoryMapper<T: PrimInt + Unsigned> {
    mappings: Vec<Option<Vec<Option<(NonNull<dyn B8MemoryMap>, T)>>>>,
    directory_mask: Bitfield,
    page_mask: Bitfield,
    offset_mask: Bitfield,
}

impl<T> B8MemoryMapper<T> 
where
    T: PrimInt + Unsigned + TryInto<usize> + Debug + UpperHex,
    <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug,
{
    pub fn new(directory_bits: usize, page_bits: usize) -> B8MemoryMapper<T> {
        let type_bits = size_of::<T>() * 8;
        let directory_mask = Bitfield::new(type_bits - directory_bits, directory_bits);
        let page_mask = Bitfield::new(type_bits - directory_bits - page_bits, page_bits);
        let offset_mask = Bitfield::new(0, type_bits - directory_bits - page_bits);

        debug_assert!((directory_mask.length + page_mask.length + offset_mask.length) == type_bits);

        B8MemoryMapper {
            mappings: vec![None; 1 << directory_bits],
            directory_mask: directory_mask,
            page_mask: page_mask,
            offset_mask: offset_mask,
        }
    }

    pub fn map(&mut self, address: T, size: usize, object: *mut dyn B8MemoryMap) {
        debug_assert!(size > 0);
        debug_assert!(!object.is_null());

        let mut directory_index: usize = self.directory_mask.extract_from(address).try_into().unwrap();
        let mut page_index: usize = self.page_mask.extract_from(address).try_into().unwrap();
        let len_pages = 1 << self.page_mask.length;
        let mut map_size: usize = 0;

        while map_size < size {
            let directory = &mut self.mappings[directory_index];

            while (page_index < len_pages) && (map_size < size) {
                if directory.is_none() {
                    *directory = Some(vec![None; len_pages]);
                }

                let directory = directory.as_mut().unwrap();
                let page = &mut directory[page_index];

                if page.is_some() {
                    panic!(format!("Address already mapped: 0x{:0X}", address));
                }

                *page = Some((unsafe { NonNull::new_unchecked(object) }, address));

                map_size += 1 << self.offset_mask.length;
                page_index += 1;
            }

            directory_index += 1;
            page_index = 0;
        }
    }

    fn object_at(&self, address: T) -> (*mut dyn B8MemoryMap, T)
    {
        unsafe {
            let directory_index: usize = self.directory_mask.extract_from(address).try_into().unwrap();
            let directory = match self.mappings.get_unchecked(directory_index).as_ref() {
                Some(d) => d,
                None => panic!(format!("Missing object map at address 0x{:0X}", address)),
            };
            let page_index: usize = self.page_mask.extract_from(address).try_into().unwrap();
            let page = &directory.get_unchecked(page_index);
            let page = match page.as_ref() {
                Some(p) => p,
                None => panic!(format!("Missing object map at address 0x{:0X}", address)),
            };
            (page.0.as_ptr(), page.1)
        }
    }

    pub fn read_u8(&self, address: T) -> ReadResult<u8> {
        let (object, base_address) = self.object_at(address);
        let offset_index: usize = ((address - base_address) + self.offset_mask.extract_from(address)).try_into().unwrap();

        unsafe {
            let object = &mut *object;
            object.read_u8(offset_index)
        }
    }

    pub fn write_u8(&self, address: T, value: u8) -> WriteResult {
        let (object, base_address) = self.object_at(address);
        let offset_index: usize = ((address - base_address) + self.offset_mask.extract_from(address)).try_into().unwrap();

        unsafe {
            let object = &mut *object;
            object.write_u8(offset_index, value)
        }
    }

    pub fn read_u16(&self, address: T) -> ReadResult<u16> {
        let (object, base_address) = self.object_at(address);
        let offset_index: usize = ((address - base_address) + self.offset_mask.extract_from(address)).try_into().unwrap();

        unsafe {
            let object = &mut *object;
            object.read_u16(offset_index)
        }
    }

    pub fn write_u16(&self, address: T, value: u16) -> WriteResult {
        let (object, base_address) = self.object_at(address);
        let offset_index: usize = ((address - base_address) + self.offset_mask.extract_from(address)).try_into().unwrap();

        unsafe {
            let object = &mut *object;
            object.write_u16(offset_index, value)
        }
    }

    pub fn read_u32(&self, address: T) -> ReadResult<u32> {
        let (object, base_address) = self.object_at(address);
        let offset_index: usize = ((address - base_address) + self.offset_mask.extract_from(address)).try_into().unwrap();

        unsafe {
            let object = &mut *object;
            object.read_u32(offset_index)
        }
    }

    pub fn write_u32(&self, address: T, value: u32) -> WriteResult {
        let (object, base_address) = self.object_at(address);
        let offset_index: usize = ((address - base_address) + self.offset_mask.extract_from(address)).try_into().unwrap();

        unsafe {
            let object = &mut *object;
            object.write_u32(offset_index, value)
        }
    }
}
