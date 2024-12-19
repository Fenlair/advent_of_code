local util = require("util")
local inspect = require("inspect")

local example = [[
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
]]
local file = util.get_input({ is_example = false, year = 2024, day = 06 }, example)

-- Processing
Field = { empty = 0, blocked = 1, updown = 2, leftright = 3, both = 4, fake = 5 }

local foo = function(char)
  if char == "." then return Field.empty end
  if char == "#" then return Field.blocked end
  if char == "^" then return Field.updown end
  assert(false, "Received illegal char: " .. char)
end

local ingest = function(input)
  local data = {}
  for line in input:lines() do
    local row = {}
    for char in string.gmatch(line, "(.)") do
      local a = foo(char)
      table.insert(row, a)
    end
    table.insert(data, row)
  end
  return data
end

-- Part 1 Functions
local find_start = function(data)
  for m, line in ipairs(data) do
    for n, type in ipairs(line) do
      if type == Field.updown then return m, n end
    end
  end
  assert(false)
end


local move_horizontal
move_horizontal = function(m, n, direction, data, action)
  local new_n
  if direction == Dir.left then
    new_n = n - 1
  elseif direction == Dir.right then
    new_n = n + 1
  else
    assert(false)
  end

  if data[m][new_n] == Field.blocked then
    return m, n, data
  elseif new_n == 1 or new_n == #data[1] then
    action(data, m, new_n, direction)
    return nil, nil, data
  else
    action(data, m, new_n, direction)
    return move_horizontal(m, new_n, direction, data, action)
  end
end


local move_vertical
move_vertical = function(m, n, direction, data, action)
  local new_m
  if direction == Dir.up then
    new_m = m - 1
  elseif direction == Dir.down then
    new_m = m + 1
  else
    assert(false)
  end

  local next_field = data[new_m][n]
  if next_field == Field.blocked then
    return m, n, data
  elseif new_m == 1 or new_m == #data then
    action(data, new_m, n, direction)
    return nil, nil, data
  else
    action(data, new_m, n, direction)
    return move_vertical(new_m, n, direction, data, action)
  end
end

local update_field = function(data, m, n, direction)
  local next_field = data[m][n]
  if direction == Dir.left or direction == Dir.right then
    if next_field == Field.updown or next_field == Field.both then
      data[m][n] = Field.both
    else
      data[m][n] = Field.leftright
    end
  else
    if next_field == Field.leftright or next_field == Field.both then
      data[m][n] = Field.both
    else
      data[m][n] = Field.updown
    end
  end
end

Dir = { left = 0, up = 1, right = 2, down = 3 }

local update_direction = function(direction)
  if direction == Dir.left then
    direction = Dir.up
  elseif direction == Dir.up then
    direction = Dir.right
  elseif direction == Dir.right then
    direction = Dir.down
  elseif direction == Dir.down then
    direction = Dir.left
  end
  return direction
end


local walk = function(m, n, data, action)
  local steps_taken = {}
  local direction = Dir.left
  repeat
    direction = update_direction(direction)

    if direction == Dir.left or direction == Dir.right then
      m, n, data = move_horizontal(m, n, direction, data, action)
    else
      m, n, data = move_vertical(m, n, direction, data, action)
    end

    if steps_taken[{m, n, direction}] then
      Obstacles = Obstacles + 1
      break
    end
    steps_taken[{m, n, direction}] = true
  until not m
  return data
end


-- Part 1
local data = ingest(file)
local m_start, n_start = find_start(data)
data = walk(m_start, n_start, data, update_field)

local sum = 0
for _, line in ipairs(data) do
  for _, field in ipairs(line) do
    if field == Field.leftright or field == Field.updown or field == Field.both then sum = sum + 1 end
  end
end
print("Part 1:", sum)

-- Part 2

walk(m_start, n_start, data, find_blocks)
for _, line in ipairs(data) do
  print(inspect(line))
end

print("Part 2:", Obstacles)
