import heapq
from typing import List, Tuple


class ShortestPath:
    def run(self, graph, src, dst):
        heap = []
        distance = dict()
        distance[src] = 0
        heapq.heappush(heap, (0, src))
        while heap:
            dist, node = heapq.heappop(heap)
            if node == dst:
                return dist

            if dist > distance[node]:
                continue

            for (next, drisk) in graph.neighbors(node):
                risk = drisk + dist
                if next not in distance or risk < distance[next]:
                    heapq.heappush(heap, (risk, next))
                    distance[next] = risk
        return None


class GridGraph:
    def __init__(self, input):
        self.grid = dict()
        lines = input.splitlines()
        self.width = len(lines[0])
        self.height = len(lines)
        for y, line in enumerate(lines):
            for x, c in enumerate(line):
                self.grid[(x, y)] = int(c)

    def neighbors(self, node: Tuple[int, int]) -> List[Tuple[int, int]]:
        x0, y0 = node
        acc = []
        for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            x = x0 + dx
            y = y0 + dy
            if 0 <= x and 0 <= y:
                x1, ix = x % self.width, x // self.width
                y1, iy = y % self.height, y // self.height
                rl = self.grid[(x1, y1)] + ix + iy
                while rl >= 10:
                    rl = rl - 9
                acc.append(((x, y), rl))
        return acc


def run(input):
    graph = GridGraph(input)
    sp = ShortestPath()
    print(sp.run(graph, (0, 0), (graph.width - 1, graph.height - 1)))
    print(sp.run(graph, (0, 0), (5 * graph.width - 1, 5 * graph.height - 1)))
