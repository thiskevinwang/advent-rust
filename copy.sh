#!/usr/bin/env bash
set -euo pipefail

# cyan
info() {
  printf "\r\033[00;36m$1\033[0m\n"
}

# grey
secondary() {
  printf "\r\033[00;90m$1\033[0m\n"
}

success() {
  printf "\r\033[00;32m$1\033[0m\n"
}

fail() {
  printf "\r\033[0;31m$1\033[0m\n"
}

get_day_input() {
  read -rsp $'What day are you working on? (ctrl-c to quit):\n' -n1 key
  echo $key
}

printf "\r\033[00;36;1m
--------------------------------------------------------------------------
Advent of Code: Template
-------------------------------------------------------------------------\033[0m"
echo
secondary "This script will generate some boilerplate code."
echo

day=`get_day_input`

secondary "ℹ️  You entered: $day"

# copy dir'
cp -R src/2022/template/day{{n}}/ src/2022/day$day

# replace {{n}} with day
sed -i '' "s/{{n}}/$day/g" src/2022/day$day/mod.rs

success "✨ Generated ./src/2022/day$day"