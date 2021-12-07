from typing import List


def costn(cost, positions: List[int], rdv: int) -> int:
    return sum([cost(abs(pos - rdv)) for pos in positions])


def min_cost(cost, positions: List[int]) -> int:
    low = min(positions)
    high = max(positions)
    return min([costn(cost, positions, rdv) for rdv in range(low, high + 1)])


def cost_linear(n: int) -> int:
    return n


def cost_sum(n: int) -> int:
    return (n * (n + 1)) // 2


def run(input: str):
    positions = list(map(int, input.split(",")))
    print(min_cost(cost_linear, positions))
    print(min_cost(cost_sum, positions))
