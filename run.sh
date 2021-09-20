#!/bin/bash

set -euf -o pipefail

rm -f output

for i in $(seq -f "%02g" 01 25)
do
    if [ -f "src/day_$i.rs" ]; then
        (echo "# $i" && cargo run $i data/day_$i.txt && echo "") >> output
    fi 
done
