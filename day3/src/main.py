import re
import collections
from math import prod
from pathlib import Path
import pytest


def main(_input: str):
    buckets = collections.defaultdict(list)
    chars = collections.defaultdict(list)
    num_re = re.compile(r"\d+")
    for r, line in enumerate(_input.splitlines()):
        for m in num_re.finditer(line.strip()):
            buckets[r].append(
                (int(m.group()), m.start(), m.end() - 1)
            )

    for row, line in enumerate(_input.splitlines()):
        for c, ch in enumerate(line.strip()):
            if ch in "0123456789.":
                continue
            for r in [row - 1, row, row + 1]:
                nums = buckets.get(r)
                if not nums:
                    continue
                for number, start, end in nums:
                    if max(start, c - 1) <= min(end, c + 1):
                        chars[(row, c)].append(number)

    return sum(map(sum, chars.values())), sum(map(prod, filter(lambda l: len(l) == 2, chars.values())))


@pytest.fixture
def _input():
    return """467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."""


def test_main(_input):
    assert (4361, 467835) == main(_input)


if __name__ == '__main__':
    with open(Path(__file__).parent / "input.txt") as f:
        _input = f.read()
        print(main(_input))
