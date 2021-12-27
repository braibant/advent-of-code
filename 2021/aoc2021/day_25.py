class W:
    def __init__(self, width, height, down, right):
        self.width = width
        self.height = height
        self.down = down
        self.right = right

    def occupied(self, x, y):
        x = x % self.width
        y = y % self.height
        return (x, y) in self.down or (x, y) in self.right

    def move(self, s, dx, dy):
        next = set()
        moved = 0
        for (x, y) in s:
            if self.occupied(x + dx, y + dy):
                next.add((x, y))
            else:
                next.add(((x + dx) % self.width, (y + dy) % self.height))
                moved += 1
        return next, moved

    def step(self):
        right, moved_right = self.move(self.right, 1, 0)
        self.right = right
        down, moved_down = self.move(self.down, 0, 1)
        self.down = down
        return moved_right + moved_down

    def str(self):
        acc = []
        for y in range(self.height):
            line = []
            for x in range(self.width):
                pos = (x, y)
                if pos in self.right:
                    line.append(">")
                elif pos in self.down:
                    line.append("v")
                else:
                    line.append(".")
            acc.append("".join(line))
        return "\n".join(acc)


def parse(input: str):
    down = set()
    right = set()
    for y, line in enumerate(input.splitlines()):
        height = y
        for x, c in enumerate(line):
            width = x
            if c == "v":
                down.add((x, y))
            elif c == ">":
                right.add((x, y))
    return W(width + 1, height + 1, down, right)


def run(input: str):
    w = parse(input)
    steps = 0
    while True:
        steps += 1
        n = w.step()
        if n == 0:
            break
    print(steps)
