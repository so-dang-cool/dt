#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail

bats=./tests/bats/bin/bats

target="${1:-unknown}"
case "$target" in
  test) $bats test.bats ;;
  *)    echo "Unimplemented target: $target"
        exit 1 ;;
esac