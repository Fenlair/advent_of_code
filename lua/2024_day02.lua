local util = require("util")


local example = [[
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
]]
local file = util.get_input({is_example=false, year=2024, day=2}, example)

-- Processing
local data = {}
for line in file:lines() do
  local numbers = {}
  for token in string.gmatch(line, "(%d+)") do
    table.insert(numbers, tonumber(token))
  end
  table.insert(data, numbers)
end

-- Part 1
local function num_offending_monotonic(series)
  local inc_off, dec_off = 0, 0
  for x, y in util.zip(series, util.slice(series, 2)) do
    if x <= y then dec_off = dec_off + 1 end
    if x >= y then inc_off = inc_off + 1 end
  end
  return math.min(inc_off, dec_off)
end

local function max_diff(series, max_value)
  local at_most_max_val = {}
  for x, y in util.zip(series, util.slice(series, 2)) do
    table.insert(at_most_max_val, math.abs(x - y) <= max_value)
  end
  return util.all(at_most_max_val)
end

local function is_safe(line, num_offending)
  num_offending = num_offending or 0
	if num_offending_monotonic(line) <= num_offending and max_diff(line, 3) then
    return true
  end
end

local sum = 0
for _, line in ipairs(data) do
	if is_safe(line) then
    sum = sum + 1
  end
end
print("Part 1:", sum)

-- Part 2
local function brute_force(line)
  -- iterate through all slots and try with that element removed
	for i = 1, #line do
    -- copy with one element removed
    local copy = {}
    for k, v in ipairs(line) do
      if i ~= k then copy[#copy+1] = v end
    end

    if is_safe(copy) then return true end
  end
  return false
end

sum = 0
for _, line in ipairs(data) do
  if brute_force(line) then sum = sum + 1 end
end
print("Part 2:", sum)
