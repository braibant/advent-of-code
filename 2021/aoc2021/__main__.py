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
import aoc2021.day_10 as day_10
import aoc2021.day_11 as day_11
import aoc2021.day_12 as day_12
import aoc2021.day_13 as day_13
import aoc2021.day_14 as day_14
import aoc2021.day_15 as day_15
import aoc2021.day_16 as day_16
import aoc2021.day_17 as day_17
import aoc2021.day_18 as day_18
import aoc2021.day_19 as day_19
import aoc2021.day_20 as day_20
import aoc2021.day_21 as day_21
import aoc2021.day_22 as day_22
import aoc2021.day_23 as day_23
import aoc2021.day_24 as day_24
import aoc2021.day_25 as day_25


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
    elif day == "02":
        day_02.run(content)
    elif day == "03":
        day_03.run(content)
    elif day == "04":
        day_04.run(content)
    elif day == "05":
        day_05.run(content)
    elif day == "06":
        day_06.run(content)
    elif day == "07":
        day_07.run(content)
    elif day == "08":
        day_08.run(content)
    elif day == "09":
        day_09.run(content)
    elif day == "10":
        day_10.run(content)
    elif day == "11":
        day_11.run(content)
    elif day == "12":
        day_12.run(content)
    elif day == "13":
        day_13.run(content)
    elif day == "14":
        day_14.run(content)
    elif day == "15":
        day_15.run(content)
    elif day == "16":
        day_16.run(content)
    elif day == "17":
        day_17.run(content)
    elif day == "18":
        day_18.run(content)
    elif day == "19":
        day_19.run(content)
    elif day == "20":
        day_20.run(content)
    elif day == "21":
        day_21.run(content)
    elif day == "22":
        day_22.run(content)
    elif day == "23":
        day_23.run(content)
    elif day == "24":
        day_24.run(content)
    elif day == "25":
        day_25.run(content)


if __name__ == "__main__":
    main()
