import std/strutils
import std/sequtils
import std/tables
import std/algorithm
import std/options

let
  real = readFile("../inputs/day08.txt")
  data = real.splitLines()
  p = data.map(proc (x: string): seq[string] = x.split(" | "))

proc check_mapping(input: string, mapping: Table[char, char]): bool =
  let valid = @["abcefg", "cf", "acdeg", "acdfg", "bcdf",
                "abdfg", "abdefg", "abcdefg", "abcdfg"]
  for dig in input.split(' '):
    var mapped_dig = ""
    for c in dig:
      mapped_dig.add(mapping[c])
    mapped_dig.sort()
    if not (mapped_dig in valid):
      return false
  return true

echo check_mapping("abcefg fc", {'a': 'a', 'b': 'b', 'c': 'c', 'd': 'd', 'e': 'e', 'f': 'f', 'g': 'g'}.toTable())

proc disp_mapping(input: string): Option[Table[char, char]] =
  var
    f = @['a', 'b', 'c', 'd', 'e', 'f', 'g']
    p = f
    done = false
    is_valid = false

  while not done:
    var mapping = initTable[char, char]()
    for (k, v) in zip(f, p):
      mapping[k] = v
    echo mapping
    is_valid = check_mapping(input, mapping)
    if is_valid:
      return some(mapping)
    done = not p.nextPermutation()

echo p[0][0]
echo disp_mapping(p[0][0])
