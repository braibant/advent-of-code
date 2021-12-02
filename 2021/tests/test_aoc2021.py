from aoc2021 import __version__
import aoc2021.day_02 as day_02


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
