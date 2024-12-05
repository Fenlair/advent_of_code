local util = require("util")
local inspect = require("inspect")

local example = [[
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

47,75,61,53,29
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
]]
local file = util.get_input({is_example=false, year=2024, day=05}, example)

-- Processing
local ingest = function(input)
  local rules = {}
  for line in input:lines() do
    if line == "" then break end
    local str1, str2 = string.gmatch(line, "(%d+)|(%d+)")()
    local num1, num2 = assert(math.tointeger(str1)), assert(math.tointeger(str2))
    if not rules[num1] then
      rules[num1] = {num2}
    else
      table.insert(rules[num1], num2)
    end
  end
  -- print(inspect(rules))
  local pages = {}
  for line in input:lines() do
    local page = {}
    for token in string.gmatch(line, "(%d+)") do
      table.insert(page, assert(math.tointeger(token)))
    end
    table.insert(pages, page)
  end
  -- print(inspect(pages))
  return pages, rules
end

-- Part 1 Functions
local check_entry = function(index, page, rules)
  local entry = assert(page[index])
  if not rules[entry] then return true end
  for _, should_be_later in ipairs(rules[entry]) do
    for i = 1, (index - 1) do
      if page[i] == should_be_later then return false end
    end
  end
  return true
end

local check_page = function(page, rules, p)
  for index, _ in ipairs(page) do
    if not check_entry(index, page, rules) then
      if p then print(index) end
      return false
    end
  end
  return true
end

local middle_element = function(page)
  assert(#page % 2 == 1)
  return page[#page // 2 + 1]
end

-- Part 2 Functions
local swap = function(tbl, index1, index2)
  tbl[index1], tbl[index2] = tbl[index2], tbl[index1]
  return tbl
end

local bubble_sort = function(page, rules)
  repeat
    for index, _ in ipairs(page) do
      if not check_entry(index, page, rules) then
        swap(page, index, index-1)
      end
    end
  until check_page(page, rules)
end

-- Part 1
local pages, rules = ingest(file)

local sum = 0
for _, page in ipairs(pages) do
  if check_page(page, rules) then sum = sum + middle_element(page) end
end
print("Part 1:", sum)

-- Part 2
sum = 0
for _, page in ipairs(pages) do
  if not check_page(page, rules) then
    bubble_sort(page, rules)
    sum = sum + middle_element(page)
  end
end
print("Part 2:", sum)
