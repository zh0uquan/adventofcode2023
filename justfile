create day:
    cargo generate --path ./daily-template --name {{day}}
    just get-input {{day}}

get-input day:
    ./scripts/get_aoc_input.py --day {{day}} --current-working-directory {{justfile_directory()}}

set dotenv-load := true