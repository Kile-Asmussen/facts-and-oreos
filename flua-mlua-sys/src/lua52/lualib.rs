//! FFI bindings from `lualib.h`.

use std::os::raw::{c_char, c_int};

use super::lua::lua_State;

pub const LUA_COLIBNAME: *const c_char = c"coroutine".as_ptr();
pub const LUA_TABLIBNAME: *const c_char = c"table".as_ptr();
pub const LUA_IOLIBNAME: *const c_char = c"io".as_ptr();
pub const LUA_OSLIBNAME: *const c_char = c"os".as_ptr();
pub const LUA_STRLIBNAME: *const c_char = c"string".as_ptr();
pub const LUA_BITLIBNAME: *const c_char = c"bit32".as_ptr();
pub const LUA_MATHLIBNAME: *const c_char = c"math".as_ptr();
pub const LUA_DBLIBNAME: *const c_char = c"debug".as_ptr();
pub const LUA_LOADLIBNAME: *const c_char = c"package".as_ptr();

unsafe extern "C-unwind" {
    pub fn luaopen_base(L: *mut lua_State) -> c_int;
    pub fn luaopen_coroutine(L: *mut lua_State) -> c_int;
    pub fn luaopen_table(L: *mut lua_State) -> c_int;
    pub fn luaopen_io(L: *mut lua_State) -> c_int;
    pub fn luaopen_os(L: *mut lua_State) -> c_int;
    pub fn luaopen_string(L: *mut lua_State) -> c_int;
    pub fn luaopen_bit32(L: *mut lua_State) -> c_int;
    pub fn luaopen_math(L: *mut lua_State) -> c_int;
    pub fn luaopen_debug(L: *mut lua_State) -> c_int;
    pub fn luaopen_package(L: *mut lua_State) -> c_int;
    pub fn luaL_openlibs(L: *mut lua_State);
}
