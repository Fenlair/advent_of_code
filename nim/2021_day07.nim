import std/strutils
import std/sequtils

let
  # example = "16,1,2,0,4,2,7,1,2,14"
  real = readFile("../inputs/2021_day07.txt").strip()
  positions = real.split(',').map(parseInt)

proc map_abs_diff(positions: seq[int], pos: int): seq[int] =
  positions.map(proc (p: int): int = abs(pos - p))

type
  CrabFn = proc (x: int): int

proc simple_crab_cost(pos: int): int =
  positions.map_abs_diff(pos).foldl(a + b)

proc revised_crab_cost(pos: int): int =
  positions.map_abs_diff(pos)
           .map(proc (x: int): int = int(x * (x+1) / 2))
           .foldl(a + b)

proc best_position(crab_fn: CrabFn): int =
  toSeq(0..positions.len).map(crab_fn).min()

echo simple_crab_cost(2)
echo best_position(simple_crab_cost)
echo best_position(revised_crab_cost)
