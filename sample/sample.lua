local wasm = require("wasm")

local f = io.open("module.wasm", "rb")
if not f then
  print("Failed to open module.wasm")
  return
end

local content = f:read("*a")
f:close()

local engine, err = wasm.engine()
if not engine then
  print(err)
  return
end

print("Engine", engine)

local module, err = engine:module(content)
if not module then
  print(err)
  return
end

print("Module", module)

local linker, err = engine:linker()
if not linker then
  print(err)
  return
end

print("Linker", linker)

local instance, err = linker:instantiate(module)
if not instance then
  print(err)
  return
end

print("Instance", instance)