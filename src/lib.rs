#![no_std]
#![feature(min_const_generics)]
extern crate alloc;
use alloc::alloc::{alloc, dealloc, Layout};
use core::mem::size_of;

mod iter;

#[cfg(test)]
mod tests {
    use crate::SmartBuffer;

    #[test]
    fn it_works() {
        let buf = SmartBuffer::<&str, 10>::new("",25);
    }
}

#[feature(min_const_generics)]
pub struct SmartBuffer<T, const N:usize>
    where T: Clone
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
    where T: Clone
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
            self.push(elem.clone());
        }
    }

    /// Safely inserts a slice of data at an index;
    pub fn insert_slice_at(&mut self, slice: &[T], mut index:usize){
        for elem in slice{
            self.insert(elem.clone(), index);
            index += 1;
        }
    }


    /// Safely inserts an array, starting at the size.
    pub fn insert_arr<const M: usize>(&mut self, arr: &[T; M]){
        for elem in arr{
            self.push(elem.clone());
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
    pub fn get(&mut self, index:usize) -> Option<&T> {
        if index < N {
            return Some(&self.s_buf[index])
        } else if index < self.capacity {
            return unsafe { Some(&*((self.d_buf.unwrap() as usize + (index - N) * size_of::<T>()) as *mut T)) }
        }
        None
    }

    /// Unsafely get a value at an index. An index too large will result in a fault
    pub unsafe fn get_unchecked(&mut self, index:usize) -> &T{
        if index < N{
            return &self.s_buf[index];
        }
        &*((self.d_buf.unwrap() as usize + (index - N) * size_of::<T>()) as *mut T)
    }

    pub fn as_mut_ptr(mut self) -> *mut Self{
        &mut self as *mut Self
    }

    /// Safely allocate extra heap memory
    fn allocate(&mut self, elements:usize){
        let layout = Layout::from_size_align(elements*size_of::<T>(), 1);
        if let Ok(layout) = layout{
            let ptr = unsafe {alloc(layout) as *mut T};
            self.capacity += layout.size();
            self.layout = Some(layout);
            self.d_buf = Some(ptr);
        }
    }
    // creates a SmartBuffer where the default value is set to the value entered
    pub fn new(value: T, len:usize) -> Self
    where T: Copy + Clone
    {
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

    // /// Creates a SmartBuffer where the default value is the first value entered.
    // ///
    // /// This function is not recommended
    // pub fn gen(buf:[T; N], len:usize) -> Self{
    //
    // }

}

impl<T, const N:usize> SmartBuffer<T,N>
    where T:Clone + PartialEq
{
    /// Recalculates the size
    pub fn calc_size(&mut self){
        let default = self.default.clone();

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

impl<T, const N:usize> Drop for SmartBuffer<T,N>
    where T: Clone
{
    fn drop(&mut self) {
        if let Some(ptr) = self.d_buf{
            unsafe {dealloc(ptr as *mut u8, self.layout.unwrap())};
        }
    }
}


