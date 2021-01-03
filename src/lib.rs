#![no_std]
#![feature(min_const_generics)]
extern crate alloc;
use alloc::alloc::{alloc, dealloc, Layout};
use core::mem::size_of;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[feature(min_const_generics)]
pub struct SmartBuffer<T, const N:usize>
    where T: Copy + Clone
{
    s_buf: [T; N],
    d_buf: Option<*mut T>,
    layout: Option<Layout>,
    size: usize,
    capacity: usize,
    default: T, // The zero value
cursor: usize,
}

impl<T, const N:usize> SmartBuffer<T,N>
    where T: Copy + Clone
{

    /// Safely push a value into the SmartBuffer
    pub fn push(&mut self, other: T){
        if self.size < N{ // insert into stack
            self.s_buf[self.size] = other;
            self.size += 1;
        } else if self.size < self.capacity{ // insert into heap
            unsafe {*((self.d_buf.unwrap() as usize + (self.size - N) * size_of::<T>()) as *mut T) = other};
            self.size += 1;
        }
    }

    /// Sets the size of the buffer (does not reduce capacity)
    pub fn set_size(&mut self, size:usize){
        if self.size < self.capacity{
            self.size = size;
        }
    }

    /// Safely inserts a slice of data, starting at the size.
    pub fn insert_slice(&mut self, slice: &[T]){
        for elem in slice{
            self.push(*elem);
        }
    }

    /// Safely inserts a slice of data at an index;
    pub fn insert_slice_at(&mut self, slice: &[T], mut index:usize){
        for elem in slice{
            self.insert(*elem, index);
            index += 1;
        }
    }


    /// Safely inserts an array, starting at the size.
    pub fn insert_arr<const M: usize>(&mut self, arr: &[T; M]){
        for elem in arr{
            self.push(*elem);
        }
    }

    /// Safely insert a value into the SmartBuffer
    pub fn insert(&mut self, other: T, index: usize){
        if index < N{ // insert into stack
            self.s_buf[index] = other;
            if index > self.size{
                self.size = index;
            }
        } else if index < self.capacity{ // insert into heap
            unsafe {*((self.d_buf.unwrap() as usize + (index - N) * size_of::<T>()) as *mut T) = other};
            if index > self.size{
                self.size = index;
            }
        }
    }

    /// Safely get a value at an index
    pub fn get(&mut self, index:usize) -> Option<T> {
        if index < N {
            return Some(self.s_buf[index])
        } else if index < self.capacity {
            return unsafe { Some(*((self.d_buf.unwrap() as usize + (index - N) * size_of::<T>()) as *mut T)) }
        }
        None
    }

    /// Unsafely get a value at an index. An index too large will result in a fault
    pub unsafe fn get_unchecked(&mut self, index:usize) -> T{
        if index < N{
            return self.s_buf[index];
        }
        *((self.d_buf.unwrap() as usize + (index - N) * size_of::<T>()) as *mut T)
    }

    pub fn as_mut_ptr(mut self) -> *mut Self{
        &mut self as *mut Self
    }

    /// Safely allocate extra heap memory
    fn allocate(&mut self, elements:usize){
        let layout = Layout::from_size_align(elements*size_of::<T>(), size_of::<T>()); // aligned to itself
        if let Ok(layout) = layout{
            let ptr = unsafe {alloc(layout) as *mut T};
            self.capacity += layout.size();
            self.layout = Some(layout);
            self.d_buf = Some(ptr);
        }
    }

    pub fn new(value: T, len:usize) -> Self{
        let mut buf = Self{
            s_buf: [value; N],
            d_buf: None,
            layout: None,
            size: 0,
            capacity: N,
            default: value,
            cursor: 0,
        };
        if N < len{
            buf.allocate(len - N);
        }
        buf
    }

}

impl<T, const N:usize> SmartBuffer<T,N>
    where T: Copy + Clone + PartialEq
{
    /// Recalculates the size
    pub fn calc_size(&mut self){
        let default = self.default;

        let mut size = 0;
        for elem in &*self{
            if *elem == default{
                break;
            }
            size += 1;
        }
        self.set_size(size + 1);
    }
}


impl<T, const N: usize> IntoIterator for SmartBuffer<T,N>
    where T: Copy + Clone
{
    type Item = T;
    type IntoIter = SmartBufferIter<T,N>;
    /// Creates a consuming Iterator
    fn into_iter(mut self) -> Self::IntoIter {
        let stack_ptr = self.s_buf.as_mut_ptr();
        let heap_ptr = self.d_buf;
        let total_elem = self.size;

        Self::IntoIter {
            smart_buffer: self, // Self will be dropped when IntoIter is over
            stack_ptr,
            heap_ptr,
            total_elem,
            count: 0
        }
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a SmartBuffer<T,N>
    where T: Copy + Clone
{
    type Item = &'a T;
    type IntoIter = SmartBufferIterRef<'a,T,N>;
    /// Creates a consuming Iterator
    fn into_iter(self) -> Self::IntoIter {
        let stack_ptr = self.s_buf.as_ptr();
        let heap_ptr = self.d_buf;
        let total_elem = self.size;

        Self::IntoIter {
            smart_buffer: self,
            stack_ptr,
            heap_ptr,
            total_elem,
            count: 0
        }

    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut SmartBuffer<T,N>
    where T: Copy + Clone
{
    type Item = &'a mut T;
    type IntoIter = SmartBufferIterRefMut<'a,T,N>;
    /// Creates a consuming Iterator
    fn into_iter(self) -> Self::IntoIter {
        let stack_ptr = self.s_buf.as_mut_ptr();
        let heap_ptr = self.d_buf;
        let total_elem = self.size;

        Self::IntoIter {
            smart_buffer: self,
            stack_ptr,
            heap_ptr,
            total_elem,
            count: 0
        }
    }
}

/// Iterator for SmartBuffer where the SmartBuffer is Consumed
pub struct SmartBufferIter<T, const N:usize>
    where T: Copy + Clone
{
    smart_buffer: SmartBuffer<T,N>,
    stack_ptr: *mut T,
    heap_ptr: Option<*mut T>,
    total_elem: usize,
    count: usize,
}

impl<T, const N: usize> Iterator for SmartBufferIter<T,N>
    where T: Copy + Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.total_elem{
            if self.count < N{
                self.count += 1;
                return unsafe {Some(*((self.stack_ptr as usize + self.count - 1 * size_of::<T>()) as *mut T))}
            }
            self.count += 1;
            return unsafe {Some(*((self.heap_ptr.unwrap() as usize + self.count - N - 1 * size_of::<T>()) as *mut T))}
        }
        None
    }
}

/// Iterator for SmartBuffer where the SmartBuffer is immutably referenced to
pub struct SmartBufferIterRef<'a, T, const N:usize>
    where T: 'a + Copy + Clone
{
    smart_buffer: &'a SmartBuffer<T,N>,
    stack_ptr: *const T,
    heap_ptr: Option<*mut T>, // will not change values in heap_ptr
total_elem: usize,
    count: usize,
}

impl<'a, T, const N: usize> Iterator for SmartBufferIterRef<'a, T,N>
    where T: 'a + Copy + Clone
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.total_elem{
            if self.count < N{
                self.count += 1;
                return unsafe {Some(&*((self.stack_ptr as usize + self.count - 1 * size_of::<T>()) as *const T))}
            }
            self.count += 1;
            return unsafe {Some(&*((self.heap_ptr.unwrap() as usize + self.count - N - 1 * size_of::<T>()) as *const T))}
        }
        None
    }
}

/// Iterator for SmartBuffer where SmartBuffer is mutably referenced to
pub struct SmartBufferIterRefMut<'a, T, const N:usize>
    where T: 'a + Copy + Clone
{
    smart_buffer: &'a mut SmartBuffer<T,N>,
    stack_ptr: *mut T,
    heap_ptr: Option<*mut T>, // will not change values in heap_ptr
total_elem: usize,
    count: usize,
}

impl<'a, T, const N: usize> Iterator for SmartBufferIterRefMut<'a, T,N>
    where T: 'a + Copy + Clone
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.total_elem{
            if self.count < N{
                self.count += 1;
                return unsafe {Some(&mut *((self.stack_ptr as usize + self.count - 1 * size_of::<T>()) as *mut T))}
            }
            self.count += 1;
            return unsafe {Some(&mut *((self.heap_ptr.unwrap() as usize + self.count - N - 1 * size_of::<T>()) as *mut T))}
        }
        None
    }
}

impl<T, const N:usize> Drop for SmartBuffer<T,N>
    where T: Copy + Clone
{
    fn drop(&mut self) {
        if let Some(ptr) = self.d_buf{
            unsafe {dealloc(ptr as *mut u8, self.layout.unwrap())};
        }
    }
}

