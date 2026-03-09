// This is free and unencumbered software released into the public domain.

#![allow(non_camel_case_types)]

pub use core::ffi::CStr as Str;
pub use core::ffi::{FromBytesUntilNulError, FromBytesWithNulError};

pub use core::ffi::c_char as char;
pub use core::ffi::c_double as double;
pub use core::ffi::c_float as float;
pub use core::ffi::c_int as int;
pub use core::ffi::c_long as long;
pub use core::ffi::c_longlong as longlong;
pub use core::ffi::c_schar as schar;
pub use core::ffi::c_short as short;
pub use core::ffi::c_uchar as uchar;
pub use core::ffi::c_uint as uint;
pub use core::ffi::c_ulong as ulong;
pub use core::ffi::c_ulonglong as ulonglong;
pub use core::ffi::c_ushort as ushort;
pub use core::ffi::c_void as void;

#[cfg(feature = "nightly")]
pub use core::ffi::c_ptrdiff_t as ptrdiff_t;
#[cfg(feature = "nightly")]
pub use core::ffi::c_size_t as size_t;
#[cfg(feature = "nightly")]
pub use core::ffi::c_ssize_t as ssize_t;

pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type intmax_t = i64;

pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type uintmax_t = u64;
