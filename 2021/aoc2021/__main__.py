import sys
from pathlib import Path

import aoc2021.day_01 as day_01
import aoc2021.day_02 as day_02
import aoc2021.day_03 as day_03
import aoc2021.day_04 as day_04
import aoc2021.day_05 as day_05
import aoc2021.day_06 as day_06
import aoc2021.day_07 as day_07
import aoc2021.day_08 as day_08
import aoc2021.day_09 as day_09


def read_file(filename):
    # path = Path(__file__).parent.resolve()
    with open(filename, "r") as f:
        return f.read()


def main():
    if len(sys.argv) != 3:
        print("{} DAY INPUTFILE".format(sys.argv[0]))
        exit(1)
    day = sys.argv[1]
    filename = sys.argv[2]

    content = read_file(filename)
    if day == "01":
        day_01.run(content)
    if day == "02":
        day_02.run(content)
    if day == "03":
        day_03.run(content)
    if day == "04":
        day_04.run(content)
    if day == "05":
        day_05.run(content)
    if day == "06":
        day_06.run(content)
    if day == "07":
        day_07.run(content)
    if day == "08":
        day_08.run(content)
    if day == "09":
        day_09.run(content)


if __name__ == "__main__":
    main()
