from math import prod

decode = {
    "0": "0000",
    "1": "0001",
    "2": "0010",
    "3": "0011",
    "4": "0100",
    "5": "0101",
    "6": "0110",
    "7": "0111",
    "8": "1000",
    "9": "1001",
    "A": "1010",
    "B": "1011",
    "C": "1100",
    "D": "1101",
    "E": "1110",
    "F": "1111",
}


class Packet:
    def __init__(self, version, type):
        self.version = version
        self.type = type
        self.packets = []
        self.literal = None

    def append(self, packet):
        self.packets.append(packet)

    def sumup_versions(self):
        n = self.version
        for packet in self.packets:
            n += packet.sumup_versions()
        return n

    def set_literal(self, literal):
        self.literal = literal

    def eval(self):
        if self.type == 4:
            return self.literal
        else:
            packets = list(map(lambda p: p.eval(), self.packets))
            if self.type == 0:
                return sum(packets)
            elif self.type == 1:
                return prod(packets)
            elif self.type == 2:
                return min(packets)
            elif self.type == 3:
                return max(packets)
            elif self.type == 5:
                return int(packets[0] > packets[1])
            elif self.type == 6:
                return int(packets[0] < packets[1])
            elif self.type == 7:
                return int(packets[0] == packets[1])


class Bits:
    def __init__(self, hex):
        content = []
        for c in hex:
            content.append(decode[c])
        self.content = "".join(content)

    def read_int(self, pos, len):
        n = 0
        for i in range(len):
            n = n * 2
            n += int(self.content[pos + i])
        return n, pos + len

    def read_literal(self, pos):
        n = 0
        while True:
            pos0 = pos
            n = n * 16
            m, pos = self.read_int(pos + 1, 4)
            n += m
            if self.content[pos0] == "0":
                return n, pos


# Parse a packet starting at pos
def parse(bits, pos):
    version, pos = bits.read_int(pos, 3)
    ty, pos = bits.read_int(pos, 3)
    packet = Packet(version, ty)
    if ty == 4:
        # Literal
        literal, pos = bits.read_literal(pos)
        packet.set_literal(literal)
    else:
        length_ty, pos = bits.read_int(pos, 1)
        if length_ty == 0:
            length, pos = bits.read_int(pos, 15)
            until = length + pos
            while pos < until:
                subpacket, pos = parse(bits, pos)
                packet.append(subpacket)
        else:
            num_packets, pos = bits.read_int(pos, 11)
            for i in range(num_packets):
                subpacket, pos = parse(bits, pos)
                packet.append(subpacket)
    return packet, pos


def run(input: str):
    bits = Bits(input)
    packet, _ = parse(bits, 0)
    print(packet.sumup_versions())
    print(packet.eval())
