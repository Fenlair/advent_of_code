import math
import re

example = "target area: x=20..30, y=-10..-5"
real = "target area: x=144..178, y=-100..-76"

def parse(data):
    m = re.search(r"x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)", data)
    xa, xb, ya, yb = m.groups()
    return ((int(xa), int(xb)), (int(ya), int(yb)))

def step(p, v):
    p = [p[0] + v[0], p[1] + v[1]]
    v[0] = v[0]-1 if v[0]>0 else 0 if v[0]==0 else v[0]+1
    v[1] -= 1
    return p, v

def is_in_target(p):
    x = min(target[0]) <= p[0] <= max(target[0])
    y = min(target[1]) <= p[1] <= max(target[1])
    return x and y

def calc_x_vel(target):
    """Calculate minimum velocity to reach target."""
    t = min(target[0])
    v = math.ceil(-1/2 + math.sqrt(1/4 + 2*t))
    assert (v+1)/2*v <= max(target[0])
    return v

def throw(v):
    global record, made_it
    v_orig = tuple(v)
    p = [0, 0]
    while True:
        p, v = step(p, v)
        if p[0] > max(target[0]):
            break
        if v[1] <= 0 and p[1] < min(target[1]):
            break
        if p[1] >= record.get(v_orig, -1000):
            record[v_orig] = p[1]
        if is_in_target(p):
            made_it.add(v_orig)
            break

target = parse(real)
record = {}
made_it = set()

for y in range(min(target[1]), 200):
    for x in range(calc_x_vel(target), max(target[0])+1):
        throw([x, y])

record = {k: v for k, v in record.items() if k in made_it}
m = max(record, key=record.get)
print(f'Puzzle1: {record[m]} for initial velocity of {m}')
print(f'Puzzle2: {len(made_it)}')
