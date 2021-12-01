# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Runs clippy on the sources 
check:
	cargo clippy --locked -- -D warnings

# Runs all dates 
all:
	./run.sh

# Run a single date
run target:
	cargo run {{target}} data/day_{{target}}.txt


build:
    cargo build