local Vector = {x = 0, y = 0}
            
function Vector:magnitude()
    return math.sqrt(self.x * self.x + self.y * self.y)
end

return Vector