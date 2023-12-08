from pathlib import Path


def part1(_input: str) -> int:
    res = 0
    for line in _input.splitlines():
        digits = [ch for ch in line.strip() if ch.isdigit()]
        res += int(f"{digits[0]}{digits[-1]}")
    return res


def part2(_input: str):
    res = 0
    for line in _input.splitlines():
        line = line.strip().replace("one", "one1one") \
            .replace("two", "two2two") \
            .replace("three", "three3three") \
            .replace("four", "four4four") \
            .replace("five", "five5five") \
            .replace("six", "six6six") \
            .replace("seven", "seven7seven") \
            .replace("eight", "eight8eight") \
            .replace("nine", "nine9nine")
        digits = [ch for ch in line.strip() if ch.isdigit()]
        res += int(f"{digits[0]}{digits[-1]}")
    return res


def test_part1():
    _input = """1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"""
    assert 142 == part1(_input)


def test_part2():
    _input = """two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"""
    assert 281 == part2(_input)


def main():
    with open(Path(__file__).parent / "input.txt") as f:
        _input = f.read()
        print(part1(_input))
        print(part2(_input))


if __name__ == '__main__':
    main()
