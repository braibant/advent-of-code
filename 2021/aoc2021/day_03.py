from typing import List


def part1(lines: List[str]):
    n = len(lines[0])
    count = [0] * n
    for line in lines:
        for i in range(n):
            if line[i] == "1":
                count[i] += 1

    epsilon = 0
    gamma = 0
    for i in range(n):
        gamma = gamma * 2
        epsilon = epsilon * 2
        if count[i] > len(lines) // 2:
            gamma += 1
        else:
            epsilon += 1
    print("epsilon {}, gamma {}".format(epsilon, gamma))
    return epsilon * gamma


def filter(lines: List[str], op, pos) -> str:
    if len(lines) == 1:
        return lines[0]
    count = 0
    for line in lines:
        if line[pos] == "1":
            count += 1
    if count * 2 >= len(lines):
        # the most common is 1
        keep = 1
    else:
        keep = 0

    if op == "least":
        keep = 1 - keep

    if keep == 1:
        keep = "1"
    else:
        keep = "0"

    next = []
    for line in lines:
        if line[pos] == keep:
            next.append(line)
    return filter(next, op, pos + 1)


def int_of_bits(line: str):
    n = 0
    for i in range(len(line)):
        n = n * 2
        if line[i] == "1":
            n = n + 1
    return n


def part2(lines: List[str]):
    oxygen = int_of_bits(filter(lines, "most", 0))
    co2 = int_of_bits(filter(lines, "least", 0))
    print("oxygen {}, co2 {}".format(oxygen, co2))
    return co2 * oxygen


def run(input: str):
    lines = input.splitlines()
    print("{}".format(part1(lines)))
    print("{}".format(part2(lines)))
