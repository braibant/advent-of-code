import aoc2021.day_18 as day_18


def test_parse():
    n = day_18.parse_line("[1,1]")
    assert n.str() == "[1,1]"


def test_example():
    input = """[1,1]
[2,2]
[3,3]
[4,4]"""
    l = day_18.parse(input)
    n = day_18.addn(l)
    assert n.str() == "[[[[1,1],[2,2]],[3,3]],[4,4]]"


def test_magnitude():
    n = day_18.parse_line("[[1,2],[[3,4],5]]")
    assert n.magnitude() == 143


def test_complete():
    input = """[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"""

    l = day_18.parse(input)
    n = day_18.addn(l)
    assert n.str() == "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
    assert n.magnitude() == 4140