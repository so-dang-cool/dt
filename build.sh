#!/bin/bash

set -euxo pipefail

project_root="$(dirname "$0")"

compile_zig() {
    cd "$project_root"/zig-impl || exit 1
    zig build
}

cargo_test() {
    cd "$project_root" || exit 1
    cargo test
}

compile_zig && cargo_test
