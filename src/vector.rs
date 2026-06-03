use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr;

const DEFAULT_CAPACITY: usize = 16;

#[derive(Debug)]
pub struct Vector<T> {
    ptr: *mut T,
    capacity: usize,
    size: usize
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                dealloc(self.ptr as *mut u8, Layout::array::<T>(self.capacity).unwrap())
            }
        }
    }
}

impl<T: Default + Copy> Vector<T> {
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            capacity: 0,
            size: 0
        }
    }
    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.capacity == 0 {
            (DEFAULT_CAPACITY, Layout::array::<T>(DEFAULT_CAPACITY).unwrap())
        } else {
            (self.capacity * 2, Layout::array::<T>(DEFAULT_CAPACITY).unwrap())
        };

        let new_ptr = if self.capacity == 0 {
            unsafe { alloc(new_layout) as *mut T }
        } else {
            unsafe { realloc(self.ptr as *mut u8, new_layout, self.capacity * 2) as *mut T }
        };

        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(new_layout);
        }

        self.ptr = new_ptr;
        self.capacity = new_cap;
    }
    pub fn size(&self) -> usize { self.size }
    pub fn capacity(&self) -> usize { self.capacity }
    pub fn is_empty(&self) -> bool { self.size == 0 }
    pub fn at(&self, idx: usize) -> Result<T, String> {
        if idx > self.capacity || idx > self.size - 1 {
            return Err("Vector index out of bounds".to_string());
        }
        let v = unsafe { *self.ptr.add(idx) };
        Ok(v)
    }
    pub fn push(&mut self, item: T) {
        if self.size == self.capacity {
            self.grow();
        }
        unsafe { self.ptr.add(self.size).write(item) }
        self.size += 1;
    }
}
