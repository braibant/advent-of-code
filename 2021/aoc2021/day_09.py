from typing import List, Tuple
from collections import defaultdict


class M:
    def __init__(self, lines: List[str]):
        self.lines = lines
        self.n = len(lines)
        self.m = len(lines[0])

    def height(self, row, col):
        return int(self.lines[row][col])

    def neighbours(self, row: int, col: int) -> List[Tuple[int, int]]:
        acc = []
        for (dr, dc) in [(-1, 0), (1, 0), (0, 1), (0, -1)]:
            r = row + dr
            c = col + dc

            if 0 <= r and r < self.n and 0 <= c and c < self.m:
                acc.append((r, c))
        return acc

    def low_points(self) -> List:
        lp = []
        for row in range(self.n):
            for col in range(self.m):
                l = self.neighbours(row, col)
                h = self.height(row, col)
                if all([h < self.height(r, c) for (r, c) in l]):
                    lp.append((row, col))
        return lp

    def part1(self) -> int:
        def rl(rc):
            return 1 + self.height(rc[0], rc[1])

        return sum(map(rl, self.low_points()))

    # For part2, we rely on the assumption (stated in the task description) that
    # each tile is part of exactly one basin. This is a property of the input,
    # not a general one.
    def part2(self) -> int:
        basin = dict()
        basin_size = defaultdict(int)

        def fill(row: int, col: int, b: int):
            todo = [(row, col)]
            while todo:
                row, col = todo.pop()
                if (row, col) not in basin and self.height(row, col) != 9:
                    basin[(row, col)] = b
                    basin_size[b] += 1
                    todo.extend(self.neighbours(row, col))

        lp = self.low_points()
        for i, (r, c) in enumerate(lp):
            fill(r, c, i)
        acc = 1
        for s in sorted(basin_size.values())[-3:]:
            acc *= s
        return acc


def run(input: str):
    map = M(input.splitlines())
    print(map.part1())
    print(map.part2())
