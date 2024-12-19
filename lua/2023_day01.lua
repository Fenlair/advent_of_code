local util = require("util")

local is_example, day = true, 1


local example = [[
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
]]
local file = util.get_input(is_example, day, example)

local function extract_calibration_value(line)
  local x = string.sub(string.match(line, "%d+"), 1, 1)
  local y = string.sub(string.match(string.reverse(line), "%d+"), 1, 1)
  local pair = x .. y
  return tonumber(pair)
end

-- Part 1
local sum = 0
for line in file:lines() do
  sum = sum + extract_calibration_value(line)
end

print("Part 1: " .. sum)

-- Part 2
example = [[
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
]]
file = util.get_input(is_example, day, example)


local map = {
  one = 1,
  two = 2,
  three = 3,
  four = 4,
  five = 5,
  six = 6,
  seven = 7,
  eight = 8,
  nine = 9,
}

local function find_first_match(line)
  local lowest_index = math.huge
  local current_pattern = nil

  for pattern, _ in pairs(map) do
	local index = string.find(line, pattern)
    if index and index < lowest_index then
      lowest_index = index
      current_pattern = pattern
    end
  end
  return current_pattern
end

local function convert_to_number(line)
  local match = find_first_match(line)
  if not match then
	return line
  end

  local new_line = line:gsub(match, map[match])
  return convert_to_number(new_line)
end

local sum = 0
for line in file:lines() do
  local new_line = convert_to_number(line)
  sum = sum + extract_calibration_value(new_line)
end

print("Part 2: " .. sum)
