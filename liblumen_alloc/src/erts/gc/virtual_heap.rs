#![allow(unused)]
use core::mem;

use intrusive_collections::intrusive_adapter;
use intrusive_collections::{LinkedList, LinkedListLink, UnsafeRef};

use crate::erts::*;

intrusive_adapter!(pub ProcBinAdapter = UnsafeRef<ProcBin>: ProcBin { link: LinkedListLink });

pub struct VirtualBinaryHeap {
    bins: LinkedList<ProcBinAdapter>,
    size: usize,
}
impl VirtualBinaryHeap {
    /// Create a new virtual heap
    pub fn new() -> Self {
        Self {
            bins: LinkedList::new(ProcBinAdapter::new()),
            size: 0,
        }
    }

    /// Gets the current amount of virtual binary heap space used (in bytes)
    /// by binaries referenced from the current process
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Like `size`, but in units of size `Term`
    ///
    /// NOTE: This is used when calculating whether to
    /// perform a garbage collection, as a large virtual binary heap
    /// indicates there is likely a considerable amount of memory that can
    /// be reclaimed by freeing references to binaries in the virtual
    /// heap
    #[inline]
    pub fn word_size(&self) -> usize {
        let bin_size = self.size();
        let bin_words = bin_size / mem::size_of::<Term>();
        let extra = bin_size % mem::size_of::<Term>();
        if extra > 0 {
            bin_words + 1
        } else {
            bin_words
        }
    }

    /// Adds the given `ProcBin` to the virtual binary heap
    ///
    /// Returns a box `Term` which wraps the pointer to the binary,
    /// and should be placed somewhere on the process heap to ensure
    /// the binary is not leaked
    #[inline]
    pub fn push(&mut self, bin: ProcBin) -> Term {
        let term = unsafe { bin.as_term() };
        let size = bin.size();
        self.bins
            .push_front(unsafe { UnsafeRef::from_raw(bin.into_raw()) });
        self.size += size;
        term
    }

    /// Removes the pointed-to `ProcBin` from the virtual binary heap
    ///
    /// Returns the `ProcBin` indicated, which can either be dropped,
    /// or placed on a new virtual heap, whichever is desired.
    ///
    /// NOTE: This operation is intended to mirror `push`, do not
    /// use it under any other circumstances
    #[inline]
    pub fn pop(&mut self, bin: *mut ProcBin) -> ProcBin {
        let mut cursor = unsafe { self.bins.cursor_mut_from_ptr(bin) };
        let raw = cursor.remove().unwrap();
        let bin = unsafe { ProcBin::from_raw_noincrement(UnsafeRef::into_raw(raw)) };
        let size = bin.size();
        self.size -= size;
        bin
    }
}
