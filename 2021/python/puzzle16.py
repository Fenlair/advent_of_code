from itertools import islice
from math import prod

example = "9C0141080250320F1802104A08"
real= open("../inputs/day16.txt").read().strip()

def hex2bin(hnum):
    return iter((bin(int(hnum, 16))[2:]).zfill(len(hnum)*4))


def take_str(n, iterable):
    res = "".join(islice(iterable, n))
    if res == "": raise StopIteration
    return res


def take_int(n, iterable):
    return int(take_str(n, iterable), 2)


def literal_value(it, acc=0):
    is_last = not take_int(1, it)
    cur_val = take_int(4, it)
    if is_last:
        return 16*acc + cur_val
    return literal_value(it, 16*acc+cur_val)


fn_map = {0: sum,
          1: prod,
          2: min,
          3: max,
          5: lambda vs: 1 if vs[0] > vs[1] else 0,
          6: lambda vs: 1 if vs[0] < vs[1] else 0,
          7: lambda vs: 1 if vs[0] == vs[1] else 0}

def interpret(packet):
    global version
    version += take_int(3, packet)
    type_id = take_int(3, packet)

    if type_id == 4:
        return literal_value(packet)
    else:
        vals = []
        len_type = take_int(1, packet)
        if len_type:
            length = take_int(11, packet)  # in packets
            for _ in range(length):
                vals.append(interpret(packet))
        else:
            length = take_int(15, packet)  # in bits
            subpackets = iter(take_str(length, packet))
            while True:
                try:
                    vals.append(interpret(subpackets))
                except StopIteration:
                    break
        return fn_map[type_id](vals)


version = 0
packet = hex2bin(real)
res = interpret(packet)
print("Puzzle1:", version)
print("Puzzle2:", res)

for ex, res in [("C200B40A82", 3),
                ("04005AC33890", 54),
                ("880086C3E88112", 7),
                ("CE00C43D881120", 9),
                ("D8005AC2A8F0", 1),
                ("F600BC2D8F", 0),
                ("9C005AC2F8F0", 0),
                ("9C0141080250320F1802104A08", 1)]:
    packet = hex2bin(ex)
    if res != interpret(packet):
        raise ValueError
