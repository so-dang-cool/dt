#!/bin/bash

set -euxo pipefail

cd "$(dirname "$0")" || exit 1
project_root="$(pwd)"

compile_zig() {
    cd "$project_root" || exit 1
    zig build
}

cargo_test() {
    cd "$project_root"/old-rust-tests || exit 1
    cargo test
}

compile_zig && cargo_test
