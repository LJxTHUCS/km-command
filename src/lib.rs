//! Command Defination of Kernel Model Check.

// Harness runs on no_std environment
#![cfg_attr(feature = "harness", no_std)]

// Check features
#[cfg(all(feature = "checker", feature = "harness"))]
compile_error!("Cannot enable both `checker` and `harness` features at the same time.");

/// Memory-related commands.
pub mod mem;

/// Define a command with a fixed command id. Implement `Debug`,
/// `Serialize`, and `Deserialize` for the command.
///
/// If a serialization format is specified, methods that serialize
/// the command into a byte array and deserialize the command from
/// a byte array are also implemented.
///
/// Format: `command!(struct $name { ... }, $id)`
#[macro_export]
macro_rules! command {
    (
        $(#[$outer:meta])*
        struct $name:ident {
            $($(#[$attr:meta])* $field:ident: $t:ty,)*
        },
        $id:literal
    ) => {
        $(#[$outer])*
        #[derive(Debug, Serialize, Deserialize)]
        #[repr(C)]
        pub struct $name {
            $($(#[$attr])* pub $field: $t),*
        }

        impl $name {
            /// Command id.
            pub const ID: usize = $id;

            /// Create a new command.
            pub fn new($($field: $t),*) -> Self {
                Self {
                    $($field,)*
                }
            }
        }

        #[cfg(feature = "checker")]
        impl $name {
            /// Serialize the command into a byte array
            pub fn to_bytes(&self) -> Vec<u8> {
                if cfg!(feature = "postcard") {
                    postcard::to_allocvec(self).unwrap()
                } else {
                    Vec::new()
                }
            }
        }

        #[cfg(feature = "harness")]
        impl $name {
            /// Deserialize the command from a byte array, return the command and the remaining data.
            pub fn from_bytes(data: &[u8]) -> Option<(Self, &[u8])> {
                if cfg!(feature = "postcard") {
                    postcard::take_from_bytes::<Self>(data).ok()
                } else {
                    None
                }
            }
        }
    };
}

/// Serialize command id.
#[cfg(feature = "checker")]
pub fn id_to_bytes(id: usize) -> Vec<u8> {
    id.to_le_bytes().to_vec()
}

/// Deserialize command id, return the id and the remaining data.
#[cfg(feature = "harness")]
pub fn id_from_bytes(data: &[u8]) -> (usize, &[u8]) {
    let id = usize::from_le_bytes(data[..core::mem::size_of::<usize>()].try_into().unwrap());
    (id, &data[core::mem::size_of::<usize>()..])
}
