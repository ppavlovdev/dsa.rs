use std::alloc::{Layout, alloc, dealloc, realloc};
use std::ptr;

const DEFAULT_CAPACITY: usize = 16;

#[derive(Debug)]
pub struct Vector<T> {
    ptr: *mut T,
    capacity: usize,
    size: usize,
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                dealloc(
                    self.ptr as *mut u8,
                    Layout::array::<T>(self.capacity).unwrap(),
                )
            }
        }
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self {
            ptr: ptr::null_mut(),
            capacity: 0,
            size: 0,
        }
    }
}

impl<T: Default + Copy + PartialEq> Vector<T> {
    pub fn new() -> Self {
        Self::default()
    }

    fn resize(&mut self, new_capacity: usize) {
        let (new_cap, new_layout) = if self.capacity == 0 {
            (
                DEFAULT_CAPACITY,
                Layout::array::<T>(DEFAULT_CAPACITY).unwrap(),
            )
        } else {
            (new_capacity, Layout::array::<T>(new_capacity).unwrap())
        };

        let new_ptr = if self.capacity == 0 {
            unsafe { alloc(new_layout) as *mut T }
        } else {
            let old_layout = Layout::array::<T>(DEFAULT_CAPACITY).unwrap();
            unsafe { realloc(self.ptr as *mut u8, old_layout, new_layout.size()) as *mut T }
        };

        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(new_layout);
        }

        self.ptr = new_ptr;
        self.capacity = new_cap;
    }

    fn is_out_of_bounds(&self, idx: usize) -> bool {
        idx > self.capacity || idx > self.size - 1
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn at(&self, idx: usize) -> Result<T, String> {
        if self.is_out_of_bounds(idx) {
            return Err("Vector index out of bounds".to_string());
        }
        let v = unsafe { *self.ptr.add(idx) };
        Ok(v)
    }

    pub fn push(&mut self, item: T) {
        if self.size == self.capacity {
            self.resize(self.capacity * 2);
        }
        unsafe { self.ptr.add(self.size).write(item) }
        self.size += 1;
    }

    pub fn insert(&mut self, idx: usize, item: T) {
        if self.size + 1 > self.capacity {
            self.resize(self.capacity * 2);
        }
        unsafe {
            for i in (idx..self.size).rev() {
                let curr = self.ptr.add(i);
                self.ptr.add(i + 1).write(*curr);
            }
            self.ptr.add(idx).write(item);
        };
        self.size += 1;
    }

    pub fn prepend(&mut self, item: T) {
        self.insert(0, item);
    }

    pub fn pop(&mut self) -> Result<T, String> {
        if self.is_empty() {
            return Err("Vector is empty".to_string());
        };
        let res = unsafe {
            let v = self.at(self.size - 1).unwrap();
            self.ptr.add(self.size - 1).write(T::default());
            self.size -= 1;
            if (self.capacity / self.size) as f64 <= 0.25 {
                self.resize(self.capacity / 2);
            }
            v
        };
        Ok(res)
    }

    pub fn delete(&mut self, idx: usize) -> Result<(), String> {
        if self.is_out_of_bounds(idx) {
            return Err("Vector index out of bounds".to_string());
        };
        unsafe {
            self.ptr.add(idx).write(T::default());
            for i in idx..self.size {
                let next = self.ptr.add(i + 1);
                self.ptr.add(i).write(*next);
            }
            self.size -= 1;
            if (self.capacity / self.size) as f64 <= 0.25 {
                self.resize(self.capacity / 2);
            }
        }
        Ok(())
    }

    pub fn remove(&mut self, item: T) {
        unsafe {
            for i in (0..self.size).rev() {
                match self.at(i) {
                    Ok(_) if *self.ptr.add(i) == item => {
                        let _ = self.delete(i);
                    }
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
        }
    }

    pub fn find(&self, item: T) -> Option<usize> {
        for i in (0..self.size).rev() {
            match self.at(i) {
                Ok(v) if v == item => return Some(i),
                Ok(_) => continue,
                Err(_) => break,
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vector() {
        let v = Vector::<i32>::new();

        assert_eq!(v.capacity(), 0);
        assert_eq!(v.size(), 0);
        assert!(v.is_empty());
        assert_eq!(v.at(100), Err("Vector index out of bounds".to_string()));
    }

    #[test]
    fn test_filled_vector() {
        let mut v = Vector::<i32>::new();

        v.push(1);
        v.push(2);
        v.push(3);

        assert_eq!(v.capacity(), 16usize);
        assert_eq!(v.size(), 3usize);
        assert!(!v.is_empty());
        assert_eq!(v.at(0), Ok(1));
        assert_eq!(v.at(1), Ok(2));
        assert_eq!(v.at(2), Ok(3));
        assert_eq!(v.at(3), Err("Vector index out of bounds".to_string()));
    }

    #[test]
    fn test_vector_grow() {
        let mut v = Vector::<i32>::new();

        for i in 0..16 {
            v.push(i);
        }

        assert_eq!(v.capacity(), 16usize);
        assert_eq!(v.size(), 16usize);

        v.push(100);
        v.push(101);

        assert_eq!(v.capacity(), 32usize);
        assert_eq!(v.size(), 18usize);
    }

    #[test]
    fn test_vector_insert() {
        let mut v = Vector::<i32>::new();

        v.insert(0usize, 1);
        assert_eq!(v.capacity(), 16);
        assert_eq!(v.size(), 1usize);
        assert_eq!(v.at(0), Ok(1));
    }

    #[test]
    fn test_vector_insert_with_grow() {
        let mut v = Vector::<i32>::new();

        for i in 0..16 {
            v.push(i);
        }

        assert_eq!(v.capacity(), 16);
        assert_eq!(v.size(), 16usize);
        assert_eq!(v.at(0), Ok(0));

        v.insert(0, 777);
        assert_eq!(v.capacity(), 32usize);
        assert_eq!(v.size(), 17usize);
        assert_eq!(v.at(0), Ok(777));
        assert_eq!(v.at(1), Ok(0));
        assert_eq!(v.at(2), Ok(1));
        assert_eq!(v.at(16), Ok(15));
    }
}
