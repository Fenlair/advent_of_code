#!/usr/bin/env python3

real = open("../inputs/2021_day13.txt").read().splitlines()
example = ["6,10",
           "0,14",
           "9,10",
           "0,3",
           "10,4",
           "4,11",
           "6,0",
           "6,12",
           "4,1",
           "0,13",
           "10,12",
           "3,4",
           "3,0",
           "8,4",
           "1,10",
           "2,14",
           "8,10",
           "9,0",
           "",
           "fold along y=7",
           "fold along x=5"]

dots, folds = set(), []

def parse(data):
    it = iter(data)
    for line in it:
        if line == "": break
        dots.add(tuple(map(int, line.split(","))))
    for line in it:
        _, _, f = line.split()
        d, v = f.split("=")
        folds.append((0 if d == "x" else 1, int(v)))

def fold(idx, value):
    global dots
    to_be_folded = {dot for dot in dots if dot[idx] > value}
    new_dots = {(x, 2*value-y) if idx else (2*value-x, y) for x, y in to_be_folded}
    dots = (dots - to_be_folded) | new_dots

def print_dots():
    xmax, ymax = map(max, zip(*dots))
    for y in range(ymax+1):
        for x in range(xmax+1):
            print("#" if (x, y) in dots else ".", end=" ")
        print()

parse(real), fold(*folds[0])
print(f"Puzzle1: {len(dots)}")
[fold(*f) for f in folds[1:]]
print_dots()
