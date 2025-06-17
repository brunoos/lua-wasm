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

-- Gabage collector for engine.
enginemeta.__gc = function(e)
  print("Destroying engine: ", e._refengine)
  core.destroy_engine(e._refengine)
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

modulemeta.__gc = function(m)
  if m._refmodule then
    print("Destroying module: ", m._refmodule)
    core.destroy_module(m._refmodule)
    m._refmodule = nil
    m._engine = nil
  end
end

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

linkermeta.__gc = function(l)
  if l._reflinker then
    print("Destroying linker: ", l._reflinker)
    core.destroy_linker(l._reflinker)
    l._reflinker = nil
    l._engine = nil
  end
end

modulemeta.__index.newinstance = function(self, linker)
  local ref = core.create_instance(self._engine._refengine, linker._reflinker, self._refmodule)
  if not ref then
    return nil, "failed to create instance"
  end
  return setmetatable({
    _refinstance = ref,
    _module = self,
  }, instancemeta)
end

instancemeta.__gc = function(i)
  if i._refinstance then
    print("Destroying instance: ", i._refinstance)
    core.destroy_instance(i._refinstance)
    i._refinstance = nil
    i._module = nil
  end
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