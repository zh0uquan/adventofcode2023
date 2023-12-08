# Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
from pathlib import Path

import pytest


def main(_input: str):
    part1_res = 0
    part2_res = 0
    for line in _input.splitlines():
        game, list_of_cubes = line.split(":")
        game_id = int(game.lstrip("Game "))
        red, blue, green = 0, 0, 0
        for cubes in list_of_cubes.split(";"):
            for pair in cubes.split(","):
                number, color = pair.lstrip(" ").split(" ")
                number = int(number)
                match color:
                    case "red":
                        red = max(red, number)
                    case "blue":
                        blue = max(blue, number)
                    case "green":
                        green = max(green, number)
        if red <= 12 and green <= 13 and blue <= 14:
            part1_res += game_id
        part2_res += red * blue * green
    return part1_res, part2_res


@pytest.fixture
def _input():
    return """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""


def test_main(_input):
    assert (8, 2286) == main(_input)


if __name__ == '__main__':
    with open(Path(__file__).parent / "input.txt") as f:
        _input = f.read()
        print(main(_input))
