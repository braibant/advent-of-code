from collections import defaultdict, deque
from typing import Dict, NamedTuple, Tuple, Set
import heapq

cost = {"A": 1, "B": 10, "C": 100, "D": 1000}
hallway = {(x, 1) for x in range(1, 12) if x not in [3, 5, 7, 9]}
targetsx = {"A": 3, "B": 5, "C": 7, "D": 9}


def targets(kind: str):
    x = targetsx[kind]
    return {(x, y) for y in range(2, 4)}


def adj(x, y):
    return [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]


N = NamedTuple("N", [("pos", Tuple[int, int]), ("final", bool), ("kind", str)])


def occupied(ns: Tuple[N]) -> Set[Tuple[int, int]]:
    return {n.pos for n in ns}


class T:
    def __init__(self, tiles, initial):
        self.tiles = tiles
        self.initial = initial

    def path(self, pos, start, tgt):
        q = deque((start, 0))
        visited = set()
        occupied_ = occupied(pos)
        while q:
            p, cost = q.popleft()

            if p in visited:
                continue
            visited.add(p)

            if p == tgt:
                return cost

            for next in adj(p[0], p[1]):
                if next in self.tiles and next not in occupied_:
                    q.append(next, cost + 1)

    def solve(self):
        # There are a bunch of constraints that actually simplify the problem.
        # 1) Animals move to the hallway, then to their final rooms.
        # 2) We know the final layout.
        h = [(0, self.initial)]
        while h:
            c, pos = heapq.heappop(h)
            for i in range(len(pos)):
                n = pos[i]
                for h in hallway + targets(n.kind):
                    moves = self.path(pos, n.pos, h)
                    if moves is not None:
                        pos1 = list(pos)
                        m = n.replace_(pos=h)
                        pos1[i] = m
                        heapq.heappush(h, (c + cost[n.kind] * moves, tuple(pos1)))


def parse(input: str) -> T:
    tiles = set()
    pos = defaultdict(set)
    for y, line in enumerate(input.splitlines()):
        for x, char in enumerate(line.split()):
            if char == "#" or char == " ":
                continue
            tiles.add((x, y))
            if char != ".":
                pos[char].add((x, y))
    T(tiles, pos)


def run(input: str):
    w = parse(input)
    print(w.solve())
