local util = require("util")

local example = [[
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
]]
local file = util.get_input({is_example=false, year=2024, day=04}, example)

-- Processing
local horizontal = {}
for line in file:lines() do
  table.insert(horizontal, line)
end

-- Part 1
local create_vertical = function(t)
  local num_columns = #t[1]
  local vertical = {}
  for i = 1, num_columns do vertical[i] = "" end

  for _, line in ipairs(t) do
    for i = 1, num_columns do
      vertical[i] = vertical[i]..line:sub(i, i)
    end
  end
  return vertical
end

local make_array = function(len, default)
  local a = {}
  for i = 1, len do
    a[i] = default or 0
  end
  return a
end

local create_diagonals = function(t)
  local M <const> = #t
  local N <const> = #t[1]
  local num_diagonals <const> = (M + N - 1)

  local diagonal, antidiagonal = make_array(num_diagonals, ""), make_array(num_diagonals, "")

  for m = 1, M do
    for n = 1, N do
      local cur_diagonal = M + (n - m)
      local cur_antidiag = (m + n - 1)

      diagonal[cur_diagonal]     =     diagonal[cur_diagonal]..t[m]:sub(n, n)
      antidiagonal[cur_antidiag] = antidiagonal[cur_antidiag]..t[m]:sub(n, n)
    end
  end

  return diagonal, antidiagonal
end


local vertical = create_vertical(horizontal)
local diagonal, antidiagonal = create_diagonals(horizontal)
-- print(table.unpack(horizontal))
-- print(table.unpack(vertical))
-- print(table.unpack(diagonal))

local window_string = function(s, len)
  local i = 0
  return function()
    i = i + 1
    if i+len-1 <= #s then
      return i, s:sub(i, i+len-1)
    end
  end
end

local match = function(line, pattern)
  local sum = 0
  for _, s in window_string(line, #pattern) do
    if s == pattern or s == string.reverse(pattern) then
      sum = sum + 1
    end
  end
  return sum
end

local res = 0
for _, line in ipairs(horizontal) do
  res = res + match(line, "XMAS")
end
for _, line in ipairs(vertical) do
  res = res + match(line, "XMAS")
end
for _, line in ipairs(diagonal) do
  res = res + match(line, "XMAS")
end
for _, line in ipairs(antidiagonal) do
	res = res + match(line, "XMAS")
end
print("Part 1:", res)

-- Part 2
local xmatch = function(horis, diags, pattern)
  local M <const> = #horis
  local N <const> = #horis[1]

  local get = function(m, n)
    assert(m <= M, "get(m, n): m too large (got "..m.."), max: "..M)
    assert(n <= N, "get(m, n): n too large (got "..n.."), max: "..N)
    return horis[m]:sub(n, n)
  end

  local sum = 0
  for cur_diag, diag in ipairs(diags) do
    for i, s in window_string(diag, #pattern) do
      if s == pattern or s == string.reverse(pattern) then
        local m = math.max(M - (cur_diag-1), 1) + i
        local n = math.max(cur_diag - (M-1), 1) + i
        if m+1 <= M and m-1 >= 1 and n+1 <= N and n-1 >= 0 then
          local anti = get(m+1, n-1)..get(m, n)..get(m-1, n+1)
          if anti == pattern or anti == string.reverse(pattern) then
            sum = sum + 1
          end
        end
      end
    end
  end
  return sum
end

print("Part 2:", xmatch(horizontal, diagonal, "MAS"))
