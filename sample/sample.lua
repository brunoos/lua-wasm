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

print("---")

print("Getting function 'adder' info:")
local item, err = instance:getexport("adder")
print(item, err)
for k, v in pairs(item) do
  print(k, v)
  if k == "params" or k == "results" then
    for k, v in ipairs(v) do
      print(k, v)
    end
  end
end

print("---")

print("Getting 'memory' info:")
local item, err = instance:getexport("memory")
print(item, err)
for k, v in pairs(item) do
  print(k, v)
end

print("---")

print("Getting 'nothing' info:")
local item, err = instance:getexport("nothing")
print(item, err)

print("---")

print("Invoking 'adder':")
local res, err = instance:invoke("adder", 10, 20)
print(res, err)
