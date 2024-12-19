local util = require("util")

local example = [[
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
]]
local file = util.get_input({ is_example = false, year = 2024, day = 7 }, example)

-- Processing
---@param f file*
local ingest = function(f)
	local data = {}
	for line in f:lines() do
		local nums = {}
		for token in string.gmatch(line, "(%d+)") do
			table.insert(nums, tonumber(token))
		end
		table.insert(data, nums)
	end
	return data
end

-- Part 1 Functions
local try_permutation = function(nums, ops)
	assert(#nums == #ops + 2)
	assert(#nums >= 2)
	local result = nums[1]
	local acc = nums[2]
	for num, op in util.zip({ table.unpack(nums, 3) }, ops) do
		if op == "+" then
			acc = acc + num
		elseif op == "*" then
			acc = acc * num
		elseif op == "|" then
			acc = tonumber(tostring(acc) .. tostring(num))
		end
	end
	return result == acc
end

local try_permutations = function(nums, set)
	local len = #nums - 2
	for ops in util.permutation(set, len) do
		if try_permutation(nums, ops) then
			return nums[1]
		end
	end
end

local try_equations = function(data, set)
	local sum = 0
	for _, nums in ipairs(data) do
		local a = try_permutations(nums, set)
		if a then
			sum = sum + a
		end
	end
	return sum
end

-- Part 1
local data = ingest(file)
local p1 = try_equations(data, { "+", "*" })
print("Part 1:", p1)
local p2 = try_equations(data, { "+", "*", "|" })
print("Part 2:", p2)
