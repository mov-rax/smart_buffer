use crate::SmartBuffer;
use alloc::vec::Vec;
use crate::iter::SmartBufferIter;

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
