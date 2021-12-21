from typing import Dict


class Image:
    @classmethod
    def from_string(cls, s: str):
        pixels = set()
        for (y, line) in enumerate(s.splitlines()):
            for (x, c) in enumerate(line):
                if c == "#":
                    pixels.add((x, y))
                else:
                    assert c == "."
        return cls(pixels, False)

    def __init__(self, pixels: Dict, default: bool):
        self.pixels = pixels.copy()
        self.default = default

    def len(self):
        if not self.default:
            return len(self.pixels)

    def get(self, x: int, y: int) -> bool:
        if self.default:
            return (x, y) not in self.pixels
        else:
            return (x, y) in self.pixels

    def index(self, x: int, y: int):
        acc = 0
        for dy in [-1, 0, 1]:
            for dx in [-1, 0, 1]:
                acc = acc * 2
                if self.get(x + dx, y + dy):
                    acc += 1
        return acc

    def print(self):
        assert not self.default
        xs = list(map(lambda xy: xy[0], self.pixels))
        ys = list(map(lambda xy: xy[1], self.pixels))

        ry = range(min(ys) - 2, max(ys) + 3)
        rx = range(min(xs) - 2, max(xs) + 3)
        print(rx, ry, self.pixels)
        acc = []
        for y in ry:
            line = []
            for x in rx:
                if self.get(x, y):
                    line.append("#")
                else:
                    line.append(".")
            acc.append("".join(line))
        print("\n".join(acc))


class Mapping:
    def __init__(self, algo):
        assert len(algo) == 512
        self.algo = algo

    def enhance(self, image: Image) -> Image:
        # There is a nice joke here: the example that is given maps 9 unlit
        # pixels to 1 unlit pixels, while the spec I got maps 9 unlit pixels to
        # lit pixels, vice and versa. Oh oh oh.
        next = set()
        if self.algo[0] == "#" and self.algo[511] == ".":
            default = not image.default
        elif self.algo[0] == "." and self.algo[511] == "#":
            default = image.default
        else:
            assert False

        for x, y in image.pixels:
            for dy in [-1, 0, 1]:
                for dx in [-1, 0, 1]:
                    index = image.index(x + dx, y + dy)

                    if default:
                        if self.algo[index] == ".":
                            next.add((x + dx, y + dy))
                    else:
                        if self.algo[index] == "#":
                            next.add((x + dx, y + dy))
        return Image(next, default)


def run(input: str):
    parts = input.split("\n\n")
    mapping = Mapping(parts[0])
    print(".^9 maps to ", parts[0][0])
    print("#^9 maps to ", parts[0][511])

    i = Image.from_string(parts[1])
    for _ in range(2):
        i = mapping.enhance(i)
    print(i.len())
    for step in range(48):
        print("Step: {}".format(step + 2))
        i = mapping.enhance(i)
    print(i.len())
