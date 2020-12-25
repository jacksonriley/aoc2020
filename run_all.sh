#!/bin/bash
function run_release {
    for day in $@; do
        echo "Day $day:"
        ./target/release/$day
        echo "-------"
    done
}

run_release 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25