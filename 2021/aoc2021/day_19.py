from email.policy import default
from typing import Tuple, List, FrozenSet
from collections import defaultdict


# If we restrict ourselves to rotations around the x,y,z angles that are
# multiples of 90 degrees, there should be 24 rotations. This
# https://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm
# is a very useful resource. It describes the 3d matrix encoding of all such 24
# rotations.

_x = 0
_y = 1
_z = 2

pos = 1
neg = -1

rotations = [
    (((_x, pos), (_y, pos), (_z, pos))),
    (((_x, pos), (_z, pos), (_y, neg))),
    (((_x, pos), (_y, neg), (_z, neg))),
    (((_x, pos), (_z, neg), (_y, pos))),
    (((_x, neg), (_y, pos), (_z, neg))),
    (((_x, neg), (_z, neg), (_y, neg))),
    (((_x, neg), (_y, neg), (_z, pos))),
    (((_x, neg), (_z, pos), (_y, pos))),
    (((_y, pos), (_x, pos), (_z, neg))),
    (((_y, pos), (_z, neg), (_x, neg))),
    (((_y, pos), (_x, neg), (_z, pos))),
    (((_y, pos), (_z, pos), (_x, pos))),
    (((_y, neg), (_x, pos), (_z, pos))),
    (((_y, neg), (_z, pos), (_x, neg))),
    (((_y, neg), (_x, neg), (_z, neg))),
    (((_y, neg), (_z, neg), (_x, pos))),
    (((_z, pos), (_x, pos), (_y, pos))),
    (((_z, pos), (_y, pos), (_x, neg))),
    (((_z, pos), (_x, neg), (_y, neg))),
    (((_z, pos), (_y, neg), (_x, pos))),
    (((_z, neg), (_x, pos), (_y, neg))),
    (((_z, neg), (_y, neg), (_x, neg))),
    (((_z, neg), (_x, neg), (_y, pos))),
    (((_z, neg), (_y, pos), (_x, pos))),
]


class Point:
    def __init__(self, v: Tuple[int, int, int]):
        self.v = v

    def __str__(self):
        return "({}, {}, {})".format(self.v[0], self.v[1], self.v[2])

    def __eq__(self, other):
        """Overrides the default implementation"""
        if isinstance(other, Point):
            return all(map(lambda x: x[0] == x[1], zip(self.v, other.v)))
        return NotImplemented

    def __hash__(self):
        """Overrides the default implementation"""
        return hash(tuple(self.v))

    @classmethod
    def from_str(cls, s: str):
        v = tuple(map(int, s.split(",")))
        return Point(v)

    def distance(self, other: "Point") -> int:
        return (
            abs(self.v[0] - other.v[0])
            + abs(self.v[1] - other.v[1])
            + abs(self.v[2] - other.v[2])
        )

    def rotate(self, r):
        return Point(tuple([s * self.v[i - 1] for (i, s) in r]))

    def sub(self, p: "Point"):
        return Point(tuple(map(lambda x: x[0] - x[1], zip(self.v, p.v))))

    def add(self, p: "Point"):
        return Point(tuple(map(lambda x: x[0] + x[1], zip(self.v, p.v))))


def parse_scan(scan: str):
    lines = scan.splitlines()
    return set(map(Point.from_str, lines[1:]))


# High level strategy: use similarity of point to point distances between scans
# as a proxy for matching of scanners. Scanners with the highest similarity are
# nearby. Match the scanners together by rotating along all axis, then merge the
# scans together (and recompute similarity for all the other elements). Iterate.
# There are two main ways we could handle the combination: pair wise matching of
# scans with high similarity would allow to compute a rotation R and a
# translation T for the pair, which we can then assemble together. Another
# strategy is to build a big anchor (starting from let's say scan 0), and then
# fold in each scan.

import itertools


def run(input: str):
    scans = list(map(parse_scan, input.split("\n\n")))
    base = scans[0]
    scans = scans[1:]
    coords = [Point((0, 0, 0)), Point((1, 0, 0))]

    while scans:
        scan = scans.pop(0)
        progress = False

        for r in rotations:
            if progress:
                break
            offsets = defaultdict(int)
            for b in base:
                for p in scan:
                    rp = p.rotate(r)
                    offset = rp.sub(b)
                    offsets[offset] += 1
            for offset, n in offsets.items():
                if n >= 12:
                    progress = True
                    coords.append(offset)
                    for p in scan:
                        rp = p.rotate(r)
                        base.add(rp.sub(offset))
        if not progress:
            scans.append(scan)
    print(len(base))

    print(max(a.distance(b) for a, b in itertools.product(coords, coords)))
