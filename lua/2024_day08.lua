local util = require("util")
local inspect = require("inspect")


local example = [[
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
]]
local file = util.get_input({ is_example = true, year = 2024, day = 8 }, example)

-- Processing
local ingest = function(f)
    local data = {}
    for line in f:lines() do
        table.insert(data, line)
    end
    return data
end


-- Part 1 Functions
local frequencies = function(arr)
    local freqs = {}
    for _, line in ipairs(arr) do
        for token in string.gmatch(line, '(.)') do
            if token ~= '.' then freqs[token] = true end
        end
    end
    return freqs
end






local data = ingest(file)
local freqs = frequencies(data)
print(inspect(data))
print(inspect(freqs))
