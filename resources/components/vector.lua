local Vector = {x: 0.0, y: 0.0}

function Vector:new(object) {
  object = object or {}
  self.__index = self
  setmetatable(object, self)
  return object
}

return Engine:createComponent(Vector)