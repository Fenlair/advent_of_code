#!/usr/bin/env python3

from collections import Counter
from functools import cache

example = ["NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N", "CN -> C"]
real = open("../inputs/2021_day14.txt").read().splitlines()

def parse(data):
    template, _, *tmp = data
    return template, dict(m.split(" -> ") for m in tmp)

@cache
def step(l, r, depth=40):
    if depth == 0:
        return Counter("")
    m = lookup[l + r]
    return Counter(m) + step(l, m, depth-1) + step(m, r, depth-1)

def metric(c: Counter):
    return max(c.values()) - min(c.values())

template, lookup = parse(real)
c = sum(map(step, template, template[1:]), Counter(template))
print(metric(c))
