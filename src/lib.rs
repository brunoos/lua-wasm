use std::os::raw::c_void;
use wasmtime::{Engine, Module, Linker, Store};

mod lua;
use lua::LuaState;

fn meth_create_engine(state: &LuaState) -> i32 {
    let engine = Engine::default();
    state.pushlightuserdata(Box::into_raw(Box::new(engine)) as *mut c_void);
    return 1;
}

fn meth_destroy_engine(state: &LuaState) -> i32 {
    let engine_ptr = state.touserdata(1) as *mut Engine;
    if !engine_ptr.is_null() {
        unsafe {
            drop(Box::from_raw(engine_ptr));
        }
    }
    return 0;
}

fn meth_create_module(state: &LuaState) -> i32 {
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

fn meth_destroy_module(state: &LuaState) -> i32 {
    let module_ptr = state.touserdata(1) as *mut Module;
    if !module_ptr.is_null() {
        unsafe {
            drop(Box::from_raw(module_ptr));
        }
    }
    return 0;
}

fn meth_create_linker(state: &LuaState) -> i32 {
    let engine_ptr = state.touserdata(1) as *mut Engine;
    if engine_ptr.is_null() {
        state.pushnil();
        return 1;
    }
    let engine = unsafe { &*engine_ptr };
    let linker: Linker<u32> = Linker::new(engine);
    state.pushlightuserdata(Box::into_raw(Box::new(linker)) as *mut c_void);
    return 1;
}

fn meth_destroy_linker(state: &LuaState) -> i32 {
    let linker_ptr = state.touserdata(1) as *mut Linker<u32>;
    if !linker_ptr.is_null() {
        unsafe {
            drop(Box::from_raw(linker_ptr));
        }
    }
    return 0;
}

fn meth_create_instance(state: &LuaState) -> i32 {
    let engine_ptr = state.touserdata(1) as *mut Engine;
    if engine_ptr.is_null() {
        state.pushnil();
        return 1;
    }
    let engine = unsafe { &*engine_ptr };    

    let linker_ptr = state.touserdata(2) as *mut Linker<u32>;
    if linker_ptr.is_null() {
        state.pushnil();
        return 1;
    }
    let linker = unsafe { &*linker_ptr };

    let module_ptr = state.touserdata(3) as *mut Module;
    if module_ptr.is_null() {
        state.pushnil();
        return 1;
    }
    let module = unsafe { &*module_ptr };

    let mut store: Store<u32> = Store::new(engine, 0);
    let instance = linker.instantiate(&mut store, module);
    if instance.is_err() {
        state.pushnil();
        return 1;
    }

    state.pushlightuserdata(Box::into_raw(Box::new(instance.unwrap())) as *mut c_void);
    return 1;
}

fn meth_destroy_instance(state: &LuaState) -> i32 {
    let instance_ptr = state.touserdata(1) as *mut wasmtime::Instance;
    if !instance_ptr.is_null() {
        unsafe {
            drop(Box::from_raw(instance_ptr));
        }
    }
    return 0;
}

//------------------------------------------------------------------------------

derive_cfunctions!(
    meth_create_engine,
    meth_destroy_engine,
    meth_create_module,
    meth_destroy_module,
    meth_create_linker,
    meth_destroy_linker,
    meth_create_instance,
    meth_destroy_instance
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

    state.pushstring("create_linker");
    state.pushcfunction(Some(cfunctions::meth_create_linker));
    state.rawset(-3);

    state.pushstring("destroy_linker");
    state.pushcfunction(Some(cfunctions::meth_destroy_linker));
    state.rawset(-3);

    state.pushstring("create_instance");
    state.pushcfunction(Some(cfunctions::meth_create_instance));
    state.rawset(-3);

    state.pushstring("destroy_instance");
    state.pushcfunction(Some(cfunctions::meth_destroy_instance));
    state.rawset(-3);

    return 1;
}