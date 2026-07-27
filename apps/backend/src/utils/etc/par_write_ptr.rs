#[derive(Clone, Copy)]
pub struct ParallelWritePtr<T>(*mut T);

unsafe impl<T: Send> Send for ParallelWritePtr<T> {}
unsafe impl<T: Send> Sync for ParallelWritePtr<T> {}

impl<T> ParallelWritePtr<T> {
    pub fn new(ptr: *mut T) -> Self {
        Self(ptr)
    }

    /// # Safety
    /// Caller must ensure `index` is in bounds, unique across concurrent
    /// writers, and that the backing allocation stays valid & non-aliased.
    #[inline]
    pub unsafe fn write(self, index: usize, value: T) {
        unsafe {
            self.0.add(index).write(value);
        }
    }
}
