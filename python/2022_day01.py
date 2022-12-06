#!/usr/bin/env python3
from aocd import get_data, submit
puzzle = get_data(day=1, year=2022)

elves = [x.split("\n") for x in puzzle.split("\n\n")]
elves = [list(map(lambda x: int(x), elf)) for elf in elves]
elves = [sum(elf) for elf in elves]
max_cal = max(elves)
# submit(max_cal, part="a", day=1, year=2022)

elves = sorted(elves)
cal = sum(elves[-3:])
submit(cal, part="b", day=1, year=2022)
