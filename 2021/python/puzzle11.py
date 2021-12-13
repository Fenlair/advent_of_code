#!/usr/bin/env python3

example_input = ["5483143223", "2745854711", "5264556173", "6141336146", "6357385478", "4167524645", "2176841721", "6882881134", "4846848554", "5283751526"]
real_input = open("input.txt").read().splitlines()
grid = {(m, n): int(c) for m, line in enumerate(real_input) for n, c in enumerate(line)}

def inc_neighbours(grid, m, n):
    neighbour_incs = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
    for dm, dn in neighbour_incs:
        if (m+dm, n+dn) in grid:
            grid[(m+dm, n+dn)] += 1

def flash(grid, flashed, flashed_now=set()):
    flashed_now.clear()
    for ind, v in grid.items():
        if v > 9 and ind not in flashed:
            flashed_now.add(ind)
            inc_neighbours(grid, *ind)
    if not flashed_now:
        return grid, flashed
    return flash(grid, flashed | flashed_now)

def step(grid):
    for ind in grid:                       # 1) increment grid
        grid[ind] += 1
    grid, flashed = flash(grid, set())     # 2) flash the octopusses
    for ind in flashed:                    # 3) reset the octopusses that flashed
        grid[ind] = 0
    return len(flashed), grid

total = 0
for i in range(1000):
    flashes, grid = step(grid)
    total += flashes
    if i == 99:
        print(f'Puzzle1: {total}')
    if flashes == len(grid):
        print(f'Puzzle2: {i + 1}')
        break
