# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Runs clippy on the sources 
check:
	cargo clippy --locked -- -D warnings

# Runs all dates 
run:
	./run.sh


build:
    cargo build