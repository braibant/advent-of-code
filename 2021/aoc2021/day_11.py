def grid(input: str):
    d = dict()
    for i, line in enumerate(input.splitlines()):
        for j, char in enumerate(line):
            d[i, j] = int(char)
    return d


def step(grid):
    flashed = set()
    incr = list(grid.keys())
    while incr:
        p = incr.pop()
        grid[p] += 1
        if grid[p] >= 10 and p not in flashed:
            flashed.add(p)
            for dx in [-1, 0, 1]:
                for dy in [-1, 0, 1]:
                    x = dx + p[0]
                    y = dy + p[1]
                    if (dx != 0 or dy != 0) and 0 <= x and x < 10 and 0 <= y and y < 10:
                        incr.append((x, y))
    for p in flashed:
        grid[p] = 0
    return len(flashed)


def part1(grid):
    flashes = 0
    for i in range(100):
        flashes += step(grid)
    return flashes


def part2(grid):
    i = 1
    while True:
        if step(grid) == len(grid):
            return i
        i += 1


def run(input: str):
    g = grid(input)
    print(part1(g))
    g = grid(input)
    print(part2(g))
