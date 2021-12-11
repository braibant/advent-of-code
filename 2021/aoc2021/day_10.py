from typing import List


matching = {"(": ")", "{": "}", "[": "]", "<": ">"}
score1 = {")": 3, "]": 57, "}": 1197, ">": 25137}
score2 = {"(": 1, "[": 2, "{": 3, "<": 4}


def corrupted(line: str) -> int:
    stack = []
    for c in line:
        if c == "(" or c == "[" or c == "{" or c == "<":
            stack.append(c)
        elif c == ")" or c == "]" or c == "}" or c == ">":
            t = stack.pop()
            if matching[t] != c:
                return score1[c]
        else:
            raise ValueError
    return 0


def incomplete(line: str):
    stack = []
    for c in line:
        if c == "(" or c == "[" or c == "{" or c == "<":
            stack.append(c)
        elif c == ")" or c == "]" or c == "}" or c == ">":
            t = stack.pop()
            if matching[t] != c:
                # corrupted
                return None
        else:
            raise ValueError
    # completion process
    score = 0
    while stack:
        score = score * 5
        c = stack.pop()
        score += score2[c]
    return score


def part1(lines: List[str]):
    return sum([corrupted(line) for line in lines])


def part2(lines: List[str]):
    scores = sorted(list(filter(lambda x: x is not None, map(incomplete, lines))))
    return scores[len(scores) // 2]


def run(input: str):
    lines = input.splitlines()
    print(part1(lines))
    print(part2(lines))
