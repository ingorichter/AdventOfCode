#!/bin/bash
set -euo pipefail

output_path="$(printf './Sources/AOC2021/Day %d/Day%d.txt' "${1:?gimme a day}" "${1:?gimme a day}")"
year=2021
cookie='session=53616c7465645f5f972ab5c84cc0f4c39f32b0209200db2fba95394dd65a92b46344976053a472af6e5b649933af5f5d'  # set this from the login session

[ -f "${output_path}" ] && {
    echo already loaded
    exit
} >&2

curl --fail -sS -b "${cookie}" "https://adventofcode.com/${year}/day/$1/input" -o "${output_path}"
