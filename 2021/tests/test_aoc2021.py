from aoc2021 import __version__
import aoc2021.day_02 as day_02
import aoc2021.day_04 as day_04
import aoc2021.day_06 as day_06
import aoc2021.day_07 as day_07


def test_version():
    assert __version__ == "0.1.0"


def test_day_02_part1():
    sample = """forward 5
down 5
forward 8
up 3
down 8
forward 2
"""
    assert day_02.part1(sample.splitlines()) == 150


def test_day_02_part2():
    sample = """forward 5
down 5
forward 8
up 3
down 8
forward 2
"""
    assert day_02.part2(sample.splitlines()) == 900


def test_day_04_part1():
    sample = """7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"""

    (nums, grids) = day_04.parse(sample)
    assert day_04.part1(nums, grids) == 4512


def test_day_06_part1():
    sample = list(map(int, "3,4,3,1,2".split(",")))
    assert day_06.part1(sample) == 5934


def test_day_07_part2():
    positions = list(map(int, "16,1,2,0,4,2,7,1,2,14".split(",")))
    assert day_07.cost_sum(16 - 5) == 66
    assert day_07.cost_sum(14 - 5) == 45
    assert day_07.costn(day_07.cost_sum, positions, 5) == 168
    assert day_07.min_cost(day_07.cost_sum, positions) == 168
