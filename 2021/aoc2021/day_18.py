from multiprocessing.sharedctypes import Value
from typing import List, Optional
import copy


class Node:
    left: Optional["Node"]
    right: Optional["Node"]
    value: Optional[int]

    def __init__(self):
        self.left = None
        self.right = None
        self.value = None

    def is_value(self):
        return self.value is not None and self.left is None and self.right is None

    def is_pair(self):
        return self.value is None and self.left is not None and self.right is not None

    def split(self):
        x = self.value
        assert x is not None and x >= 10
        self.value = None
        self.left = value(x // 2)
        self.right = value((x + 1) // 2)

    def explode(self):
        if self.left is None or self.right is None:
            raise ValueError
        # assert self.is_pair()
        # assert self.left.is_value()
        # assert self.right.is_value()

        l, r = self.left.value, self.right.value
        self.value = 0
        self.left = None
        self.right = None
        return l, r

    def str_rec(self, b):
        if self.left is not None and self.right is not None:
            assert self.is_pair()
            b.append("[")
            self.left.str_rec(b)
            b.append(",")
            self.right.str_rec(b)
            b.append("]")
        else:
            b.append(str(self.value))

    def str(self):
        l = []
        self.str_rec(l)
        return "".join(l)

    def magnitude(self) -> int:
        if self.value is not None:
            return self.value
        else:
            return self.left.magnitude() * 3 + self.right.magnitude() * 2


def value(value: int) -> Node:
    n = Node()
    n.value = value
    return n


def add_rightmost(node, x):
    if node.is_value():
        node.value += x
    else:
        add_rightmost(node.right, x)


def add_leftmost(node, x):
    if node.is_value():
        node.value += x
    else:
        add_leftmost(node.left, x)


def explode(n: Node, depth, left, right) -> bool:
    if n.is_value():
        return False
    elif depth >= 4:
        l, r = n.explode()
        if left is not None:
            add_rightmost(left, l)
        if right is not None:
            add_leftmost(right, r)
        return True
    else:
        return explode(n.left, depth + 1, left, n.right) or explode(
            n.right, depth + 1, n.left, right
        )


def split(n: Node):
    if n.is_value():
        if n.value >= 10:
            n.split()
            return True
        return False
    else:
        return split(n.left) or split(n.right)


def reduce(n):
    while explode(n, 0, None, None) or split(n):
        pass


def add(n1, n2):
    n = Node()
    n.left = n1
    n.right = n2
    reduce(n)
    return n


def addn(l):
    acc = l[0]
    for n in l[1:]:
        acc = add(acc, n)
    return acc


def parse_line(s: str) -> Node:
    stack = []
    for c in s:
        if c == "]":
            n = Node()
            n.right = stack.pop()
            n.left = stack.pop()
            stack.append(n)
        elif "0" <= c and c <= "9":
            n = Node()
            n.value = int(c)
            stack.append(n)
        else:
            pass
    assert len(stack) == 1
    return stack.pop()


def parse(input: str) -> List[Node]:
    return [parse_line(s) for s in input.splitlines()]


def run(input: str):
    l = parse(input)
    n = addn(copy.deepcopy(l))
    print(n.magnitude())

    m = 0
    for a in range(len(l)):
        for b in range(a + 1, len(l)):
            n = add(copy.deepcopy(l[a]), copy.deepcopy(l[b]))
            k = n.magnitude()
            if k > m:
                m = k
            n = add(copy.deepcopy(l[b]), copy.deepcopy(l[a]))
            k = n.magnitude()
            if k > m:
                m = k

    print(m)
