#! /bin/zsh

if [[ ! $1 =~ ^[0-9]+$ ]]; then
  echo "Error: first argument must be a year"
  exit 1
fi

if [ -d "y${1}" ]; then
  echo "Error: y${1} already exists"
  exit 1
fi

cargo generate -p template/ -n y${1} -d "year=${1}"

./add_day.sh "${1}" "1"
