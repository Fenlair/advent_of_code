#!/usr/bin/env python3

from collections import Counter, defaultdict


data = ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"]
data = ["dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa", "kj-HN", "kj-dc"]
data = ["fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he", "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW"]
data = open("../inputs/2021_day12.txt").read().splitlines()

out_edges = defaultdict(list)
for edge in data:
    a, b = edge.split("-")
    if b != "start": out_edges[a].append(b)
    if a != "start": out_edges[b].append(a)

def unique(path):
    return all(v <= 1 for v in Counter(filter(str.islower, path)).values())

def valid(puzzle1):
    return lambda target, path: target.isupper() or target not in path or (False if puzzle1 else unique(path))

def depth_first(node, path, valid):
    if node == "end":
        return 1
    targets = [target for target in out_edges[node] if valid(target, path)]
    return sum(depth_first(target, [*path, target], valid) for target in targets)

print(f'result1: {depth_first("start", ["start"], valid(puzzle1=True))}')
print(f'result2: {depth_first("start", ["start"], valid(puzzle1=False))}')
