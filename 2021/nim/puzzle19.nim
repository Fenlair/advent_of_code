import system/io
import std/tables
import std/strutils
from std/sequtils import map

type
  coordinate = tuple[x: int, y: int, z: int]
  coordinates = seq[coordinate]
  scanners = seq[coordinates]
  ocmap = Table[coordinate, coordinates]

proc parse(fn: string): scanners =
  var x, y, z: int
  var cors: coordinates
  for line in fn.lines:
    if "scanner" in line:
      cors = @[]
    elif line == "":
      result.add(cors)
    else:
      (x, y, z) = line.split(',').map(parseInt)
      cors.add((x: x, y: y, z: z))

echo parse("../inputs/example19.txt")

iterator trafos(): proc(c: coordinate) =
  for xm in [1, -1]:
    for (xi, yi, zi) in @[(1, 2, 3), (2, 3, 1), (3, 1, 2)]:
      for (ym, zm) in [(1, 1), (-1, -1)]:
        yield proc(c: coordinate): coordinate = (x: xm * c[xi], y: ym * c[yi], z: zm * c[zi])
    for (xi, yi, zi) in [(1, 3, 2), (2, 1, 3), (3, 2, 1)]:
      for (ym, zm) in [(1, -1), (-1, 1)]:
        yield proc(c: coordinate): coordinate = (x: xm * c[xi], y: ym * c[yi], z: zm * c[zi])

iterator trafo(cs: coordinates): coordinates =
  for t_fn in trafos():
    var cors: coordinates = @[]
    for c in cs:
      cors.add(t_fn(c))
    yield cors

proc diff(c1, c2: coordinate): coordinate =
  result = (x: c2.x - c1.x, y: c2.y - c1.y, z: c2.z - c1.z)

proc map_offsets_to_cors(cors1, cors2: coordinates): ocmap =
  for cr in ref:
    for cs in subt:
      offset = diff(cr, cs)
      result[offset].add(cr)

proc is_criterea_met(tbl: ocmap): Tuple[bool, coordinate] =
  for o, cors in tbl:
    if len(cors) >= 12:
      return (True, o)
  return (False, nil)

proc match(ref, sub: coordinates): coordinates =
  var
    offset: coordinate
    tbl: ocmap
    is_matched = False

  for subt in trafo(sub):
    tbl = map_offsets_to_cors(ref, subt)
    (is_matched, offset) = is_criterea_met(tbl)
    if is_matched: break



      

  

  # x  y  z
  # x -y -z
  # x  z -y
  # x -z  y

  # y  z  x
  # y -z -x
  # y  x -z
  # y -x  z

  # z  x  y
  # z -x -y
  # z  y -x
  # z -y  x
