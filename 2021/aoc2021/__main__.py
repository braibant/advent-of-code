import sys
from pathlib import Path

import aoc2021.day_01 as day_01


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


if __name__ == "__main__":
    main()
