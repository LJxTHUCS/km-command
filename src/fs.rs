use crate::command;
use bitflags::bitflags;
use core::{
    ops::{Deref, DerefMut},
    str::{self, FromStr},
};
use heapless::String;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Max file path length.
pub const MAX_PATH_LEN: usize = 256;

/// Path type - a fixed capacity string.
#[derive(Debug, Clone)]
pub struct Path(pub String<MAX_PATH_LEN>);

impl Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <&str>::deserialize(deserializer).map(|s| Path(String::from_str(s).unwrap()))
    }
}

impl Deref for Path {
    type Target = String<MAX_PATH_LEN>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Path {
    /// Check if the path is absolute.
    pub fn absolute(&self) -> bool {
        self.0.starts_with('/')
    }
    /// Check if the path is relative.
    pub fn relative(&self) -> bool {
        !self.absolute()
    }
}

command!(
    /// Open and possibly create a file.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/open.2.html
    struct Openat {
        /// The file descriptor of the directory to open the file in.
        dirfd: isize,
        /// The path to the file to open.
        path: Path,
        /// The flags to use when opening the file.
        flags: OpenFlags,
        /// The mode to use when creating the file.
        mode: FileMode,
    },
    56
);

command!(
    /// Close a file descriptor.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/close.2.html
    struct Close {
        /// The file descriptor to close.
        fd: isize,
    },
    57
);

command!(
    /// Get file status.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/fstat.2.html
    struct Fstat {
        /// The file descriptor to get the status of.
        fd: isize,
    },
    80
);

command!(
    /// Get directory entrys. This command returns all the directory
    /// entries in the directory. Harness may call `getdents` multiple
    /// times to get all the directory entries.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/getdents.2.html
    struct Getdents {
        /// The file descriptor to get directory entries from.
        fd: isize,
        /// The expected number of dentries to get.
        count: usize,
    },
    61
);

command!(
    /// Make a new name for a file.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/link.2.html
    struct Linkat {
        /// File descriptor of the old directory.
        olddirfd: isize,
        /// The old path.
        oldpath: Path,
        /// File descriptor of the new directory.
        newdirfd: isize,
        /// The new path.
        newpath: Path,
    },
    37
);

command!(
    /// Delete a name and possibly the file it refers to.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/unlink.2.html
    struct Unlinkat {
        /// File descriptor of the directory.
        dirfd: isize,
        /// The path of the file to delete.
        path: Path,
    },
    35
);

command!(
    /// Create a directory.
    ///
    /// https://man7.org/linux/man-pages/man2/mkdirat.2.html
    struct Mkdirat {
        /// File descriptor of the directory in which the new
        /// directory is to be created.
        dirfd: isize,
        /// The path of the new directory.
        path: Path,
        /// The mode of the new directory.
        mode: FileMode,
    },
    34
);

command!(
    /// Get current working directory.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/getcwd.2.html
    struct Getcwd {},
    17
);

command!(
    /// Duplicate a file descriptor.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/dup.2.html
    struct Dup {
        /// The file descriptor to be duplicated.
        oldfd: isize,
    },
    23
);

command!(
    /// Changes the current working directory of the calling
    /// process to the directory specified in path.
    ///
    /// Ref: https://man7.org/linux/man-pages/man2/chdir.2.html
    struct Chdir {
        /// The path to the directory to change to.
        path: Path,
    },
    49
);

bitflags! {
    /// Flags for the `Open` command.
    #[derive(Debug, Clone, Copy)]
    pub struct OpenFlags: u32 {
        /// Open for reading only.
        const RDONLY = 0o00000000;
        /// Open for writing only.
        const WRONLY = 0o00000001;
        /// Open for reading and writing.
        const RDWR = 0o00000002;
        /// Create file if it does not exist.
        const CREAT = 0o00000100;
        /// Append data to the file.
        const APPEND = 0o00000200;
        /// Truncate file to size 0.
        const TRUNC = 0o00000400;
        /// Expect to open a directory.
        const DIRECTORY = 0o01000000;
    }
}

impl Serialize for OpenFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for OpenFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(OpenFlags::from_bits_truncate(u32::deserialize(
            deserializer,
        )?))
    }
}

bitflags! {
    /// File mode.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FileMode: u32 {
        /// User readable.
        const USER_READ = 0o400;
        /// User writable.
        const USER_WRITE = 0o200;
        /// User executable.
        const USER_EXEC = 0o100;
        /// Group readable.
        const GROUP_READ = 0o040;
        /// Group writable.
        const GROUP_WRITE = 0o020;
        /// Group executable.
        const GROUP_EXEC = 0o010;
        /// Other readable.
        const OTHER_READ = 0o004;
        /// Other writable.
        const OTHER_WRITE = 0o002;
        /// Other executable.
        const OTHER_EXEC = 0o001;
    }
}

impl Serialize for FileMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for FileMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(FileMode::from_bits_truncate(u32::deserialize(
            deserializer,
        )?))
    }
}

/// File kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FileKind {
    File,
    Directory,
}

/// File status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct FileStat {
    /// inode number.
    pub ino: usize,
    /// File mode.
    pub mode: FileMode,
    /// File size in bytes.
    pub uid: u32,
    /// Group ID.
    pub gid: u32,
    /// File kind.
    pub kind: FileKind,
    /// Link count.
    pub nlink: usize,
}

/// Directory entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct DirEntry {
    /// Inode number.
    pub ino: usize,
    /// Name length.
    pub name_len: usize,
    /// Name.
    pub name: [u8; 256],
}

impl DirEntry {
    /// Parse name as str.
    pub fn name(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.name[..self.name_len]) }
    }
}
