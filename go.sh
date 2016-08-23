#!/usr/bin/env bash

set -e

CONTAINER="scorpil/rust:1.11"
SOURCE_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

function main {
  case "$1" in

  setup)
    setup
    ;;

  test)
    test
    ;;

  run)
    run "${@:2}"
    ;;

  *)
    help
    exit 1
    ;;

  esac
}

function help {
  echo "Usage:"
  echo " setup   downloads the prerequisites needed for other tasks (Docker must already be installed)"
  echo " test    runs the test suite"
  echo " run     compiles and runs the application"
}

function setup {
  docker pull $CONTAINER
}

function test {
  runInDockerContainer cargo test
}

function run {
  runInDockerContainer cargo run --quiet "$@"
}

function runInDockerContainer {
  docker run \
    -it --rm \
    -v $SOURCE_DIR:/work \
    -w /work \
    $CONTAINER \
    "$@"
}

main "$@"
