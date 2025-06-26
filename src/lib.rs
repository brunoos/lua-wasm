use wasmtime::{
    Engine, ExternType, Func, Instance, Linker, Module, Store, Val, ValType,
    Mutability,
};

mod lua;
use lua::{LuaState, LuaNumber, LuaInteger, RawLuaState};


fn meth_create_engine(state: &LuaState) -> i32 {
    let engine = Engine::default();
    state.push_typed_lightuserdata(engine);
    return 1;
}

fn meth_destroy_engine(state: &LuaState) -> i32 {
    if let Some(engine) = state.to_raw_typed_userdata::<Engine>(1) {
        unsafe { drop(Box::from_raw(engine)); }
    }
    return 0;
}

fn meth_create_module(state: &LuaState) -> i32 {
    let engine = match state.to_typed_userdata::<Engine>(1) {
        Some(e) => e,
        None => return 0,
    };
    let data = state.tobytes(2);
    if data.is_none() {
        return 0;
    }
    let module = Module::new(&engine, data.unwrap());
    if module.is_err() {
        return 0;
    }
    state.push_typed_lightuserdata(module.unwrap());
    return 1;
}

fn meth_destroy_module(state: &LuaState) -> i32 {
    if let Some(module) = state.to_raw_typed_userdata::<Module>(1) {
        unsafe { drop(Box::from_raw(module)); }
    }
    return 0;
}

fn meth_create_linker(state: &LuaState) -> i32 {
    let engine = match state.to_typed_userdata::<Engine>(1) {
        Some(e) => e,
        None => return 0,
    };
    let linker: Linker<*mut RawLuaState> = Linker::new(engine);
    state.push_typed_lightuserdata(linker);
    return 1;
}

fn meth_destroy_linker(state: &LuaState) -> i32 {
    if let Some(linker) = state.to_raw_typed_userdata::<Linker<*mut RawLuaState>>(1) {
        unsafe { drop(Box::from_raw(linker)); }
    }
    return 0;
}

fn meth_create_instance(state: &LuaState) -> i32 {
    let linker = match state.to_typed_userdata::<Linker<*mut RawLuaState>>(1) {
        Some(l) => l,
        None => return 0,
    };

    let module = match state.to_typed_userdata::<Module>(2) {
        Some(m) => m,
        None => return 0,
    };

    let mut store = match state.to_typed_userdata::<Store<*mut RawLuaState>>(3) {
        Some(s) => s,
        None => return 0,
    };

    let instance = linker.instantiate(&mut store, module);
    if instance.is_err() {
        return 0;
    }

    state.push_typed_lightuserdata(instance.unwrap());
    return 1;
}

fn meth_destroy_instance(state: &LuaState) -> i32 {
    if let Some(instance) = state.to_raw_typed_userdata::<Instance>(1) {
        unsafe { drop(Box::from_raw(instance)); }
    }
    return 0;
}

fn meth_create_store(state: &LuaState) -> i32 {
    let engine = match state.to_typed_userdata::<Engine>(1) {
        Some(e) => e,
        None => return 0,
    };

    let store: Store<*mut RawLuaState> = Store::new(engine, state.as_raw());
    state.push_typed_lightuserdata(store);
    return 1;
}

fn meth_destroy_store(state: &LuaState) -> i32 {
    if let Some(store) = state.to_raw_typed_userdata::<Store<*mut RawLuaState>>(1) {
        unsafe { drop(Box::from_raw(store)); }
    }
    return 0;
}

fn meth_get_exports(state: &LuaState) -> i32 {
    let module = match state.to_typed_userdata::<Module>(1) {
        Some(module) => module,
        None => return 0,
    };

    state.newtable();
    for (n, export) in module.exports().enumerate() {
        let ty = export.ty();
        match ty {
            ExternType::Func(ft) => {
                state.pushnumber((n+1) as LuaNumber);
                state.newtable();

                state.pushstring("type");
                state.pushstring("function");
                state.rawset(-3);

                state.pushstring("name");
                state.pushstring(export.name());
                state.rawset(-3);

                state.pushstring("params");
                state.newtable();
                for (n, t) in ft.params().enumerate() {
                    state.pushnumber((n+1) as LuaNumber);
                    state.pushstring(t.to_string().as_str());
                    state.rawset(-3);
                };
                state.rawset(-3);

                state.pushstring("results");
                state.newtable();
                for (n, t) in ft.results().enumerate() {
                    state.pushnumber((n+1) as LuaNumber);
                    state.pushstring(t.to_string().as_str());
                    state.rawset(-3);
                };
                state.rawset(-3);

                state.rawset(-3);
            },
            ExternType::Global(gt) => {
                state.pushnumber((n+1) as LuaNumber);
                state.newtable();

                state.pushstring("type");
                state.pushstring("global");
                state.rawset(-3);

                state.pushstring("name");
                state.pushstring(export.name());
                state.rawset(-3);

                state.pushstring("mutable");
                state.pushboolean(gt.mutability() == Mutability::Var);
                state.rawset(-3);

                state.rawset(-3);
            },
            ExternType::Table(_) => {
                state.pushnumber((n+1) as LuaNumber);
                state.newtable();

                state.pushstring("type");
                state.pushstring("table");
                state.rawset(-3);

                state.pushstring("name");
                state.pushstring(export.name());
                state.rawset(-3);

                state.rawset(-3);
            },
            ExternType::Memory(_) => {
                state.pushnumber((n+1) as LuaNumber);
                state.newtable();

                state.pushstring("type");
                state.pushstring("memory");
                state.rawset(-3);

                state.pushstring("name");
                state.pushstring(export.name());
                state.rawset(-3);

                state.rawset(-3);
            },
            _ => {},
        }
    }

    return 1;
}

fn meth_get_export(state: &LuaState) -> i32 {
    let module = match state.to_typed_userdata::<Module>(1) {
        Some(module) => module,
        None => return 0,
    };

    let name = match state.tostring(2) {
        Some(name) => name,
        None => return 0,
    };

    for export in module.exports() {
        if export.name() != name {
            continue;
        }
        if let ExternType::Func(ft) = export.ty() {
            state.newtable();
            state.pushstring("type");
            state.pushstring("function");
            state.rawset(-3);

            state.pushstring("name");
            state.pushstring(export.name());
            state.rawset(-3);

            state.pushstring("params");
            state.newtable();
            for (n, t) in ft.params().enumerate() {
                state.pushnumber((n+1) as LuaNumber);
                state.pushstring(t.to_string().as_str());
                state.rawset(-3);
            };
            state.rawset(-3);

            state.pushstring("results");
            state.newtable();
            for (n, t) in ft.results().enumerate() {
                state.pushnumber((n+1) as LuaNumber);
                state.pushstring(t.to_string().as_str());
                state.rawset(-3);
            };
            state.rawset(-3);

            return 1;
        }
    }

    return 0;
}

fn meth_invoke(state: &LuaState) -> i32 {
    let instance = match state.to_typed_userdata::<Instance>(1) {
        Some(instance) => instance,
        None => return 0,
    };

    let name = match state.tostring(3) {
        Some(name) => name,
        None => return 0,
    };

    let func: Func;
    { 
        let store = match state.to_typed_userdata::<Store<*mut RawLuaState>>(2) {
            Some(store) => store,
            None => return 0,
        };

        func = match instance.get_func(store, &name) {
            Some(func) => func,
            None => return 0,
        }
    }

    let mut params = Vec::new();
    let mut results: Vec<Val> = Vec::new();

    {
        let store = state.to_typed_userdata::<Store<*mut RawLuaState>>(2).unwrap();
        let ft = func.ty(store);

        for (n , vt) in ft.params().enumerate() {
            match vt {
                ValType::I32 => {
                    match state.tointeger((n+4) as i32) {
                        Some(v) => {
                            params.push(Val::I32(v as i32));
                        },
                        None => return 0,
                    }
                },
                ValType::I64 => {
                    match state.tointeger((n+4) as i32) {
                        Some(v) => {
                            params.push(Val::I64(v))
                        },
                        None => return 0,
                    }
                },
                ValType::F32 => {
                    match state.tonumber((n+4) as i32) {
                        Some(v) => {
                            let v = u32::from_ne_bytes((v as f32).to_ne_bytes());
                            params.push(Val::F32(v));
                        },
                        None => return 0,
                    }
                },
                ValType::F64 => {
                    match state.tonumber((n+4) as i32) {
                        Some(v) => {
                            let v = u64::from_ne_bytes(v.to_ne_bytes());
                            params.push(Val::F64(v));
                        },
                        None => return 0,
                    }
                }
                _ => return 0,
            }
        }

        for vt in ft.results() {
            match vt {
                ValType::I32 => results.push(Val::I32(0)),
                ValType::I64 => results.push(Val::I64(0)),
                ValType::F32 => results.push(Val::F32(0)),
                ValType::F64 => results.push(Val::F64(0)),
                _ => return 0,
            }
        }
    }

    {
        let store = state.to_typed_userdata::<Store<*mut RawLuaState>>(2).unwrap();
        if let Err(_) = func.call(store, &params, &mut results) {
            return 0;
        }
    }

    for val in results.iter() {
        match val {
            Val::I32(v) => state.pushinteger(*v as LuaInteger),
            Val::I64(v) => state.pushinteger(*v as LuaInteger),
            Val::F32(v) => state.pushnumber(f32::from_ne_bytes((*v).to_ne_bytes()) as LuaNumber),
            Val::F64(v) => state.pushnumber(f64::from_ne_bytes((*v).to_ne_bytes()) as LuaNumber),
            _ => return 0,
        }
    }

    return results.len() as i32;
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
    meth_destroy_instance,
    meth_get_export,
    meth_get_exports,
    meth_create_store,
    meth_destroy_store,
    meth_invoke
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

    state.pushstring("get_export");
    state.pushcfunction(Some(cfunctions::meth_get_export));
    state.rawset(-3);

    state.pushstring("get_exports");
    state.pushcfunction(Some(cfunctions::meth_get_exports));
    state.rawset(-3);

    state.pushstring("create_store");
    state.pushcfunction(Some(cfunctions::meth_create_store));
    state.rawset(-3);

    state.pushstring("destroy_store");
    state.pushcfunction(Some(cfunctions::meth_destroy_store));
    state.rawset(-3);

    state.pushstring("invoke");
    state.pushcfunction(Some(cfunctions::meth_invoke));
    state.rawset(-3);

    return 1;
}