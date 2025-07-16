local core = require('wasm.core')

local engine_meta = {}
local linker_meta = {}
local module_meta = {}
local instance_meta = {}

engine_meta.__index = {}
linker_meta.__index = {}
instance_meta.__index = {}

-- Destroy the engine
engine_meta.__gc = function(self)
  if self._ref then
    core.del_engine(self._ref)
    self._ref = nil
  end
end

-- Create a new module from an engine
engine_meta.__index.module = function(self, content)
  local ref, err = core.new_module(self._ref, content)
  if not ref then
    return nil, err
  end
  return setmetatable({_ref = ref, _engine = self}, module_meta)
end

-- Create a new linker from an engine
engine_meta.__index.linker = function(self)
  local ref, err = core.new_linker(self._ref)
  if not ref then
    return nil, err
  end
  return setmetatable({_ref = ref, _engine = self}, linker_meta)
end

-- Destroy the module
module_meta.__gc = function(self)
  if self._ref then
    core.del_module(self._ref)
    self._ref = nil
    self._engine = nil
  end
end

-- Destroy the linker
linker_meta.__gc = function(self)
  if self._ref then
    core.del_linker(self._ref)
    self._ref = nil
    self._engine = nil
  end
end

-- Create a new instance from a module, using this linker
linker_meta.__index.instantiate = function(self, module)
  local ref, refstore, err = core.instantiate(self._ref, self._engine._ref, module._ref)
  if not ref then
    return nil, err
  end
  return setmetatable({_ref = ref, _refstore = refstore, _module = module}, instance_meta)
end

-- Destroy the instance
instance_meta.__gc = function(self)
  if self._ref then
    core.del_instance(self._ref)
    core.del_store(self._refstore)
    self._ref = nil
    self._refstore = nil
    self._module = nil
  end
end

-- Invoke a function in the instance
instance_meta.__index.invoke = function(self, name, ...)
  return core.invoke(self._ref, self._refstore, name, ...)
end

-- Get export item type
instance_meta.__index.getexport = function(self, name)
  return core.get_export(self._ref, self._refstore, name)
end

--------------------------------------------------------------------------------

local _M = {}

-- Create a new engine
function _M.engine()
  local ref, err = core.new_engine()
  if not ref then
    return nil, err
  end
  return setmetatable({_ref = ref}, engine_meta)
end

return _M