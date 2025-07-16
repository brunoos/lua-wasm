#include <stdlib.h>

#include <lua.h>
#include <lauxlib.h>

#include <wasmtime.h>

/**
 * Create a new engine.
 */
static int meth_new_engine(lua_State *L)
{
   wasm_engine_t *engine = wasm_engine_new();
   if (engine == NULL) {
      lua_pushnil(L);
      lua_pushstring(L, "failed to create an engine");
      return 2;
   }
   lua_pushlightuserdata(L, engine);
   return 1;
}

/**
 * Destroy an engine.
 */
static int meth_del_engine(lua_State *L)
{
   wasm_engine_t *engine = (wasm_engine_t*)lua_touserdata(L, 1);
   wasm_engine_delete(engine);
   return 0;
}

/**
 * Create a new module from the given content.
 */
static int meth_new_module(lua_State *L)
{
   wasm_engine_t *engine = (wasm_engine_t*)lua_touserdata(L, 1);
   const uint8_t *content = (const uint8_t*)luaL_checkstring(L, 2);
   size_t len = lua_rawlen(L, 2);
   wasmtime_error_t *error;
   wasmtime_module_t *module;
   error = wasmtime_module_new(engine, content, len, &module);
   if (error != NULL) {
      wasm_byte_vec_t message;
      wasmtime_error_message(error, &message);
      lua_pushnil(L);
      lua_pushlstring(L, (const char*)message.data, message.size);
      wasm_byte_vec_delete(&message);
      wasmtime_error_delete(error);
      return 2;
   }
   lua_pushlightuserdata(L, module);
   return 1;
}

/**
 * Destroy a module.
 */
static int meth_del_module(lua_State *L)
{
   wasmtime_module_t *module = (wasmtime_module_t*)lua_touserdata(L, 1);
   wasmtime_module_delete(module);
   return 0;
}

/**
 * Destroy a store.
 */
static int meth_del_store(lua_State *L)
{
   wasmtime_store_t *store = (wasmtime_store_t*)lua_touserdata(L, 1);
   wasmtime_store_delete(store);
   return 0;
}

/**
 * Create a new linker from an engine.
 */
static int meth_new_linker(lua_State *L)
{
   wasm_engine_t *engine = (wasm_engine_t*)lua_touserdata(L, 1);
   wasmtime_linker_t *linker = wasmtime_linker_new(engine);
   if (linker == NULL) {
      lua_pushnil(L);
      lua_pushstring(L, "failed to create a linker");
      return 2;
   }
   lua_pushlightuserdata(L, linker);
   return 1;
}

/**
 * Destroy a linker.
 */
static int meth_del_linker(lua_State *L)
{
   wasmtime_linker_t *linker = (wasmtime_linker_t*)lua_touserdata(L, 1);
   wasmtime_linker_delete(linker);
   return 0;
}

/**
 * Instantiate a module using the linker and engine.
 * This function creates a new store for the instance.
 */
static int meth_instantiate(lua_State *L)
{
   wasm_trap_t *trap = NULL;
   wasmtime_error_t *error = NULL;

   wasmtime_linker_t *linker = (wasmtime_linker_t*)lua_touserdata(L, 1);
   wasm_engine_t *engine     = (wasm_engine_t*)lua_touserdata(L, 2);
   wasmtime_module_t *module = (wasmtime_module_t*)lua_touserdata(L, 3);

   wasmtime_store_t *store = wasmtime_store_new(engine, L, NULL);
   if (store == NULL) {
      lua_pushnil(L);
      lua_pushnil(L);
      lua_pushstring(L, "failed to create a store");
      return 3;
   }

   wasmtime_instance_t *instance = (wasmtime_instance_t*)lua_newuserdata(L, sizeof(wasmtime_instance_t));
   if (instance == NULL) {
      wasmtime_store_delete(store);
      lua_pushnil(L);
      lua_pushnil(L);
      lua_pushstring(L, "failed to create an instance");
      return 3;
   }

   wasmtime_context_t *context = wasmtime_store_context(store);
   error = wasmtime_linker_instantiate(linker, context, module, instance, &trap);

   if (error != NULL) {
      wasm_byte_vec_t message;
      wasmtime_error_message(error, &message);

      lua_pushnil(L);
      lua_pushnil(L);
      lua_pushlstring(L, (const char*)message.data, message.size);

      wasmtime_store_delete(store);
      wasm_byte_vec_delete(&message);
      wasmtime_error_delete(error);

      return 3;
   }

   if (trap != NULL) {
      wasm_byte_vec_t message;
      wasm_trap_message(trap, &message);

      lua_pushnil(L);
      lua_pushnil(L);
      lua_pushlstring(L, (const char*)message.data, message.size);

      wasmtime_store_delete(store);
      wasm_byte_vec_delete(&message);
      wasm_trap_delete(trap);

      return 3;
   }

   lua_pushlightuserdata(L, store);

   return 2;
}

/**
 * Add valtypes to a table on top of stack.
 * It return false if the type is unknown.
 */
static bool set_valtype(lua_State *L, const wasm_valtype_vec_t *vet)
{
   for (size_t i = 0; i < vet->size; i++) {
      wasm_valkind_t k = wasm_valtype_kind(vet->data[i]);
      lua_pushinteger(L, i+1);
      switch (k) {
         case WASM_I32:
            lua_pushstring(L, "i32");
            lua_rawset(L, -3);
            break;
         case WASM_I64:
            lua_pushstring(L, "i64");
            lua_rawset(L, -3);
            break;
         case WASM_F32:
            lua_pushstring(L, "i32");
            lua_rawset(L, -3);
            break;
         case WASM_F64:
            lua_pushstring(L, "f64");
            lua_rawset(L, -3);
            break;
         default:
            return 0;
      }
   }
   return 1;
}

/**
 *  Get export.
 */
static int meth_get_export(lua_State *L)
{ 
   wasmtime_instance_t *instance = (wasmtime_instance_t*)lua_touserdata(L, 1);
   wasmtime_store_t *store = (wasmtime_store_t*)lua_touserdata(L, 2);
   const char *name = luaL_checkstring(L, 3);
   size_t len = lua_rawlen(L, 3);

   wasmtime_extern_t item;
   wasmtime_context_t *context = wasmtime_store_context(store);
   bool ok = wasmtime_instance_export_get(context, instance, name, len, &item);
   if (!ok) {
      lua_pushnil(L);
      lua_pushstring(L, "failed to get the exported item");
      return 2;
   }

   lua_newtable(L);

   lua_pushstring(L, "name");
   lua_pushvalue(L, 3);
   lua_rawset(L, -3);

   switch (item.kind) {
      case WASMTIME_EXTERN_FUNC:
         lua_pushstring(L, "type");
         lua_pushstring(L, "function");
         lua_rawset(L, -3);

         wasm_functype_t *ftype = wasmtime_func_type(context, (wasmtime_func_t*)&item.of);

         lua_pushstring(L, "params");
         lua_newtable(L);
         if (!set_valtype(L, wasm_functype_params(ftype))) {
            wasm_functype_delete(ftype);
            lua_pushnil(L);
            lua_pushstring(L, "unknown item type");
            return 2;
         }
         lua_rawset(L, -3);

         lua_pushstring(L, "results");
         lua_newtable(L);

         if (!set_valtype(L, wasm_functype_results(ftype))) {
            wasm_functype_delete(ftype);
            lua_pushnil(L);
            lua_pushstring(L, "unknown item type");
            return 2;
         }
         lua_rawset(L, -3);

         wasm_functype_delete(ftype);
         break;
      case WASMTIME_EXTERN_GLOBAL:
         lua_pushstring(L, "type");
         lua_pushstring(L, "global");
         lua_rawset(L, -3);
         break;
      case WASMTIME_EXTERN_TABLE:
         lua_pushstring(L, "type");
         lua_pushstring(L, "table");
         lua_rawset(L, -3);
         break;
      case WASMTIME_EXTERN_MEMORY:
         lua_pushstring(L, "type");
         lua_pushstring(L, "memory");
         lua_rawset(L, -3);

         wasm_memorytype_t *mtype = wasmtime_memory_type(context, (wasmtime_memory_t*)&item.of);
         
         uint64_t val = wasmtime_memorytype_minimum(mtype);
         lua_pushstring(L, "min");
         lua_pushinteger(L, val);
         lua_rawset(L, -3);

         if (wasmtime_memorytype_maximum(mtype, &val)) {
            lua_pushstring(L, "max");
            lua_pushinteger(L, val);
            lua_rawset(L, -3);
         }

         wasm_memorytype_delete(mtype);
         break;
      case WASMTIME_EXTERN_SHAREDMEMORY:
         lua_pushstring(L, "type");
         lua_pushstring(L, "sharedmemory");
         lua_rawset(L, -3);
         break;
      default:
         wasmtime_extern_delete(&item);
         lua_pushnil(L);
         lua_pushstring(L, "unknown item type");
         return 2;
   }

   wasmtime_extern_delete(&item);

   return 1;
}

static int meth_invoke(lua_State *L)
{
   wasm_trap_t *trap = NULL;
   wasmtime_error_t *error;

   wasmtime_instance_t *instance = (wasmtime_instance_t*)lua_touserdata(L, 1);
   wasmtime_store_t *store = (wasmtime_store_t*)lua_touserdata(L, 2);
   wasmtime_context_t *context = wasmtime_store_context(store);

   const char *name = luaL_checkstring(L, 3);
   size_t len = lua_rawlen(L, 3);

   size_t nparams = (size_t)lua_tointeger(L, 4);
   size_t nresults = (size_t)lua_tointeger(L, 5);

   wasmtime_extern_t item;
   bool ok = wasmtime_instance_export_get(context, instance, name, len, &item);
   if (!ok || item.kind != WASMTIME_EXTERN_FUNC) {
      lua_pushstring(L, "failed to get the exported function");
      lua_error(L);
      return 0;
   }

   wasmtime_val_t *params = NULL;
   wasmtime_val_t *results = NULL;

   int idx = 6;

   if (nparams > 0) {
      params  = (wasmtime_val_t*)malloc(nparams  * sizeof(wasmtime_val_t));
      for (size_t i = 0; i < nparams; i++) {
         params[i].kind = lua_tointeger(L, idx++);
         switch (params[i].kind) {
         case WASMTIME_I32:
            params[i].of.i32 = (int32_t)lua_tointeger(L, idx++);
            break;
         case WASMTIME_I64:
            params[i].of.i64 = (int64_t)lua_tointeger(L, idx++);
            break;
         case WASMTIME_F32:
            params[i].of.f32 = (float)lua_tonumber(L, idx++);
            break;
         case WASMTIME_F64:
            params[i].of.f64 = (double)lua_tonumber(L, idx++);
            break;
         }
      }
   }         

   if (nresults > 0) {
      results = (wasmtime_val_t*)malloc(nresults * sizeof(wasmtime_val_t));
      for (size_t i = 0; i < nresults; i++) {
         results[i].kind = lua_tointeger(L, idx++);
      }
   }

   error = wasmtime_func_call(context, &item.of.func, params, nparams, results, nresults, &trap);
   if (error != NULL) {
      wasm_byte_vec_t message;
      wasmtime_error_message(error, &message);

      lua_pushlstring(L, (const char*)message.data, message.size);

      wasmtime_store_delete(store);
      wasm_byte_vec_delete(&message);
      wasmtime_error_delete(error);

      if (params)  free(params);
      if (results) free(results);

      lua_error(L);
      return 0;
   }

   if (trap != NULL) {
      wasm_byte_vec_t message;
      wasm_trap_message(trap, &message);

      lua_pushlstring(L, (const char*)message.data, message.size);

      wasmtime_store_delete(store);
      wasm_byte_vec_delete(&message);
      wasm_trap_delete(trap);

      if (params)  free(params);
      if (results) free(results);

      lua_error(L);
      return 0;
   }

   for (size_t i = 0; i < nresults; i++) {
      switch (results[i].kind) {
      case WASMTIME_I32:
         lua_pushinteger(L, (lua_Integer)results[i].of.i32);
         break;
      case WASMTIME_I64:
         lua_pushinteger(L, (lua_Integer)results[i].of.i64);
         break;
      case WASMTIME_F32:
         lua_pushnumber(L, (lua_Number)results[i].of.f32);
         break;
      case WASMTIME_F64:
         lua_pushnumber(L, (lua_Number)results[i].of.f64);
         break;
      }
   }

   if (params)  free(params);
   if (results) free(results);

   return (int)nresults;
}

//------------------------------------------------------------------------------

static luaL_Reg wasm_methods[] = {
   {"new_engine", meth_new_engine},
   {"del_engine", meth_del_engine},
   {"new_module", meth_new_module},
   {"del_module", meth_del_module},
   {"del_store", meth_del_store},
   {"new_linker", meth_new_linker},
   {"del_linker", meth_del_linker},
   {"instantiate", meth_instantiate},
   {"get_export", meth_get_export},
   {"invoke", meth_invoke},
   {NULL, NULL}
};

extern int luaopen_wasm_core(lua_State *L) {
   luaL_newlib(L, wasm_methods);

   lua_pushstring(L, "i32");
   lua_pushinteger(L, WASMTIME_I32);
   lua_rawset(L, -3);

   lua_pushstring(L, "i64");
   lua_pushinteger(L, WASMTIME_I64);
   lua_rawset(L, -3);

   lua_pushstring(L, "f32");
   lua_pushinteger(L, WASMTIME_F32);
   lua_rawset(L, -3);

   lua_pushstring(L, "f64");
   lua_pushinteger(L, WASMTIME_F64);
   lua_rawset(L, -3);

   return 1;
}