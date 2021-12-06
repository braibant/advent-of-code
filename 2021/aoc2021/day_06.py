from typing import List
from collections import defaultdict


def part1(age: List[int]) -> int:
    for _ in range(80):
        for i in range(len(age)):
            if age[i] == 0:
                age[i] = 6
                age.append(8)
            else:
                age[i] -= 1
    return len(age)


def population_model(n: int, age: List[int]) -> int:
    pop = defaultdict(int)
    for i in age:
        pop[i] += 1
    for i in range(n):
        pop0 = pop[0]
        for k in range(8):
            pop[k] = pop[k + 1]
        pop[8] = pop0
        pop[6] += pop0
    return sum(pop.values())


def run(input: str):
    counters = list(map(int, input.split(",")))
    print(part1(counters.copy()))
    print(population_model(256, counters.copy()))
