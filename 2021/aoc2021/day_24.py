from typing import List, Deque
from collections import defaultdict, deque
from z3 import *


def value(state, i):
    if i in ["x", "y", "z", "w"]:
        return state[i]
    else:
        return int(i)


def accept(prog: List[str], state, input):
    pos = 0
    for i in prog:
        if i.startswith("#") or not i:
            continue
        i = i.split(" ")
        if i[0] == "inp":
            state[i[1]] = int(input[pos])
            pos += 1
        elif i[0] == "add":
            state[i[1]] = state[i[1]] + value(state, i[2])
        elif i[0] == "mul":
            state[i[1]] = state[i[1]] * value(state, i[2])
        elif i[0] == "div":
            state[i[1]] = state[i[1]] // value(state, i[2])
        elif i[0] == "mod":
            state[i[1]] = state[i[1]] % value(state, i[2])
        elif i[0] == "eql":
            state[i[1]] = int(state[i[1]] == value(state, i[2]))
    return state["z"] == 0


def state():
    return {"x": 0, "y": 0, "z": 0, "w": 0}


def eval_op(state, i, a, b, c):
    state["x"] = state["z"] % 26
    state["z"] = state["z"] // a
    state["x"] = int((state["x"] + b) != i)
    state["y"] = 25 * state["x"] + 1
    state["z"] = state["z"] * state["y"]
    state["y"] = (i + c) * state["x"]
    state["z"] = state["z"] + state["y"]


def eval(prog: str, input):
    p = prog.splitlines()
    s = state()
    print(accept(p, s, input))


# Embed the repeated program fragment, which allows to reverse engineer the
# parameters by diffing.
def f(a, b, c):
    return """inp w
mul x 0
add x z
mod x 26
div z {}
add x {}
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y {}
mul y x
add z y
""".format(
        a, b, c
    )


# w = ?
# x = z mod 26
# z = z / a
# x = (x + b) != w
# y = 25 * x + 1
# z = z * y
# y = (w + c) * x
# z = z + y

ops = [
    (1, 15, 9),
    (1, 11, 1),
    (1, 10, 11),
    (1, 12, 3),
    (26, -11, 10),
    (1, 11, 5),
    (1, 14, 0),
    (26, -6, 7),
    (1, 10, 9),
    (26, -6, 15),
    (26, -6, 4),
    (26, -16, 10),
    (26, -4, 4),
    (26, -2, 9),
]


class S:
    def __init__(self, x, y, z, w):
        self.vars = {"x": [x], "y": [y], "z": [z], "w": [w]}
        self.solver = Solver()
        # self.solver.set(priority="lex")

    def set(self, name, value):
        instance = "{}_{}".format(name, len(self.vars[name]))
        var = Int(instance)
        self.vars[name].append(var)
        self.solver.add(var == value)

    def get(self, name):
        return self.vars[name][-1]

    def z(self):
        return self.get("z")

    def add_constraints1(self, i, a, b, c):
        z = self.z()
        cond = z % 26 + b != i
        thenb = z / a * 26 + i + c
        elseb = z / a
        self.set("z", If(cond, thenb, elseb))


def solve(ranges, sols, seed):
    s0 = state()
    for i in range(len(seed)):
        eval_op(s0, seed[i], ops[i][0], ops[i][1], ops[i][2])
    s = S(s0["x"], s0["y"], s0["z"], s0["w"])
    inputs = []
    for i in range(len(seed), len(ops)):
        vi = Int("i_{}".format(i))
        inputs.append(vi)
        s.solver.add(ranges[i][0] <= vi)
        s.solver.add(vi <= ranges[i][1])
        a, b, c = ops[i]
        s.add_constraints1(vi, a, b, c)
    s.solver.add(s.get("z") == 0)
    for sol in sols:
        eq = list(map(lambda ab: ab[0] == ab[1], zip(sol[len(seed) :], inputs)))
        s.solver.add(Not(And(eq)))
    if s.solver.check() == sat:
        m = s.solver.model()
        i = seed + [m[i] for i in inputs]
        return i
    else:
        return None


def minimize():
    ranges = [(1, 9)] * 14

    for i in range(14):
        for d in range(1, 10):
            ranges[i] = [d, d]
            if solve(ranges, [], []) is not None:
                break

    s = map(str, solve(ranges, [], []))
    print("".join(s))


def maximize():
    ranges = [(1, 9)] * 14

    for i in range(14):
        for d in range(1, 10):
            ranges[i] = [d, d]
            if solve(ranges, [], []) is not None:
                s = d
        ranges[i] = [s, s]

    s = map(str, solve(ranges, [], []))
    print("".join(s))


def run(prog: str):
    maximize()
    minimize()
