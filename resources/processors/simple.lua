local Vector = Yahmc:require("components/vector")
local Velocity = Yahmc:require("components/vector")

local Processor = { 
  data = {Yahmc:entities, Yahmc:fetchMut(Vector), Yahmc:fetch(Velocity)}
}

function Processor:process(elapsedTime, data)
  local entities, vecs, vels = Yahmc:Unpack(data)
  for vector, velocity in Yahmc:Join(vecs, vels) do
    vector.x += velocity.x * elapsedTime
    vector.y += velocity.y * elapsedTime
  end
end

return Processor