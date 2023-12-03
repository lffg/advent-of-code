#!/usr/bin/env bash

set -euo pipefail

last=$(
    find . -type d -maxdepth 1 -name 'day*' |\
    sort -r |\
    head -n 1 |\
    sed -n -e 's/^\.\/day\([[:digit:]]\)/\1/p'
)

today_num="$(printf "%02d" "$((last + 1))")"
today_dir="day$today_num"

echo "Creating directory '$today_dir'"
mkdir "$today_dir"

sed -e "s/{name}/$today_dir/g" template/Cargo.toml >> "$today_dir/Cargo.toml"
cp template/{input.txt,main.rs} "$today_dir"

echo "ok"
