from typing import Tuple, List
from collections import defaultdict


rotations = set()
for i in [0, 1, 2]:
    for j in [0, 1, 2]:
        for k in [0, 1, 2]:
            if i != j and i != k and j != k:
                for si in [-1, 1]:
                    for sj in [-1, 1]:
                        for sk in [-1, 1]:
                            rotations.add((si * i, sj * j, sk * k))


def sign(i: int) -> int:
    if i >= 0:
        return 1
    else:
        return -1


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
        return Point([sign(i) * self.v[abs(i)] for i in r])

    def sub(self, p: "Point"):
        return Point(list(map(lambda x: x[0] - x[1], zip(self.v, p.v))))

    def add(self, p: "Point"):
        return Point(list(map(lambda x: x[0] + x[1], zip(self.v, p.v))))


def distances(points):
    dist = set()
    for i in range(len(points)):
        for j in range(i + 1, len(points)):
            p1 = points[i]
            p2 = points[j]
            d = p1.distance(p2)
            dist.add(d)
    return dist


def similarity(p1, p2):
    d1 = distances(p1)
    d2 = distances(p2)
    return len(d1.intersection(d2))


def parse_scan(scan: str):
    lines = scan.splitlines()
    return list(map(Point.from_str, lines[1:]))


def match(scan0, scan1):
    scan0 = set(scan0)
    for r in rotations:
        rscan1 = list(map(lambda p: p.rotate(r), scan1))
        for p in scan0:
            for rp in rscan1:
                t = rp.sub(p)
                trscan1 = set(map(lambda a: a.sub(t), rscan1))
                if len(scan0.intersection(trscan1)) >= 12:
                    return r, t


# High level strategy: use similarity of point to point distances between scans
# as a proxy for matching of scanners. Scanners with the highest similarity are
# nearby. Match the scanners together by rotating along all axis, then merge the
# scans together (and recompute similarity for all the other elements). Iterate.
# There are two main ways we could handle the combination: pair wise matching of
# scans with high similarity would allow to compute a rotation R and a
# translation T for the pair, which we can then assemble together. Another
# strategy is to build a big anchor (starting from let's say scan 0), and then
# fold in each scan.
def run(input: str):
    scans = list(map(parse_scan, input.split("\n\n")))
    print(len(rotations))

    for i in range(len(scans) - 1):
        j = max(range(i + 1, len(scans)), key=lambda j: similarity(scans[i], scans[j]))
        # print(i, j, similarity(scans[i], scans[j]))
        r, t = match(scans[i], scans[j])
        print(r, t)
    # matrix = []
    # for i in range(len(scanners)):
    #     row = []
    #     di = distances(scanners[i])
    #     row.append("[{}]".format(len(scanners[i])))
    #     for j in range(len(scanners)):
    #         if j < i + 1:
    #             row.append("")
    #         else:
    #             dj = distances(scanners[j])
    #             simi = similarity(di, dj)
    #             s = "{}".format(simi)
    #             row.append(s)
    #     matrix.append(row)

    # s = [[str(e) for e in row] for row in matrix]
    # lens = [max(map(len, col)) for col in zip(*s)]
    # fmt = "\t".join("{{:{}}}".format(x) for x in lens)
    # table = [fmt.format(*row) for row in s]
    # print("\n".join(table))

    # print("\n".join(["\t".join([str(cell) for cell in row]) for row in matrix]))
