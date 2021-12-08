import aoc2021.day_08 as day_08


def test_part2():
    freqs = day_08.expected_frequencies()
    sample = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab".split(" ")
    mapping = day_08.solve(sample, freqs)
    s = []
    for x in "abcdefg":
        s.append(next(k for k, v in mapping.items() if v == x))
    assert "".join(s) == "deafgbc"
