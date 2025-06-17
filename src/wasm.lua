local core = require("wasm.core")

--------------------------------------------------------------------------------
local enginemeta = {}

function enginemeta.__gc(e)
    core.destroy_engine(e._engine)
end

--------------------------------------------------------------------------------
local function newengine()
    local e = { _engine = core.create_engine() }
    return setmetatable(e, enginemeta)
end

--------------------------------------------------------------------------------
return {
    newengine = newengine
}