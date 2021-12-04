from typing import List


class Grid:
    def __init__(self, grid):
        self.grid = grid
        self.marks = [[False] * 5 for i in range(5)]

    def __str__(self):
        return "{}\n{}".format(self.grid, self.marks)

    def mark(self, num: int):
        for i in range(5):
            for j in range(5):
                if self.grid[i][j] == num:
                    self.marks[i][j] = True
                    return self.check(i, j)

    def check(self, i, j):
        a = all([self.marks[i][k] for k in range(5)])
        b = all([self.marks[k][j] for k in range(5)])
        return a or b

    def score(self):
        score = 0
        for i in range(5):
            for j in range(5):
                if not self.marks[i][j]:
                    score += self.grid[i][j]
        return score


def parse_grid_line(s: str):
    return list(map(int, s.split()))


def parse_grid(s: str):
    lines = s.splitlines()
    return Grid(list(map(parse_grid_line, lines)))


def part1(nums, grids):
    for n in nums:
        for g in grids:
            if g.mark(n):
                return g.score() * n


def part2(nums, grids):
    boards = set()

    for n in nums:
        for gi in range(len(grids)):
            if grids[gi].mark(n):
                boards.add(gi)
            if len(boards) == len(grids):
                return grids[gi].score() * n


def parse(input: str):
    parts = input.split("\n\n")

    nums = list(map(int, parts[0].split(",")))
    grids = list(map(parse_grid, parts[1:]))
    return (nums, grids)


def run(input: str):
    (nums, grids) = parse(input)
    print(part1(nums, grids))
    print(part2(nums, grids))
