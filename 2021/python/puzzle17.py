example = "target area: x=20..30, y=-10..-5"
real = "target area: x=144..178, y=-100..-76"

target = ((20, 30), (-10, -5))

def step(p, v):
    p = (p[0] + v[0], p[1] + v[1])
    v[0] = v[0]-1 if v[0]>0 else 0 if v[0]==0 else v[0]+1
    v[1] -= 1
    return p, v

def is_in_target(p):
    x = target[0][0] <= p[0] <= target[0][1]
    y = target[1][0] <= p[1] <= target[1][1]
    return x and y

def throw(v):
    global record, made_it
    v_orig = tuple(v)
    p = 0, 0
    for _ in range(100):
        p, v = step(p, v)
        if p[1] > record.get(v_orig, 0):
            record[v_orig] = p[1]
        if is_in_target(p):
            made_it.add(v_orig)
            break

record = {}
made_it = set()
for y in range(20):
    throw([6, y])
record = {k: v for k, v in record if k in made_it}
print(record)
