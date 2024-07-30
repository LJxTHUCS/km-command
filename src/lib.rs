//! Command Defination of Kernel Model Check.
#![no_std]

/// Memory-related commands.
pub mod mem;

/// Define a command with a fixed command id.
#[macro_export]
macro_rules! command {
    (        
        $(#[$outer:meta])*
        struct $name:ident { 
            $($(#[doc=$d:expr])? $field:ident: $t:ty,)* 
        },
        $id:literal
    ) => {
        $(#[$outer])*
        #[repr(C)]
        pub struct $name {
            /// Command id
            id: usize,
            $(
                $(#[doc=$d])? 
                pub $field: $t
            ),*
        }

        impl $name {
            /// Command id.
            pub const ID: usize = $id;
            /// Create a new command.
            pub fn new($($field: $t),*) -> Self {
                Self {
                    id: $id,
                    $($field,)*
                }
            }
        }
    };
}

