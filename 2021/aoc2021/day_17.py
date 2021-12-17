def velxs(low, high):
    assert 0 <= low and 0 <= high

    acc = []
    for vel in range(high + 1):
        pos = 0
        vel0 = vel
        while pos <= high and vel > 0:
            pos += vel
            vel = max(0, vel - 1)
            if low <= pos and pos <= high:
                acc.append(vel0)
                break
    return acc


def velys(low, high):
    acc = []
    assert low <= 0 and high <= 0
    assert low <= high
    # If velocity is positive by more than abs(low) + 1, we can see that when
    # the probe reaches back the y = 0 line, it has an absolute speed greater
    # than abs(low) + 1, and hence would overshoot the box.
    for vel in range(-abs(low) - 1, abs(low) + 1):
        pos = 0
        vel0 = vel
        while pos >= low:
            pos += vel
            vel -= 1
            if low <= pos and pos <= high:
                acc.append(vel0)
                break
    return acc


class Simulator:
    def __init__(self, targetx, targety):
        self.targetx = targetx
        self.targety = targety

    def in_target(self, posx, posy):
        return (
            self.targetx[0] <= posx
            and posx <= self.targetx[1]
            and self.targety[0] <= posy
            and posy <= self.targety[1]
        )

    def successful(self, velx, vely):
        posx = 0
        posy = 0
        maxy = 0
        while posx <= max(self.targetx) and posy >= min(self.targety):
            posx += velx
            posy += vely
            velx = max(0, velx - 1)
            vely -= 1
            maxy = max(maxy, posy)

            if self.in_target(posx, posy):
                return maxy

        return None

    def initial_velocities(self):
        acc = []
        xs = velxs(self.targetx[0], self.targetx[1])
        ys = velys(self.targety[0], self.targety[1])
        for velx in xs:
            for vely in ys:
                res = self.successful(velx, vely)
                if res is not None:
                    acc.append((res, velx, vely))
        return acc


def run(input: str):
    input = input.removeprefix("target area: ")
    parts = input.split(", ")
    x = list(map(int, parts[0].removeprefix("x=").split("..")))
    y = list(map(int, parts[1].removeprefix("y=").split("..")))

    simulator = Simulator(x, y)
    velocities = simulator.initial_velocities()
    print(max(velocities, key=lambda x: x[0]))
    print(len(velocities))
