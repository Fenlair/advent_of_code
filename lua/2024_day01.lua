local util = require("util")


local example = [[
3   4
4   3
2   5
1   3
3   9
3   3
]]
local file = util.get_input({is_example=false, year=2024, day=1}, example)

-- Processing Input
local list1, list2 = {}, {}
for line in file:lines() do
  local one, two = string.gmatch(line, "(%d+)%s+(%d+)")()
  table.insert(list1, one)
  table.insert(list2, two)
end

-- Part 1
table.sort(list1)
table.sort(list2)
local sum = 0
for a, b in util.zip(list1, list2) do
	sum = sum + math.abs(a - b)
end
print(sum)

-- Part 2
local count = {}
util.set_default(count, 0)

for _, val in pairs(list1) do
  for _, ele in pairs(list2) do
    if val == ele then
      count[ele] = count[ele] + 1
    end
  end
end

sum = 0
for k, v in pairs(count) do
	sum = sum + k * v
end
print(sum)
