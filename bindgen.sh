#!/bin/sh

if [ -z "$LUA_INC" ]; then
  LUA_INC="/usr/include/lua5.4"
fi

bindgen \
 --raw-line "#![allow(non_camel_case_types)]" \
 --raw-line "#![allow(non_snake_case)]" \
 --raw-line "#![allow(non_upper_case_globals)]" \
 --no-layout-tests \
 --no-doc-comments \
 --allowlist-type "lua_State" \
 --allowlist-type "lua_CFunction" \
 --allowlist-function "lua_createtable" \
 --allowlist-function "lua_pushboolean" \
 --allowlist-function "lua_pushcclosure" \
 --allowlist-function "lua_pushlightuserdata" \
 --allowlist-function "lua_pushlstring" \
 --allowlist-function "lua_pushnumber" \
 --allowlist-function "lua_rawset" \
 --allowlist-function "lua_tointegerx" \
 --allowlist-function "lua_tolstring" \
 --allowlist-function "lua_tonumberx" \
 --allowlist-function "lua_touserdata" \
 -o src/lua/lua54.rs \
 src/binding.h -- -I$LUA_INC