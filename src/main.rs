fn main() {}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use flua_mlua_sys::lua52::{lauxlib, lualib, lua};

    #[test]
    fn d5_smoke_test() {
        unsafe {
            // D5a: open a FLua state and execute trivial Lua
            let l = lauxlib::luaL_newstate();
            assert!(!l.is_null(), "luaL_newstate returned null");
            lualib::luaL_openlibs(l);

            let code = c"return 1 + 1";
            let status = lauxlib::luaL_loadstring(l, code.as_ptr());
            assert_eq!(status, lua::LUA_OK, "luaL_loadstring failed");
            let status = lua::lua_pcall(l, 0, 1, 0);
            assert_eq!(status, lua::LUA_OK, "lua_pcall failed");
            let result = lua::lua_tonumber(l, -1);
            assert_eq!(result, 2.0, "1 + 1 should equal 2");
            lua::lua_pop(l, 1);

            // D5b: call lua_getlfield via FFI
            lua::lua_pushglobaltable(l);
            let key = c"math";
            lua::lua_getlfield(l, -1, key.as_ptr(), key.to_bytes().len());
            assert_eq!(lua::lua_type(l, -1), lua::LUA_TTABLE, "math global should be a table");
            lua::lua_pop(l, 2);

            // D5c: check tostring(2^10) output
            let code = c"return tostring(2^10)";
            lauxlib::luaL_loadstring(l, code.as_ptr());
            lua::lua_pcall(l, 0, 1, 0);
            let s = CStr::from_ptr(lua::lua_tostring(l, -1))
                .to_str()
                .expect("valid utf8")
                .to_owned();
            lua::lua_pop(l, 1);

            lua::lua_close(l);

            println!("tostring(2^10) = {:?}", s);
            // FLua uses %.14g format: 2^10 = 1024.0 as a double prints as "1024"
            assert_eq!(s, "1024", "unexpected number formatting from FLua");
        }
    }
}
