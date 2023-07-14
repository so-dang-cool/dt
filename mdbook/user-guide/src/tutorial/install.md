# Installation

## Install from a package manager

Installing dt from a package manager for your operating system **will be the
preferred way of installing and updating dt in the future.**

Currently this is waiting on:

1. The release of Zig 0.11.+
2. Adoption of Zig 0.11.+ by package managers
3. Package maintainers adding support

## Download binaries

Standalone, statically-compiled binaries of dt are available for many operating
systems and computing architectures.

For now, it's recommended to place these somewhere local like `~/.local/bin/dt`
if you normally put this on your PATH.

Binaries can be downloaded from the GitHub repository's releases page:

* [https://github.com/booniepepper/dt/releases](https://github.com/booniepepper/dt/releases)

The binares are produced in the context of github CI/CD workflows, and not
produced on random laptops. They are "deployed" as attachments to releases
automatically.

Please set yourself a reminder to check for updates in a month or so! dt is
being actively developed, but is not available from package managers yet, so
you don't get automated updates.

## Building from source

Prerequisites:

1. [Install Zig 0.11.+](https://ziglang.org/download/)
   _(As of 2023-07, this requires the "master" release of Zig)_
2. If you also want to run tests, you'll need to install
   [a Rust toolchain](https://rustup.rs/).

Clone and build:

```
git clone https://github.com/booniepepper/dt.git
cd ./dt
./build release
```

The resulting binary will be available in `./zig-out/bin/dt`.

(Note: there is also a `./dt` in the root of the project. This can be used
to run dt, but is intended for development reasons and is not a general
entrypoint.)

