local util = require("util")

local example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
local file = util.get_input({is_example=false, year=2024, day=3}, example)

-- Processing
local data = {}
for line in file:lines() do
  table.insert(data, line)
end
local line = table.concat(data, "")

-- Part 1

local function multiply(code)
  local sum = 0
  for num1, num2 in string.gmatch(code, "mul%((%d+),(%d+)%)") do
    sum = sum + math.tointeger(num1) * math.tointeger(num2)
  end
  return sum
end

print("Part 1:", multiply(line))

-- Part 2
line = line .. "do()"
line = string.gsub(line, "don\'t%(%).-do%(%)", "")
print("Part 2:", multiply(line))
