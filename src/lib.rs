#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "allocator_api", feature(allocator_api, alloc_layout_extra))]
#![warn(missing_docs)]

#[cfg(feature = "llff")]
mod llff;
#[cfg(feature = "tlsf")]
mod tlsf;

#[cfg(feature = "llff")]
pub use llff::Heap as LlffHeap;
#[cfg(feature = "tlsf")]
pub use tlsf::Heap as TlsfHeap;
