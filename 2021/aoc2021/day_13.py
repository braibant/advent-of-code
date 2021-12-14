from typing import List, Set, Tuple


def fold_vertically(x: int, points):
    acc = set()
    for p in points:
        if p[0] > x:
            d = p[0] - x
            acc.add((x - d, p[1]))
        else:
            acc.add((p[0], p[1]))
    return acc


def fold_horizontally(y: int, points):
    acc = set()
    for p in points:
        if p[1] > y:
            d = p[1] - y
            acc.add((p[0], y - d))
        else:
            acc.add((p[0], p[1]))
    return acc


pass


def next(points, fold):
    if fold.startswith("x"):
        x = int(fold.removeprefix("x="))
        return fold_vertically(x, points)
    else:
        y = int(fold.removeprefix("y="))
        return fold_horizontally(y, points)


def part1(points, folds):
    return len(next(points, folds[0]))


def pp(points: Set[Tuple[int, int]]):
    s = []
    width = 1 + max(map(lambda p: p[0], points))
    height = 1 + max(map(lambda p: p[1], points))
    for y in range(height):
        s.append("{} ".format(y))
        for x in range(width):
            if (x, y) in points:
                s.append("#")
            else:
                s.append(".")
        s.append("\n")
    print("".join(s))


def part2(points: Set[Tuple[int, int]], folds):
    for fold in folds:
        points = next(points, fold)
    pp(points)


def run(input: str):
    parts = input.split("\n\n")
    points = set(map(lambda l: tuple(map(int, l.split(","))), parts[0].splitlines()))
    folds = list(map(lambda l: l.removeprefix("fold along "), parts[1].splitlines()))
    print(part1(points, folds))
    part2(points, folds)
