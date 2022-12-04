from functools import reduce
from itertools import chain
from typing import Optional
from math import floor, ceil
from copy import deepcopy

class Tree:
    def __init__(self, left=None, right=None, value=None) -> None:
        self.left: Optional[Tree] = left
        self.right: Optional[Tree] = right
        self.parent: Optional[Tree] = None
        self.value: Optional[int] = value
        if left: left.parent = self
        if right: right.parent = self

    def __iter__(self):
        if self.value is not None:
            yield self
        it = []
        if self.left:  it.append(iter(self.left))
        if self.right: it.append(iter(self.right))
        for n in chain(*it):
            yield n

    def __repr__(self) -> str:
        def prn(n: Tree):
            if n.value is not None:
                return str(n.value)
            return f'[{prn(n.left)},{prn(n.right)}]'
        return prn(self)

    @property
    def path(self):
        l, p = [], self
        while p := p.parent:
            l.append(p)
        return l

    @property
    def depth(self):
        return len(self.path)

    @property
    def is_left(self):
        if self.parent and self is self.parent.left:
            return True
        else:
            return False

    def to_the_left(self) -> Optional['Tree']:
        p = self
        while p := p.parent:
            if p.parent and not p.is_left:
                n = p.parent.left
                while n.value is None:
                    n = n.right
                return n
        return None

    def to_the_right(self):
        p = self
        while p := p.parent:
            if p.parent and p.is_left:
                n = p.parent.right
                while n.value is None:
                    n = n.left
                return n
        return None

    # def deepcopy(self):
    #     def foo(t: Tree):
    #         if t.value is not None:
    #             return Tree(value=self.value)
    #         assert t.left is not None
    #         assert t.right is not None
    #         return Tree(foo(t.left), foo(t.right))
    #     return foo(self)

def parse(s: str) -> Tree:
    if s.startswith("["):
        assert s.endswith("]")
        i = level = 0
        a = b = ""
        for i, c in enumerate(s[1:-1]):
            if c == "[":
                level += 1
            elif c == "]":
                level -= 1
            assert level >= 0
            if c == "," and level == 0:
                break
            a += c
        for c in s[i+2:-1]:
            b += c
        return Tree(left=parse(a), right=parse(b))
    assert len(s) == 1
    return Tree(value=int(s))

def add(eq1, eq2):
    return reduce_eq(Tree(eq1, eq2))

def explode(eq: Tree) -> Optional[Tree]:
    eq = deepcopy(eq)
    for n in eq:
        if n.depth > 4 and n.value is not None:
            left_node = n.to_the_left()
            if left_node:
                left_node.value += n.value
            n = n.parent.right
            right_node = n.to_the_right()
            if right_node:
                right_node.value += n.value
            new = Tree(value=0)
            new.parent = n.parent.parent
            if n.parent.is_left:
                n.parent.parent.left = new
            else:
                n.parent.parent.right = new
            return eq
    return None


def split(eq: Tree) -> Optional[Tree]:
    eq = deepcopy(eq)
    for n in eq:
        if n.value > 9:
            n.left = Tree(value=floor(n.value/2))
            n.right = Tree(value=ceil(n.value/2))
            n.left.parent = n
            n.right.parent = n
            n.value = None
            return eq
    return None


def reduce_eq(eq):
    if neq := explode(eq):
        return reduce_eq(neq)
    elif neq := split(eq):
        return reduce_eq(neq)
    else:
        return eq

def magnitude(eq):
    if eq.value is not None:
        return eq.value
    return 3 * magnitude(eq.left) + 2 * magnitude(eq.right)

# ex = add(parse("[[[[4,3],4],4],[7,[[8,4],9]]]"), parse("[1,1]"))
ex = "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]"
ex = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
real = open("../inputs/2021_day18.txt").read()
data = list(map(parse, real.splitlines()))
print(magnitude(reduce(add, data[1:], data[0])))

res = []
for eq1 in data:
    for eq2 in data:
        if eq1 is eq2:
            continue
        res.append(magnitude(add(eq1, eq2)))
print(max(res))
