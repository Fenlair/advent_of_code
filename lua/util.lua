local function get_input(t, example)
  if t["is_example"] then
    local file = io.tmpfile()
    file:write(example)
    file:seek("set", 0)
    return file
  else
    local file = io.open("../inputs/"..tostring(t.year).."_day"..tostring(t.day)..".txt", "r")
    return file
  end
end

---Returns a sliced copy of an array
---@param tbl any[]
---@param first integer
---@param last integer
---@param step integer
local function slice(tbl, first, last, step)
  local sliced = {}

  for i = first or 1, last or #tbl, step or 1 do
    sliced[#sliced+1] = tbl[i]
  end

  return sliced
end

---Adds `__index` function to metatable
---@param t table
---@param d any
---@return nil
local function set_default(t, d)
  local mt = {__index = function () return d end}
  setmetatable(t, mt)
end

---Takes variable amount of tables or iterators and pairs the first items together, then the second, etc.
---@vararg table|fun(): any
---@return any ...
local function zip(...)
  local arrays, ans = {...}, {}
  local index = 0
  return
    function()
      index = index + 1
      for i,t in ipairs(arrays) do
        if type(t) == 'function' then ans[i] = t() else ans[i] = t[index] end
        if ans[i] == nil then return end
      end
      return table.unpack(ans)
    end
end

---Returns true if all elements of array are true
---@param t any[]
---@return boolean
local function all(t)
  for _, v in ipairs(t) do
    if not v then return false end
  end
  return true
end


local function sign(number)
  return (number > 0 and 1) or (number == 0 and 0) or -1
end

return { get_input=get_input, set_default=set_default, slice=slice, zip=zip, all=all }
