## Lua & WebAssembly

`lua-wasm` allows you to load WebAssembly modules.

This is a work in progress...

## Dependencies

- Wasmtime C API

## Build

`make LUA_INC=<Path to Lua include dir> WASMTIME_INC=<Path to Wasmtime include dir> WASMTIME_LIB=<Path to Wasmtime library dir>`

## Install

- Copy `core.so` to `LUA_CPATH/wasm`
- Copy `wasm.lua` to `LUA_PATH`

## Notice

This module uses garbage collector metamethod on tables, that is not present in Lua 5.1.