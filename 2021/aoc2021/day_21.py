from typing import NamedTuple
from collections import defaultdict
from typing import List, DefaultDict


class DeterministicDice:
    def __init__(self):
        self.next = 1
        self.rolls = 0

    def roll(self):
        self.rolls += 1
        n = self.next
        self.next += 1
        if self.next > 100:
            self.next -= 100
        return n

    def rolls():
        return self.rolls()


def mod10(n: int):
    n = n % 10
    if n == 0:
        return 10
    else:
        return n


class Game:
    def __init__(self, pos0, pos1, dice):
        self.pos = [pos0, pos1]
        self.score = [0, 0]
        self.dice = dice

    def turn(self, player):
        move = 0
        for _ in range(3):
            move += self.dice.roll()
        self.pos[player] = mod10(self.pos[player] + move)
        self.score[player] = self.score[player] + self.pos[player]

    def run(self):
        player = 0
        while True:
            self.turn(player)
            if self.score[player] >= 1000:
                return player
            player = 1 - player


# Part 2 completely changes tack on the structure

Element = NamedTuple(
    "Element",
    [("pos0", int), ("pos1", int), ("score0", int), ("score1", int), ("next", int)],
)


def dist():
    m = defaultdict(int)
    for d1 in [1, 2, 3]:
        for d2 in [1, 2, 3]:
            for d3 in [1, 2, 3]:
                m[d1 + d2 + d3] += 1
    return m


distribution = dist()


def next_elements(el: Element) -> DefaultDict[Element, int]:
    acc = defaultdict(int)
    for d, m in distribution.items():
        if el.next == 0:
            pos = mod10(el.pos0 + d)
            el1 = Element(pos, el.pos1, el.score0 + pos, el.score1, 1)
        else:
            pos = mod10(el.pos1 + d)
            el1 = Element(el.pos0, pos, el.score0, el.score1 + pos, 0)
        acc[el1] += m
    return acc


class QuantumGame:
    def __init__(self, pos0, pos1):
        element = Element(pos0, pos1, 0, 0, 0)
        self.counts = defaultdict()
        self.counts[element] = 1
        self.win0 = 0
        self.win1 = 0

    def step(self):
        next = defaultdict(int)
        for el, c in self.counts.items():
            for el1, m in next_elements(el).items():
                if el1.score0 >= 21:
                    self.win0 += c * m
                elif el1.score1 >= 21:
                    self.win1 += c * m
                else:
                    next[el1] += c * m
        self.counts = next

    def run(self):
        while self.counts:
            self.step()
        return max(self.win0, self.win1)


def run(_input: str):
    d = DeterministicDice()
    g = Game(5, 9, d)
    winner = g.run()
    score_loser = g.score[1 - winner]
    print(score_loser * d.rolls)

    qg = QuantumGame(5, 9)
    print(qg.run())
