//! The `fdisk` library allows to use the functions of the fdisk command.

#![feature(exclusive_range_pattern)]
#![feature(iter_array_chunks)]

pub mod crc32;
pub mod disk;
pub mod guid;
pub mod partition;
