from typing import List


class Runner:
    def __init__(self, state):
        self.state = state

    def run(self, input: List[str]) -> int:
        for instr in input:
            if instr.startswith("forward"):
                n = int(instr.removeprefix("forward "))
                self.state.forward(n)
            if instr.startswith("down"):
                n = int(instr.removeprefix("down "))
                self.state.down(n)
            if instr.startswith("up"):
                n = int(instr.removeprefix("up "))
                self.state.up(n)


def part1(input: List[str]) -> int:
    class State:
        def __init__(self):
            self.horizontal = 0
            self.depth = 0

        def forward(self, n):
            self.horizontal += n

        def down(self, n):
            self.depth += n

        def up(self, n):
            self.depth -= n

    runner = Runner(State())
    runner.run(input)
    return runner.state.horizontal * runner.state.depth


def part2(input: List[str]) -> int:
    class State:
        def __init__(self):
            self.aim = 0
            self.horizontal = 0
            self.depth = 0

        def forward(self, n):
            self.horizontal += n
            self.depth += self.aim * n

        def down(self, n):
            self.aim += n

        def up(self, n):
            self.aim -= n

    runner = Runner(State())
    runner.run(input)
    return runner.state.horizontal * runner.state.depth


def run(input: str):
    lines = input.splitlines()
    print(part1(lines))
    print(part2(lines))
