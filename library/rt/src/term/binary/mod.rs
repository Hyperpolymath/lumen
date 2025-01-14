mod flags;
mod iter;
mod slice;
mod traits;
mod writer;

pub use self::flags::{BinaryFlags, Encoding};
pub use self::iter::ByteIter;
pub use self::slice::BitSlice;
pub use self::traits::{Aligned, Binary, Bitstring};
pub use self::writer::BinaryWriter;

use core::any::TypeId;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{Index, IndexMut};
use core::slice::SliceIndex;

use liblumen_alloc::rc::Rc;

/// This represents binary data, i.e. byte-aligned, with a number of bits
/// divisible by 8 evenly.
pub struct BinaryData {
    flags: BinaryFlags,
    data: [u8],
}
impl<I> Index<I> for BinaryData
where
    I: SliceIndex<[u8]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        index.index(&self.data)
    }
}
impl<I> IndexMut<I> for BinaryData
where
    I: SliceIndex<[u8]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_mut(&mut self.data)
    }
}
impl BinaryData {
    pub const TYPE_ID: TypeId = TypeId::of::<BinaryData>();

    /// The maximum size of a binary stored on a process heap, in bytes
    pub const MAX_HEAP_BYTES: usize = 64;

    /// Overrides the flags/metadata of this binary data
    pub unsafe fn set_flags(&mut self, flags: BinaryFlags) {
        // We force the size value of the provided flags to match the actual size
        self.flags = flags.with_size(self.data.len());
    }

    /// Returns the size in bytes of the underlying data
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns a BinaryWriter which can write to this binary.
    #[inline]
    pub fn write<'a>(&'a mut self) -> BinaryWriter<'a> {
        BinaryWriter::new(&mut self.data)
    }

    /// Copies the bytes from the given slice into `self`
    ///
    /// NOTE: The length of the slice must match the capacity of `self`, or the function will panic
    pub fn copy_from_slice(&mut self, bytes: &[u8]) {
        assert_eq!(self.len(), bytes.len());
        self.data.copy_from_slice(bytes)
    }

    /// Constructs an Rc<BinaryData> from the given string.
    ///
    /// The encoding of the resulting BinaryData is always UTF-8.
    ///
    /// NOTE: This function always allocates via Rc, even if the binary is smaller than 64 bytes.
    pub fn from_str(s: &str) -> Rc<BinaryData> {
        let bytes = s.as_bytes();
        let mut rcbox = Rc::<BinaryData>::with_capacity(bytes.len());
        {
            let value = unsafe { Rc::get_mut_unchecked(&mut rcbox) };
            value.flags = BinaryFlags::new(bytes.len(), Encoding::Utf8);
            value.copy_from_slice(bytes);
        }
        rcbox
    }

    /// Constructs an Rc<BinaryData> from the given byte slice.
    ///
    /// The encoding of the given data is detected by examining the bytes. If you
    /// wish to construct a binary from a byte slice with a manually-specified encoding, use
    /// `from_bytes_with_encoding`.
    ///
    /// NOTE: This function always allocates via Rc, even if the binary is smaller than 64 bytes.
    pub fn from_bytes(bytes: &[u8]) -> Rc<BinaryData> {
        let encoding = Encoding::detect(bytes);
        unsafe { Self::from_bytes_with_encoding(bytes, encoding) }
    }

    /// Constructs an Rc<BinaryData> from the given byte slice.
    ///
    /// # Safety
    ///
    /// This function is unsafe because specifying the wrong encoding may violate invariants
    /// of those encodings assumed by other runtime functions. The caller must be sure that
    /// the given bytes are valid for the specified encoding, preferably by having run validation
    /// checks in a previous step.
    pub unsafe fn from_bytes_with_encoding(bytes: &[u8], encoding: Encoding) -> Rc<BinaryData> {
        let mut rcbox = Rc::<BinaryData>::with_capacity(bytes.len());
        {
            let value = Rc::get_mut_unchecked(&mut rcbox);
            value.flags = BinaryFlags::new(bytes.len(), encoding);
            value.copy_from_slice(bytes);
        }
        rcbox
    }

    #[inline]
    pub fn full_byte_iter<'a>(&'a self) -> core::iter::Copied<core::slice::Iter<'a, u8>> {
        self.data.iter().copied()
    }
}
impl fmt::Debug for BinaryData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl fmt::Display for BinaryData {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_binary(self, f)
    }
}
impl Eq for BinaryData {}
impl PartialEq for BinaryData {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl<T: Bitstring> PartialEq<T> for BinaryData {
    fn eq(&self, other: &T) -> bool {
        // An optimization: we can say for sure that if the sizes don't match,
        // the slices don't either.
        if self.bit_size() != other.bit_size() {
            return false;
        }

        // If both slices are aligned binaries, we can compare their data directly
        if other.is_aligned() && other.is_binary() {
            return self.data.eq(unsafe { other.as_bytes_unchecked() });
        }

        // Otherwise we must fall back to a byte-by-byte comparison
        self.bytes().eq(other.bytes())
    }
}
impl Ord for BinaryData {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.data.cmp(&other.data)
    }
}
impl PartialOrd for BinaryData {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Bitstring> PartialOrd<T> for BinaryData {
    // We order bitstrings lexicographically
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        // Aligned binaries can be compared using the optimal built-in slice comparisons in the standard lib
        if other.is_aligned() && other.is_binary() {
            return Some(self.data.cmp(unsafe { other.as_bytes_unchecked() }));
        }

        // Otherwise we must comapre byte-by-byte
        Some(self.bytes().cmp(other.bytes()))
    }
}

impl Hash for BinaryData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash_slice(&self.data, state);
    }
}
impl Bitstring for BinaryData {
    #[inline]
    fn byte_size(&self) -> usize {
        self.len()
    }

    #[inline]
    fn bit_size(&self) -> usize {
        self.len() * 8
    }

    #[inline(always)]
    fn is_aligned(&self) -> bool {
        true
    }

    #[inline(always)]
    fn is_binary(&self) -> bool {
        true
    }

    #[inline]
    unsafe fn as_bytes_unchecked(&self) -> &[u8] {
        &self.data
    }
}
impl Binary for BinaryData {
    #[inline]
    fn flags(&self) -> BinaryFlags {
        self.flags
    }
}
impl Aligned for BinaryData {}

/// Displays a raw binary using Erlang-style formatting
pub(crate) fn display_binary<B: Binary + Aligned>(bin: B, f: &mut fmt::Formatter) -> fmt::Result {
    use core::fmt::Write;

    if let Some(s) = bin.as_str() {
        f.write_str("<<\"")?;
        for c in s.escape_default() {
            f.write_char(c)?;
        }
        f.write_str("\">>")
    } else {
        display_raw_bytes(bin.as_bytes(), f)
    }
}

/// Formats raw bytes using Erlang-style formatting
pub(crate) fn display_raw_bytes(bytes: &[u8], f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("<<")?;

    let mut iter = bytes.iter().copied();

    let Some(byte) = iter.next() else { return Ok(()); };
    write!(f, "{}", byte)?;

    for byte in iter {
        write!(f, ",{}", byte)?;
    }

    f.write_str(">>")
}
