#! /bin/zsh

echo "adding day ${2} to year ${1}"

if [[ ! $1 =~ ^[0-9]+$ ]]; then
  echo "Error: first argument must be a year"
  exit 1
fi

if [[ ! $2 =~ ^[0-9]+$ ]]; then
  echo "Error: second argument must be a day"
  exit 1
fi

BASE="y${1}"

# exit if base does not exists
if [ ! -d "${BASE}" ]; then
  echo "${BASE} does not exist"
  exit 1
fi

INPUT_FILE="${BASE}/input/day${2}.txt"
RUST_FILE="${BASE}/src/day${2}.rs"

if [ -f "${INPUT_FILE}" ]; then
  echo "${INPUT_FILE} already exists"
  exit 1
fi

if [ -f "${RUST_FILE}" ]; then
  echo "${RUST_FILE} already exists"
  exit 1
fi

echo "creating input file ${INPUT_FILE}"
touch "${INPUT_FILE}"

echo "creating rust file ${RUST_FILE}"
touch "${RUST_FILE}"

cp "template/src/day.tmpl" "${RUST_FILE}"

# replce all ${{day}} with ${2}
if [[ "$OSTYPE" == "darwin"* ]]; then
  sed -i "" "s/\${{day}}/${2}/g" "${RUST_FILE}"
else
  sed -i "s/\${{day}}/${2}/g" "${RUST_FILE}"
fi

# append mod day${2}; to lib.rs
echo "mod day${2};" >>"${BASE}/src/lib.rs"
