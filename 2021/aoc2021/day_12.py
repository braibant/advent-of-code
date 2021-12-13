# observations:
# - edges in the input file are not oriented.
# - we may visit the same vertex multiple times.

from typing import Tuple, List
from collections import defaultdict


class Graph:
    def __init__(self, input: List[List[str]]):
        self.edges = defaultdict(set)
        for l in input:
            self.edges[l[0]].add(l[1])
            self.edges[l[1]].add(l[0])

    def paths(self, twice):
        paths = set()

        def visit(n: str, used, path):
            if n == "end":
                paths.add(",".join(path))
            elif n.islower() and (
                used[n] == 0 or (twice is not None and n == twice and used[n] == 1)
            ):
                # small rooms can be used once
                used[n] += 1
                path.append(n)
                for e in self.edges[n]:
                    visit(e, used, path)
                path.pop()
                used[n] -= 1
            elif n.isupper():
                path.append(n)
                for e in self.edges[n]:
                    visit(e, used, path)
                path.pop()

        visit("start", defaultdict(int), [])
        return paths

    def part1(self) -> int:
        return len(self.paths(None))

    def part2(self) -> int:
        paths = set()
        for e in self.edges.keys():
            if e.islower() and e != "start" and e != "end":
                paths = paths.union(self.paths(e))
        return len(paths)


def run(input: str):
    edges = list(map(lambda a: a.split("-"), input.splitlines()))
    g = Graph(edges)
    print(g.part1())
    print(g.part2())
