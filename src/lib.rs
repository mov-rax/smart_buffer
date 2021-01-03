#![no_std]
#![feature(min_const_generics)]
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate array_macro;

use alloc::alloc::{alloc, dealloc, Layout};
use alloc::vec::Vec;
use core::mem::size_of;
use array_macro::array;
use crate::iter::SmartBufferIterRef;

mod iter;
mod index;

#[macro_use]
#[cfg(test)]
mod tests {
    use crate::SmartBuffer;
    use alloc::string::String;
    use crate::buf;
    use alloc::vec::Vec;

    #[test]
    fn it_works() {
        let mut buf = buf!(String::new(), 2, 4);
        buf.push(String::from("I wonder"));
        buf.push(String::from("What you could do with this"));
        buf.push(String::from("This is in the heap now!"));
        buf.push(String::from("Look mom, no hands!"));

        buf[0] = format!("I have rearranged this!");

        let vecbuf:Vec<_> = buf.into();
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

    /// Clears all values to the given default value.
    pub fn clear(&mut self){
        let default = self.default.clone();
        for elem in self{
            *elem = default.clone();
        }
    }

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
    pub fn get(&self, index:usize) -> Option<&T> {
        if index < N {
            return Some(&(self.s_buf[index]))
        } else if index < self.capacity {
            return unsafe { Some(&*((self.d_buf.unwrap() as usize + (index - N) * size_of::<T>()) as *const T)) }
        }
        None
    }

    /// Unsafely get a reference to a value at an index. An index too large will result in a fault
    pub unsafe fn get_unchecked(&self, index:usize) -> &T{
        if index < N{
            return &self.s_buf[index];
        }
        &*((self.d_buf.unwrap() as usize + (index - N) * size_of::<T>()) as *const T)
    }

    /// Unsafely get a mutable
    pub unsafe fn get_mut_unchecked(&mut self, index:usize) -> &mut T{
        if index < N{
            return &mut self.s_buf[index]
        }
        &mut *((self.d_buf.unwrap() as usize + (index - N) * size_of::<T>()) as *mut T)
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

    /// Creates a SmartBuffer from an array
    pub fn from_arr(buf:[T; N], len:usize) -> Self
    where T: Clone
    {
        let def = buf[0].clone();
        let mut buf = Self{
            s_buf: buf,
            d_buf: None,
            layout: None,
            size: 0,
            capacity: N,
            default: def,
            cursor: 0,
        };

        if N < len{
            buf.allocate(len - N);
        }
        buf
    }
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

impl<T, const N: usize> Into<Vec<T>> for SmartBuffer<T,N>
where T: Clone
{
    fn into(self) -> Vec<T> {
        let mut temp = Vec::new();
        for elem in self{
            temp.push(elem);
        }
        temp
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

#[macro_export]
/// Macro that easily creates a new SmartBuffer!
///
///  Requires `data, s_len, t_len`
///
/// - The first element in the macro requires the data that will be used in the SmartBuffer
/// - The second element is the size of the stack portion of the SmartBuffer, whose size must be known on compile time. (CONSTANT)
/// - The third element is the total required size of the SmartBuffer, which allocates memory if necessary on the heap at runtime!
macro_rules! buf {
    ($data:expr, $s_len:expr, $t_len:expr) => {
        $crate::SmartBuffer::<_,$s_len>::from_arr(array_macro::array!(_ => $data; $s_len), $t_len)
    }
}


