mod array_list;

pub use array_list::*;
use core::ops::{Index, IndexMut};

pub trait List<T>:
    IntoIterator<Item = T> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    fn push_back(&mut self, item: T) -> &mut Self;
}
