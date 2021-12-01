def part1(lines):
    count = 0
    for i in range(len(lines) - 1):
        if lines[i + 1] > lines[i]:
            count += 1
    return count


def part2(lines):
    count = 0
    for i in range(len(lines) - 3):

        if (
            lines[i + 1] + lines[i + 2] + lines[i + 3]
            > lines[i] + lines[i + 1] + lines[i + 2]
        ):
            count += 1
    return count


def run(input: str):
    lines = input.splitlines()
    lines = list(map(int, lines))
    print("{}".format(part1(lines)))
    print("{}".format(part2(lines)))
