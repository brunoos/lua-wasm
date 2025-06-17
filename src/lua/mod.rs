use std::os::raw::{c_int, c_void};

mod lua54;

pub use lua54::lua_State;
use lua54::{
    lua_CFunction, lua_createtable, lua_pushlightuserdata,
    lua_pushcclosure, lua_pushlstring, lua_rawset, lua_type,
    lua_touserdata, lua_tolstring, lua_pushnil,
    LUA_TUSERDATA, LUA_TLIGHTUSERDATA
};

#[repr(transparent)]
pub struct LuaState(*mut lua_State);

impl LuaState {
    pub fn isuserdata(self: &LuaState, idx: i32) -> bool {
        unsafe {
            lua_type(self.0, idx as c_int) == (LUA_TUSERDATA as c_int) ||
            lua_type(self.0, idx as c_int) == (LUA_TLIGHTUSERDATA as c_int)
        }
    }

    pub fn new(state: *mut lua_State) -> Self {
        LuaState(state)
    }

    pub fn newtable(&self) {
        unsafe {
            lua_createtable(self.0, 0,0 );
        }
    }
    
    pub fn pushcfunction(self: &LuaState, f: lua_CFunction) {
        unsafe {
            lua_pushcclosure(self.0, f, 0);
        }
    }

    pub fn pushlightuserdata(&self, p: *mut c_void) {
        unsafe {
            lua_pushlightuserdata(self.0, p);
        }
    }

    pub fn pushnil(self: &LuaState) {
        unsafe {
            lua_pushnil(self.0);
        }
    }

    pub fn pushstring(self: &LuaState, str: &str) {
        unsafe {
            lua_pushlstring(
                self.0,
                str.as_ptr() as *const i8,
                str.len() as usize,
            );
        }
    }

    pub fn rawset(self: &LuaState, idx: i32) {
        unsafe {
            lua_rawset(self.0, idx as c_int);
        }
    }

    pub fn tobytes(&self, idx: i32) -> Option<&[u8]> {
        let mut len: usize = 0;
        unsafe {
            let ptr = lua_tolstring(self.0, idx as c_int, &mut len) as *const u8;
            if ptr.is_null() {
                None
            } else {
                Some(std::slice::from_raw_parts(ptr, len))
            }
        }
    }

    pub fn touserdata(&self, idx: i32) -> *mut c_void {
        unsafe {
            lua_touserdata(self.0, idx as c_int)
        }
    }

}

//------------------------------------------------------------------------------

#[macro_export]
macro_rules! derive_cfunction {
    ($name:ident) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $name(state: *mut crate::lua::lua_State) -> std::ffi::c_int {
            let mut state = crate::lua::LuaState::new(state);
            return super::$name(&mut state) as std::ffi::c_int;
        }
    };
}

#[macro_export]
macro_rules! derive_cfunctions {
    ($($ident:ident),*) => {
        mod cfunctions {
            $(
                crate::derive_cfunction!($ident);
            )*
        }
    };
}

//------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn luaopen_wasm_core(state: *mut lua_State) -> c_int {
    let state = LuaState::new(state);
    crate::init_wasm_core(&state)
}