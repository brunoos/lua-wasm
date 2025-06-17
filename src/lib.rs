use std::os::raw::c_void;
use wasmtime::{Engine,Module};

mod lua;
use lua::LuaState;

fn meth_create_engine(state: &LuaState) -> i32 {
    let engine = Engine::default();
    state.pushlightuserdata(Box::into_raw(Box::new(engine)) as *mut c_void);
    return 1;
}

fn meth_destroy_engine(state: &LuaState) -> i32 {
    if !state.isuserdata(1) {
        return 0;
    }
    let engine_ptr = state.touserdata(1) as *mut Engine;
    if !engine_ptr.is_null() {
        unsafe {
            drop(Box::from_raw(engine_ptr));
        }
    }
    return 0;
}

fn meth_create_module(state: &LuaState) -> i32 {
    if !state.isuserdata(1) {
        state.pushnil();
        return 1;
    }
    let engine_ptr = state.touserdata(1) as *mut Engine;
    if engine_ptr.is_null() {
        state.pushnil();
        return 1;
    }
    let engine = unsafe { &*engine_ptr };
    let data = state.tobytes(2);
    if data.is_none() {
        state.pushnil();
        return 1;
    }
    let module = Module::new(&engine, data.unwrap());
    if module.is_err() {
        state.pushnil();
        return 1;
    }
    state.pushlightuserdata(Box::into_raw(Box::new(module.unwrap())) as *mut c_void);
    return 1;
}

// create a function 'meth_destroy_module'
fn meth_destroy_module(state: &LuaState) -> i32 {
    if !state.isuserdata(1) {
        return 0;
    }
    let module_ptr = state.touserdata(1) as *mut Module;
    if !module_ptr.is_null() {
        unsafe {
            drop(Box::from_raw(module_ptr));
        }
    }
    return 0;
}

//------------------------------------------------------------------------------

derive_cfunctions!(
    meth_create_engine,
    meth_destroy_engine,
    meth_create_module,
    meth_destroy_module
);

//------------------------------------------------------------------------------

fn init_wasm_core(state: &LuaState) -> i32 {
    state.newtable();

    state.pushstring("create_engine");
    state.pushcfunction(Some(cfunctions::meth_create_engine));
    state.rawset(-3);

    state.pushstring("destroy_engine");
    state.pushcfunction(Some(cfunctions::meth_destroy_engine));
    state.rawset(-3);

    state.pushstring("create_module");
    state.pushcfunction(Some(cfunctions::meth_create_module));
    state.rawset(-3);

    state.pushstring("destroy_module");
    state.pushcfunction(Some(cfunctions::meth_destroy_module));
    state.rawset(-3);
    
    return 1;
}