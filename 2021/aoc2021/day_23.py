import ast
from time import perf_counter as pfc
from collections import defaultdict, deque
from typing import FrozenSet, NamedTuple, Tuple, Set, Callable, Optional, List
import heapq
import itertools


Pos = Tuple[int, int]

cost = {"A": 1, "B": 10, "C": 100, "D": 1000}
hallway = {(x, 1) for x in range(1, 12) if x not in [3, 5, 7, 9]}
targetsx = {"A": 3, "B": 5, "C": 7, "D": 9}


def adj(x, y):
    return [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]


State = FrozenSet[Tuple[str, Pos]]

from functools import cache


@cache
def path(src: Pos, tgt: Pos, tiles: FrozenSet[Pos], occupied: FrozenSet[Pos]):
    q = deque()
    q.append((src, 0))

    visited = set()

    while q:
        (p, cost) = q.popleft()

        if p in visited:
            continue

        visited.add(p)

        if p == tgt:
            return cost

        for next in adj(p[0], p[1]):
            if next in tiles and next not in occupied:
                q.append((next, cost + 1))


class Instance:
    def __init__(self, size: int, tiles: FrozenSet[Pos]):
        self.size = size
        self.tiles = tiles

        acc = []
        for kind, x in targetsx.items():
            for y in range(2, self.size):
                acc.append((kind, (x, y)))

        self.target = frozenset(acc)

    def targets(self, kind: str):
        x = targetsx[kind]
        return [(x, y) for y in reversed(range(2, self.size))]

    def locked(self, state: State, kind, pos):
        if pos[0] != targetsx[kind]:
            return False

        if pos[1] == self.size:
            return True

        p1 = (pos[0], pos[1] + 1)
        if (kind, p1) in state and self.locked(state, kind, p1):
            return True

        return False

    def move(self, state: State) -> List[Tuple[State, int]]:
        moves = []
        positions = list(state)

        for i in range(len(positions)):
            kind, pos = positions[i]

            if self.locked(state, kind, pos):
                continue

            tgts = []
            tgts_for_kind = self.targets(kind)
            if pos[1] == 1:
                # Currently in hallay
                occupiers = {m[0] for m in state if m[1] in tgts_for_kind}

                if not occupiers or occupiers == {kind}:
                    tgts.extend(tgts_for_kind)
            else:
                tgts.extend(hallway)

            for tgt in tgts:

                c = path(pos, tgt, self.tiles, frozenset({m[1] for m in positions}))
                if c is not None:
                    p1 = positions.copy()
                    p1.pop(i)
                    p1.append((kind, tgt))
                    moves.append((frozenset(p1), c * cost[kind]))

        return moves

    def final(self, state) -> bool:
        return state == self.target

    def heuristic_cost(self, state: State):
        h = 0
        empty = frozenset()
        for (kind, pos) in state:
            x = targetsx[kind]
            c = path(pos, (x, self.size - 1), self.tiles, empty)
            if c is not None:
                h += cost[kind] * c
        return h


def astar0(graph, start) -> Optional[int]:
    # Let f be the total cost of anode, g the cost between the current node and
    # the start node, and h be the heurisitc distance to an end node. We have f
    # = g + h.

    # [(f, g, node)]
    todo = [(graph.heuristic_cost(start), 0, start)]
    gscore = dict()
    gscore[start] = 0

    while todo:
        f, g, n = heapq.heappop(todo)

        if gscore[n] < g:
            continue

        if graph.final(n):
            return g

        for pos, cost in graph.move(n):
            g1 = g + cost
            h1 = graph.heuristic_cost(pos)
            f1 = g1 + h1
            if pos not in gscore or g1 < gscore[pos]:
                gscore[pos] = g1
                # I should remove other occurences of [pos] in todo.
                heapq.heappush(todo, (f1, g1, pos))

    return None


def astar(graph, start):
    # import cProfile, pstats

    # profiler = cProfile.Profile()
    # profiler.enable()
    x = astar0(graph, start)
    # profiler.disable()
    # stats = pstats.Stats(profiler).sort_stats("ncalls")
    # stats.print_stats()
    return x


def parse(input: str) -> Tuple[FrozenSet[Pos], State]:
    tiles = set()
    pos = []
    for y, line in enumerate(input.splitlines()):
        for x, char in enumerate(line):
            if char == "#" or char == " ":
                continue
            tiles.add((x, y))
            if char != ".":
                n = (char, (x, y))
                pos.append(n)
    return (frozenset(tiles), frozenset(pos))


def run(input: str):
    # Part 1
    tiles, pos = parse(input)
    inst = Instance(4, tiles)
    print(astar(inst, pos))

    # Part 2
    l = input.splitlines()
    l.insert(3, "  #D#C#B#A#")
    l.insert(4, "  #D#B#A#C#")
    input2 = "\n".join(l)
    tiles, pos = parse(input2)
    inst = Instance(6, tiles)
    print(astar(inst, pos))
