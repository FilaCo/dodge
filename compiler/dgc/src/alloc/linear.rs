use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};
use core_alloc::alloc::{AllocError, Allocator, Layout};
use core_alloc::sync::Arc;

#[derive(Debug)]
pub struct Linear {
    start: usize,
    end: usize,
    offset: Arc<AtomicUsize>,
}

unsafe impl Allocator for Linear {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let mut offset = self.offset.load(Ordering::Relaxed);
        let align_mask = !(layout.align() - 1);

        let mem_start = (offset + layout.align() - 1) & align_mask;

        if mem_start + layout.size() > self.end {
            Err(AllocError)
        } else {
            offset = mem_start + layout.size();
            self.offset.store(offset, Ordering::Relaxed);

            let raw_ptr = mem_start as *mut u8;
            let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;

            Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
        }
    }

    unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
        // Not supported for a linear allocator
    }
}

impl Linear {
    const DEFAULT_SIZE: usize = 4096;

    pub fn new<A: Allocator>(alloc: &A) -> Self {
        Self::with_size(alloc, Self::DEFAULT_SIZE)
    }

    pub fn with_size<A: Allocator>(alloc: &A, size: usize) -> Self {
        let ptr = alloc
            .allocate(Layout::new::<u8>())
            .expect("failed to allocate memory");

        let start = ptr.addr();

        Self {
            start: start.get(),
            end: start.get() + size,
            offset: Arc::new(AtomicUsize::new(start.get())),
        }
    }
}
