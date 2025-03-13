use crate::core::collection::List;
use core::alloc::Allocator;
use core::marker::PhantomData;
use core::ptr::NonNull;
use core_alloc::alloc::Global;

pub struct ArrayList<T, A: Allocator = Global> {
    buf: RawArrayList<T, A>,
    len: usize,
}

impl<T, A: Allocator> ArrayList<T, A> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        todo!()
    }
}

impl<T, A: Allocator> List<T> for ArrayList<T, A> {}

struct RawArrayList<T, A: Allocator = Global> {
    inner: RawArrayInner<A>,
    _m: PhantomData<T>,
}

struct RawArrayInner<A: Allocator = Global> {
    ptr: NonNull<u8>,
    cap: usize,
    alloc: A,
}
