from typing import List, Tuple
from collections import defaultdict


def parse_int_pair(s: str):
    parts = s.split(",")
    return tuple(map(int, parts))


def parse_line(line: str):
    parts = line.split(" -> ")
    return list(map(parse_int_pair, parts))


def slope(a, b):
    # we want a.x < b.x
    if a[0] > b[0]:
        (a, b) = (b, a)
    if a[1] < b[1]:
        return a, b, 1
    else:
        return a, b, -1


def rasterize_vents(
    lines: List[Tuple[Tuple[int, int], Tuple[int, int]]], diagonals: bool
):
    vents = defaultdict(int)
    for [a, b] in lines:
        # Horizontal or vertical
        if a[0] == b[0]:
            low, high = min(a[1], b[1]), max(a[1], b[1])
            for i in range(low, high + 1):
                vents[a[0], i] += 1
        elif a[1] == b[1]:
            low, high = min(a[0], b[0]), max(a[0], b[0])
            for i in range(low, high + 1):
                vents[i, a[1]] += 1
        elif diagonals:
            (a, b, slopey) = slope(a, b)
            vents[a] += 1
            while a != b:
                a = (a[0] + 1, a[1] + slopey)
                vents[a] += 1
    return vents


def part1(lines):
    vents = rasterize_vents(lines, False)

    count = 0
    for k, v in vents.items():
        if v > 1:
            count += 1
    return count


def part2(lines):
    vents = rasterize_vents(lines, True)

    count = 0
    for k, v in vents.items():
        if v > 1:
            count += 1
    return count


def run(input: str):
    lines = list(map(parse_line, input.splitlines()))
    print(part1(lines))
    print(part2(lines))
