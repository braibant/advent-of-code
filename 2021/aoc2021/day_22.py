from typing import Tuple, List, Set

Interval = Tuple[int, int]


def in_interval(x: int, i: Interval):
    return i[0] <= x and x <= i[1]


def interval_intersect(a, b):
    return in_interval(a[0], b) or in_interval(a[1], b)


class Cube:
    def __init__(self, rx: Interval, ry: Interval, rz: Interval):
        assert len(rx) == 2
        assert len(ry) == 2
        assert len(rz) == 2

        self.rx = rx
        self.ry = ry
        self.rz = rz

    def __eq__(self, other):
        """Overrides the default implementation"""
        if isinstance(other, Cube):
            return self.rx == other.rx and self.ry == other.ry and self.rz == other.rz
        return NotImplemented

    def __hash__(self):
        """Overrides the default implementation"""
        return hash(tuple([self.rx, self.ry, self.rz]))

    def __str__(self):
        return "x={}..{}, y={}..{}, z={}..{}".format(
            self.rx[0], self.rx[1], self.ry[0], self.ry[1], self.rz[0], self.rz[1]
        )

    def contains(self, x, y, z) -> bool:
        return all(
            [
                self.rx[0] <= x and x <= self.rx[1],
                self.ry[0] <= y and y <= self.ry[1],
                self.rz[0] <= z and z <= self.rz[1],
            ]
        )

    def intersect(self, other):
        return all(
            [
                interval_intersect(self.rx, other.rx),
                interval_intersect(self.ry, other.ry),
                interval_intersect(self.rz, other.rz),
            ]
        )

    def volume(self):
        return (
            (self.rx[1] - self.rx[0])
            * (self.ry[1] - self.ry[0])
            * (self.rz[1] - self.rz[0])
        )


def parse_range(s: str):
    parts = s.split("..")
    return (int(parts[0]), int(parts[1]))


def parse_cube(s: str) -> Cube:
    parts = s.split(",")
    rx = parse_range(parts[0][2:])
    ry = parse_range(parts[1][2:])
    rz = parse_range(parts[2][2:])
    return Cube(rx, ry, rz)


def parse_instruction(s: str) -> Tuple[bool, Cube]:
    [sign, cube] = s.split(" ")
    cube = parse_cube(cube)
    return sign == "on", cube


def status(instructions, x, y, z):
    for (sign, cube) in reversed(instructions):
        if cube.contains(x, y, z):
            return sign
    return False


def part1(instructions):
    count = 0
    reboot = Cube((-50, 50), (-50, 50), (-50, 50))
    instructions = list(filter(lambda sc: sc[1].intersect(reboot), instructions))
    for x in range(-50, 51):
        print(x)
        for y in range(-50, 51):
            for z in range(-50, 51):
                if status(instructions, x, y, z):
                    count += 1
    return count


def decompose(xs: Set[int], ys: Set[int], zs: Set[int], c: Cube) -> List[Cube]:
    xs = sorted(list(filter(lambda x: in_interval(x, c.rx), xs)))
    ys = sorted(list(filter(lambda y: in_interval(y, c.ry), ys)))
    zs = sorted(list(filter(lambda z: in_interval(z, c.rz), zs)))

    acc = []

    for ix in range(len(xs) - 1):
        for iy in range(len(ys) - 1):
            for iz in range(len(zs) - 1):
                rx = (xs[ix], xs[ix + 1])
                ry = (ys[iy], ys[iy + 1])
                rz = (zs[iz], zs[iz + 1])
                acc.append(Cube(rx, ry, rz))

    return acc


# There are on-cubes that overlap, which means that we cannot "just" keep track
# of on-cubes and substract off cubes from them. There are ~800 values in the
# problem statement for each coordinate, so, cutting the cubes that
def part2(instructions):
    cubes = list(map(lambda sc: sc[1], instructions))
    xs = set()
    ys = set()
    zs = set()
    for c in cubes:
        xs.add(c.rx[0])
        xs.add(c.rx[1])
        ys.add(c.ry[0])
        ys.add(c.ry[1])
        zs.add(c.rz[0])
        zs.add(c.rz[1])

    cubes = set()
    i = 0
    for s, c in instructions:
        i += 1
        print(i, len(cubes))
        cs = decompose(xs, ys, zs, c)
        if s:
            for c in cs:
                cubes.add(c)
        else:
            for c in cs:
                cubes.discard(c)

    return sum([c.volume() for c in cubes])


def run(input: str):
    instructions = list(map(parse_instruction, input.splitlines()))
    # print(instructions)
    # print(part1(instructions))
    part2(instructions)
