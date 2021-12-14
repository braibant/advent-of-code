from collections import defaultdict


class Rules:
    def __init__(self, rules):
        self.rw = dict()
        for rule in rules:
            rule = rule.split(" -> ")
            self.rw[rule[0]] = rule[1]


class StringBased:
    def __init__(self, s):
        self.content = s

    def freqs(self):
        f = defaultdict(int)
        for c in self.content:
            f[c] += 1
        return f

    def next(self, rules):
        next = []
        for p in range(2 * len(self.content) - 1):
            if p % 2 == 0:
                next.append(self.content[p // 2])
            else:
                lhs = "{}{}".format(self.content[p // 2], self.content[p // 2 + 1])
                rhs = rules.rw[lhs]
                next.append(rhs)
        return StringBased("".join(next))


class PairsBased:
    def __init__(self, content, start, end):
        self.content = content
        self.start = start
        self.end = end

    def from_string(s):
        npairs = defaultdict(int)
        for i in range(len(s) - 1):
            npairs[s[i : i + 2]] += 1
        return PairsBased(npairs, s[0], s[-1])

    def freqs(self):
        f = defaultdict(int)
        for p, q in self.content.items():
            f[p[0]] += q
            f[p[1]] += q
        f[self.start] += 1
        f[self.end] += 1
        for k in f.keys():
            f[k] = f[k] // 2
        return f

    def next(self, rules):
        n = defaultdict(int)
        for p, q in self.content.items():
            s = [p[0], rules.rw[p], p[1]]
            n["".join(s[0:2])] += q
            n["".join(s[1:])] += q
        return PairsBased(n, self.start, self.end)


def quantities(obj):
    freq = obj.freqs()
    most_common = max(freq.items(), key=lambda kv: kv[1])[1]
    least_common = min(freq.items(), key=lambda kv: kv[1])[1]
    return most_common - least_common


def same_freqs(a, b):
    fa = a.freqs()
    fb = b.freqs()

    keys = set(list(fa.keys()) + list(fb.keys()))
    for key in list(keys):
        if fa[key] != fb[key]:
            print("Key: {}, Left: {}, Right: {}".format(key, fa[key], fb[key]))
            print(fa)
            print(fb)
            return False
    return True


def part1(rules, template):
    polymer0 = StringBased(template)
    polymer1 = PairsBased.from_string(template)
    assert same_freqs(polymer0, polymer1)
    for i in range(10):
        assert same_freqs(polymer0, polymer1)
        print(i)
        polymer0 = polymer0.next(rules)
        polymer1 = polymer1.next(rules)
        assert same_freqs(polymer0, polymer1)
    return quantities(polymer0)


def part2(rules, template):
    polymer = PairsBased.from_string(template)
    for i in range(40):
        polymer = polymer.next(rules)
    return quantities(polymer)


def run(input):
    lines = input.splitlines()
    template = lines[0]
    rules = Rules(lines[2:])
    print(part1(rules, template))
    print(part2(rules, template))
