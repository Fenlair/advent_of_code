#!/usr/bin/env python3

from math import prod
from typing import Dict, Iterable, List, Tuple
Point = Tuple[int, int]

def get_data():
    with open("input.txt") as f:
        data = f.readlines()
    return parse(data)


def get_example_data():
    data = ["2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"]
    return parse(data)


def parse(data: List[str]) -> Dict[Point, int]:
    return {(m, n): int(h) for m, line in enumerate(data)
                           for n, h in enumerate(line.strip())}


grid = get_data()


def neighbours(m: int, n: int) -> Iterable[Point]:
    return filter(lambda p: p in grid, [(m+1, n), (m-1, n), (m, n+1), (m, n-1)])


def is_low(p: Point) -> bool:
    return all(grid[p] < grid[n] for n in neighbours(*p))


def lowpoints() -> Iterable[Point]:
    return filter(is_low, grid)


def sum_of_risks(lowpoints: Iterable[Point]) -> int:
    return sum(grid[p] + 1 for p in lowpoints)


print(f'part1: {sum_of_risks(lowpoints())}')


visited = set()
def sizeof_basin(p: Point):
    if p in visited:
        return 0
    else:
        visited.add(p)

    if grid[p] == 9: return 0
    return 1 + sum(map(sizeof_basin, neighbours(*p)))


def all_basins(lowpoints):
    return [sizeof_basin(p) for p in lowpoints]

print(f'part2: {prod(sorted(all_basins(lowpoints()))[-3:])}')
