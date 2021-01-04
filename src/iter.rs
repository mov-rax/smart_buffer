use crate::SmartBuffer;

use core::mem::size_of;
use alloc::alloc::{dealloc};
use crate::__core::iter::Map;

impl<T, const N: usize> IntoIterator for SmartBuffer<T,N>
    where T: Clone
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
    where T: Clone
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
    where T: Clone
{
    type Item = &'a mut T;
    type IntoIter = SmartBufferIterRefMut<'a,T,N>;
    /// Creates a consuming Iterator
    fn into_iter(self) -> Self::IntoIter {
        let stack_ptr = self.s_buf.as_mut_ptr();
        let heap_ptr = self.d_buf.clone();
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
    where T: Clone
{
    smart_buffer: SmartBuffer<T,N>,
    stack_ptr: *mut T,
    heap_ptr: Option<*mut T>,
    total_elem: usize,
    count: usize,
}

impl<T, const N: usize> Iterator for SmartBufferIter<T,N>
    where T: Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.total_elem{
            if self.count < N{
                self.count += 1;
                return unsafe {Some((*((self.stack_ptr as usize + (self.count - 1) * size_of::<T>()) as *mut T)).clone())}
            }
            self.count += 1;
            return unsafe {Some((*((self.heap_ptr.unwrap() as usize + (self.count - N - 1) * size_of::<T>()) as *mut T)).clone())}
        }
        None
    }

    // fn map<F>(self, f: F) -> Map<Self, F>
    // where F: FnMut(Self::Item) -> T
    // {
    //     unimplemented!()
    // }
}

/// Iterator for SmartBuffer where the SmartBuffer is immutably referenced to
pub struct SmartBufferIterRef<'a, T, const N:usize>
    where T: 'a + Clone
{
    smart_buffer: &'a SmartBuffer<T,N>,
    stack_ptr: *const T,
    heap_ptr: Option<*mut T>, // will not change values in heap_ptr
    total_elem: usize,
    count: usize,
}

impl<'a, T, const N: usize> Iterator for SmartBufferIterRef<'a, T,N>
    where T: 'a + Clone
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.total_elem{
            if self.count < N{
                self.count += 1;
                return unsafe {Some(&*((self.stack_ptr as usize + (self.count - 1) * size_of::<T>()) as *const T))}
            }
            self.count += 1;
            return unsafe {Some(&*((self.heap_ptr.unwrap() as usize + (self.count - N - 1) * size_of::<T>()) as *const T))}
        }
        None
    }
}

/// Iterator for SmartBuffer where SmartBuffer is mutably referenced to
pub struct SmartBufferIterRefMut<'a, T, const N:usize>
    where T: 'a + Clone
{
    smart_buffer: &'a mut SmartBuffer<T,N>,
    stack_ptr: *mut T,
    heap_ptr: Option<*mut T>, // will not change values in heap_ptr
    total_elem: usize,
    count: usize,
}

impl<'a, T, const N: usize> Iterator for SmartBufferIterRefMut<'a, T,N>
    where T: 'a + Clone
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.total_elem{
            if self.count < N{
                self.count += 1;
                return unsafe {Some(&mut *((self.stack_ptr as usize + (self.count - 1) * size_of::<T>()) as *mut T))}
            }
            self.count += 1;
            return unsafe {Some(&mut *((self.heap_ptr.unwrap() as usize + (self.count - N - 1) * size_of::<T>()) as *mut T))}
        }
        None
    }
}
