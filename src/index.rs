use crate::SmartBuffer;
use core::ops::Index;
use array_macro::__core::ops::IndexMut;

impl<T, const N:usize> Index<usize> for &SmartBuffer<T,N>
    where T: Clone
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {self.get_unchecked(index)}
    }
}

impl<T, const N:usize> Index<usize> for SmartBuffer<T,N>
    where T: Clone
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {self.get_unchecked(index)}
    }
}

impl<T, const N:usize> IndexMut<usize> for SmartBuffer<T,N>
where T: Clone
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {self.get_mut_unchecked(index)}
    }
}