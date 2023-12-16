#!/Users/quan/.pyenv/versions/script/bin/python3
import os
from pathlib import Path

import requests
import argparse

def download_aoc_input():
    parser = argparse.ArgumentParser()
    parser.add_argument("--day")
    parser.add_argument("--current-working-directory")
    args = parser.parse_args()

    day = str(args.day)
    current_working_directory = args.current_working_directory
    session = os.environ.get("SESSION")
    num = "".join(n for n in day if n.isdigit())
    response = requests.get(
        f"https://adventofcode.com/2022/day/{num}/input",
        cookies={
            "session": session
        }
    )
    print(args.current_working_directory)
    # print(response.text)
    curr_path = Path(current_working_directory)
    with open(curr_path / day / "src" / "input.txt", "w+") as f:
        f.write(response.text)


if __name__ == '__main__':
    download_aoc_input()