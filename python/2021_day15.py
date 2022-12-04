#!/usr/bin/env python3

from itertools import repeat, chain

PUZZLE1 = False
DIJKSTRA = True
EXAMPLE = False
example = ["1163751742",
           "1381373672",
           "2136511328",
           "3694931569",
           "7463417111",
           "1319128137",
           "1359912421",
           "3125421639",
           "1293138521",
           "2311944581"]
real = open("../inputs/2021_day15.txt").read().splitlines()
data = example if EXAMPLE else real
data_size = (len(data), len(data[0]))

def foo(k, x):
    return (int(x)-1 + k[0] // data_size[0] + k[1] // data_size[1]) % 9 + 1

def grid_factory(d, map_fn=None):
    if map_fn is None:
        map_fn = lambda k, x: x
    for m, row in enumerate(d):
        for n, v in enumerate(row):
            yield (m, n), map_fn((m,n), v)

if PUZZLE1:
    grid = {k: v for k, v in grid_factory(data, map_fn=lambda _, x: int(x))}
else:
    data = [row * 5 for row in chain(*repeat(data, 5))]
    grid = {k: v for k, v in grid_factory(data, map_fn=foo)}

graph = {(m, n): [((m+md, n+nd), grid[(m+md, n+nd)]) for md, nd in [(1, 0), (-1, 0), (0, 1), (0, -1)] if (m+md, n+nd) in grid] for m, n in grid}
target = max([m for m, _ in grid]), max([n for _, n in grid])

def dijkstra_shortes_path():
    Q = set(graph)
    prev = {k: None for k in Q}
    dist = {k: float("inf") for k in Q}
    dist[0, 0] = 0

    while Q:
        v = min({k: v for k, v in dist.items() if k in Q}, key=dist.get)
        Q.remove(v)
        print(v)

        if v == target: break

        for n, dvn in graph[v]:
            if n not in Q: continue
            alt = dist[v] + dvn
            if alt < dist[n]:
                dist[n] = alt
                prev[n] = v
    return dist, prev


def a_star():
    Q = set(graph)
    prev = {k: None for k in Q}
    heur = {k: target[0]-k[0] + target[1]+k[1] for k in Q}
    dist = {k: float("inf") for k in Q}
    dist[0, 0] = 0
    esti = {k: float("inf") for k in Q}
    esti[0, 0] = heur[0, 0]

    while Q:
        v = min({k: v for k, v in esti.items() if k in Q}, key=esti.get)
        if v == target: return dist, esti, prev

        Q.remove(v)

        for n, dvn in graph[v]:
            if n not in Q: continue
            alt = dist[v] + dvn
            if alt < dist[n]:
                prev[n] = v
                dist[n] = alt
                esti[n] = alt + heur[n]
                if n not in Q:
                    Q.add(n)
    raise ValueError("Goal unreachable!")


def reconstruct_path():
    cost = grid[target]
    path = {target}
    nxt = prev[target]
    while nxt:
        cost += grid[nxt]
        path.add(nxt)
        nxt = prev[nxt]
    cost -= grid[(0, 0)]
    return path, cost


if DIJKSTRA:
    dist, prev = dijkstra_shortes_path()
    esti = dist
else:
    dist, esti, prev = a_star()
path, cost = reconstruct_path()

if EXAMPLE:
    for m, row in enumerate(data):
        for n, v in enumerate(row):
            # v = esti.get((m, n), "x")
            v = grid.get((m, n))
            print(f"({v:2})" if (m, n) in path else f" {v:2} ", end="")
        print()
print(f'total cost: {cost}')
