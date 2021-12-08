from collections import defaultdict
from typing import DefaultDict, List


def part1(input: List[str]):
    count = 0
    for line in input:
        parts = line.split(" | ")
        output = parts[1].split(" ")
        for digit in output:
            n = len(digit)
            if n == 2 or n == 4 or n == 3 or n == 7:
                count += 1
    return count


nums = dict()
nums[0] = "abcefg"
nums[1] = "cf"
nums[2] = "acdeg"
nums[3] = "acdfg"
nums[4] = "bcdf"

nums[5] = "abdfg"
nums[6] = "abdefg"
nums[7] = "acf"
nums[8] = "abcdefg"
nums[9] = "abcdfg"


def count_chars(input):
    count = defaultdict(int)
    for display in input:
        for segment in display:
            count[segment] += 1
    return count


def expected_frequencies():
    return count_chars(nums.values())


def solve(input, freqs):
    counts = count_chars(input)
    one = next(filter(lambda s: len(s) == 2, input))
    seven = next(filter(lambda s: len(s) == 3, input))
    four = next(filter(lambda s: len(s) == 4, input))
    mapping = dict()
    if counts[one[0]] == freqs["c"]:
        mapping[one[0]] = "c"
        mapping[one[1]] = "f"
    else:
        mapping[one[0]] = "f"
        mapping[one[1]] = "c"
    a = next(x for x in seven if x not in mapping)
    mapping[a] = "a"
    b = next(x for x in four if x not in mapping and counts[x] == freqs["b"])
    mapping[b] = "b"
    d = next(x for x in four if x not in mapping and counts[x] == freqs["d"])
    mapping[d] = "d"
    # At this point, we just need to identify "e" and "g"
    e = next(k for k, v in counts.items() if v == freqs["e"] and k not in mapping)
    mapping[e] = "e"
    g = next(k for k, v in counts.items() if v == freqs["g"] and k not in mapping)
    mapping[g] = "g"
    return mapping


def decode(mapping, val: str):
    s = []
    for c in val:
        s.append(mapping[c])
    s.sort()
    n = "".join(s)
    return next(k for k, v in nums.items() if v == n)


def part2(input):
    freqs = expected_frequencies()
    total = 0
    for line in input:
        parts = line.split(" | ")
        mapping = solve(parts[0].split(" "), freqs)
        digits = list(map(lambda x: decode(mapping, x), parts[1].split(" ")))
        n = 0
        for d in digits:
            n = n * 10
            n += int(d)
        total += n
    return total


def run(input: str):
    print(part1(input.splitlines()))
    print(part2(input.splitlines()))
