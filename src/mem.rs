use crate::command;
use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

command! {
    /// [`Brk`] and [`Sbrk] change the location of the program break,
    /// which defines the end of the process's data segment.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/brk.2.html
    #[derive(Debug, Serialize, Deserialize)]
    struct Brk {
        /// The new program break.
        addr: usize,
    },
    214
}

command! {
    /// Like `brk`, but return the old program break on success.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/brk.2.html
    #[derive(Debug, Serialize, Deserialize)]
    struct Sbrk {
        /// The increment to the program break.
        increment: isize,
    },
    214
}

command! {
    /// [`Mmap`] creates a new mapping in the virtual address
    /// space of the calling process.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/mmap.2.html
    #[derive(Debug, Serialize, Deserialize)]
    struct Mmap {
        /// The starting address of the mapping.
        addr: usize,
        /// The length of the mapping.
        len: usize,
        /// Memory protection of the mapping.
        prot: ProtFlags,
        /// Mapping flags
        flags: MmapFlags,
        // File-related fields
    },
    222
}

command! {
    /// [`Munmap`] removes a mapping from the virtual address
    /// space of the calling process.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/munmap.2.html
    #[derive(Debug, Serialize, Deserialize)]
    struct Munmap {
        /// The starting address of unmapping.
        addr: usize,
        /// The length of unmapping.
        len: usize,
    },
    215
}

command! {
    /// [`Mprotect`] changes the access protections for the calling
    /// process's memory pages containing any part of the address range
    /// in the interval [addr, addr+len-1].
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/mprotect.2.html
    #[derive(Debug, Serialize, Deserialize)]
    struct Mprotect {
        /// The starting address of protection.
        start: usize,
        /// The length of protection.
        len: usize,
        /// The protection flags.
        flags: ProtFlags,
    },
    5
}

bitflags! {
    /// Generic page table entry flags that indicate the corresponding mapped
    /// memory region permissions and attributes.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ProtFlags: u8 {
        /// The memory is readable.
        const READ          = 1 << 0;
        /// The memory is writable.
        const WRITE         = 1 << 1;
        /// The memory is executable.
        const EXECUTE       = 1 << 2;
    }
}

impl Serialize for ProtFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.bits())
    }
}

impl<'de> Deserialize<'de> for ProtFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(ProtFlags::from_bits_truncate(u8::deserialize(
            deserializer,
        )?))
    }
}

bitflags! {
    /// `MmapFlags` determines whether updates to the mapping are
    /// visible to other processes mapping the same region, and whether
    /// updates are carried through to the underlying file.
    #[derive(Debug)]
    pub struct MmapFlags: u32 {
        /// Modifications to this memory are shared
        const MAP_SHARED = 1 << 0;
        /// Modifications to this memory are private
        const MAP_PRIVATE = 1 << 1;
        /// Don't interpret addr as a hint: place the mapping at
        /// exactly that address.
        const MAP_FIXED = 1 << 4;
    }
}

impl Serialize for MmapFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for MmapFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(MmapFlags::from_bits_truncate(u32::deserialize(
            deserializer,
        )?))
    }
}
