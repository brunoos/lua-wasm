local core = require("wasm.core")

--------------------------------------------------------------------------------

-- Metatables
local enginemeta = {}
local modulemeta = {}
local linkermeta = {}
local instancemeta = {}

enginemeta.__index = {}
linkermeta.__index = {}
modulemeta.__index = {}
instancemeta.__index = {}

-- Destroys a engine.
enginemeta.__gc = function(self)
  print("Destroying engine: ", self._refengine)
  core.destroy_engine(self._refengine)
end

-- Creates a new module.
enginemeta.__index.newmodule = function(self, content)
  local ref = core.create_module(self._refengine, content)
  if not ref then
    return nil, "failed to create module"
  end
  local module = setmetatable({
    _refmodule = ref,
    _engine = self
  }, modulemeta)
  return module
end

-- Creates a new linker.
enginemeta.__index.newlinker = function(self)
  local ref = core.create_linker(self._refengine)
  if not ref then
    return nil, "failed to create linker"
  end
  return setmetatable({
    _reflinker = ref,
    _engine = self
  }, linkermeta)
end

-- Destroys a module.
modulemeta.__gc = function(self)
  if self._refmodule then
    print("Destroying module: ", self._refmodule)
    core.destroy_module(self._refmodule)
    self._refmodule = nil
    self._engine = nil
  end
end

-- Creates a new instance from a module using a linker.
modulemeta.__index.newinstance = function(self, linker)
  local refinstance, refstore = core.create_instance(self._engine._refengine, linker._reflinker, self._refmodule)
  if not refinstance then
    return nil, "failed to create instance"
  end
  return setmetatable({
    _refinstance = refinstance,
    _refstore = refstore,
    _module = self,
  }, instancemeta)
end

-- Get the exports of a module.
modulemeta.__index.exports = function(self)
  local exports = core.get_exports(self._refmodule)
  if not exports then
    return nil, "failed to get exports"
  end
  return exports
end

-- Destroys a linker.
linkermeta.__gc = function(self)
  if self._reflinker then
    print("Destroying linker: ", self._reflinker)
    core.destroy_linker(self._reflinker)
    self._reflinker = nil
    self._engine = nil
  end
end

-- Destroys an instance.
instancemeta.__gc = function(self)
  if self._refinstance then
    print("Destroying instance: ", self._refinstance)
    print("Destroying store: ", self._refstore)
    core.destroy_instance(self._refinstance)
    core.destroy_store(self._refstore)
    self._refinstance = nil
    self._refstore = nil
    self._module = nil
  end
end

instancemeta.__index.exports = function(self)
  local exports = core.get_exports(self._module._refmodule)
  if not exports then
    return nil, "failed to get module exports"
  end
  return exports
end

instancemeta.__index.invoke = function(self, name, ...)
  return core.invoke(self._refinstance, self._refstore, name, ...)
end

--------------------------------------------------------------------------------

-- Creates a new engine.
local function newengine()
  local e = {}
  e._refengine = core.create_engine()
  return setmetatable(e, enginemeta)
end

--------------------------------------------------------------------------------

return {
  newengine = newengine
}