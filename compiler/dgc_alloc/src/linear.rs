use std::alloc::{AllocError, Allocator, Global, Layout};
use std::cmp::max;
use std::mem::SizedTypeProperties;
use std::ops::Range;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct Linear<'parent, A: Allocator = Global> {
    alloc: &'parent A,
    bounds: Range<usize>,
    offset: AtomicUsize,
}

impl<'parent, A: Allocator> Linear<'parent, A> {
    const DEFAULT_CAPACITY: usize = 4096;

    pub fn new_in(alloc: &'parent A) -> Self {
        Self::with_capacity_in(Self::DEFAULT_CAPACITY, alloc)
    }

    pub fn with_capacity_in(capacity: usize, alloc: &'parent A) -> Self {
        let capacity = max(capacity, Self::DEFAULT_CAPACITY); // TODO: remove dirty hack

        let layout = Layout::array::<u8>(capacity).expect("capacity overflow");

        let ptr = alloc.allocate(layout).expect("allocation failed");
        let start = ptr.addr().get();
        let end = start + capacity;

        Self {
            bounds: start..end,
            offset: AtomicUsize::new(start),
            alloc,
        }
    }

    /// Resets allocator.
    ///
    /// # Safety
    /// Pointers to this allocator memory must be freed.
    pub unsafe fn reset(&self) {
        self.offset.swap(self.bounds.start, Ordering::SeqCst);
    }

    /// Free allocator memory.
    ///
    /// # Safety
    /// Pointers to this allocator memory must be freed.
    pub unsafe fn free(self) {
        let raw_ptr = self.bounds.start as *mut u8;

        unsafe {
            let ptr = NonNull::new_unchecked(raw_ptr);
            self.alloc.deallocate(ptr, u8::LAYOUT);
        }
    }
}

unsafe impl<A: Allocator> Allocator for Linear<'_, A> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let mut offset = self.offset.load(Ordering::SeqCst);
        let align_mask = !(layout.align() - 1);

        let mem_start = (offset + layout.align() - 1) & align_mask;

        if mem_start + layout.size() > self.bounds.end {
            Err(AllocError)
        } else {
            offset = mem_start + layout.size();
            self.offset.store(offset, Ordering::SeqCst);

            let raw_ptr = mem_start as *mut u8;
            let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;

            Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
        }
    }

    unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
        // Linear allocator doesn't support such type of deallocation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_allocates_u16() {
        // arrange
        let alloc = Linear::new_in(&Global);
        let offset = alloc.offset.load(Ordering::SeqCst);
        let expected_offset = offset + 2;

        // act
        let _ = alloc.allocate(u16::LAYOUT).unwrap();

        // assert
        assert_eq!(expected_offset, alloc.offset.load(Ordering::SeqCst));
        assert_eq!(alloc.bounds.end - alloc.bounds.start, 4096);
    }

    #[test]
    fn it_works_with_vec() {
        // arrange
        let alloc = Linear::with_capacity_in(8, &Global);
        let offset = alloc.offset.load(Ordering::SeqCst);
        let expected_offset = offset + 8;

        let mut vec: Vec<u8, Linear> = Vec::new_in(alloc);

        // act
        vec.push(1);
        vec.push(2);

        // assert
        assert_eq!(
            expected_offset,
            vec.allocator().offset.load(Ordering::SeqCst)
        );
        assert_eq!(
            vec.allocator().bounds.end - vec.allocator().bounds.start,
            4096
        );
    }
}
