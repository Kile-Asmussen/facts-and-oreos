#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unsafe_op_in_unsafe_fn)]

use std::os::raw::c_int;

pub mod lua52;
pub use lua52::*;

extern crate libc;

#[doc(hidden)]
pub const LUA_MAX_UPVALUES: c_int = 255;

#[doc(hidden)]
pub const LUA_TRACEBACK_STACK: c_int = 11;

#[doc(hidden)]
#[rustfmt::skip]
pub const SYS_MIN_ALIGN: usize = if cfg!(any(
    all(target_arch = "riscv32", any(target_os = "espidf", target_os = "zkvm")),
    all(target_arch = "xtensa", target_os = "espidf"),
)) {
    4
} else if cfg!(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "m68k",
    target_arch = "csky",
    target_arch = "loongarch32",
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "sparc",
    target_arch = "wasm32",
    target_arch = "hexagon",
    target_arch = "riscv32",
    target_arch = "xtensa",
)) {
    8
} else if cfg!(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "loongarch64",
    target_arch = "mips64",
    target_arch = "mips64r6",
    target_arch = "s390x",
    target_arch = "sparc64",
    target_arch = "riscv64",
    target_arch = "wasm64",
)) {
    16
} else {
    panic!("no value for SYS_MIN_ALIGN")
};
