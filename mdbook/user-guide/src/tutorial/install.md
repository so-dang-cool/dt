# Installation


## Install from a package manager

Installing dt from a package manager for your operating system **will be the
preferred way of installing and updating dt in the future.**

Currently this is waiting on:

1. The release of Zig 0.11.+
2. Adoption of Zig 0.11.+ by package managers
3. Package maintainers adding support


## Getting updates

If you are not installing from a package manager (As of 2023-07, no one is!)
you won't get updates. The project intends for installations from package
managers to be the primary method of vending dt, and no independent update tool
or script is planned.

If you already have an account at GitHub, it's recommended to subscribe only to
release notifications for the GitHub project.

1. Navigate to the project at [https://github.com/so-dang-cool/dt](https://github.com/so-dang-cool/dt)
2. Find and click the "Watch" button
3. Choose "Custom" and then check only the "Releases" checkbox

It's not critical to follow every update, but the notifications can be useful
as an occasional reminder until your package manager is supported.


## Download binaries

Standalone, statically-compiled binaries of dt are available for many operating
systems and computing architectures.

For now, it's recommended to place these somewhere local like `~/.local/bin/dt`
if you normally put this on your PATH.

Binaries can be downloaded from the GitHub repository's releases page:

* [https://github.com/so-dang-cool/dt/releases](https://github.com/so-dang-cool/dt/releases)

The binares are produced in the context of github CI/CD workflows, and not
produced on random laptops. They are "deployed" as attachments to releases
automatically.


## Build from source (Vanilla)

Prerequisites:

1. [Install Zig 0.11.+](https://ziglang.org/download/)
   _(As of 2023-07, this requires the "master" release of Zig)_

Clone and build:

```
git clone https://github.com/so-dang-cool/dt.git
cd ./dt
zig build -Doptimize=ReleaseSmall
```

The resulting binary will be available in `./zig-out/bin/dt`.

(Note: there is also a `./dt` in the root of the project. This can be used
to run dt, but is intended for development reasons and is not a general
entrypoint.)


## Build from source (Nix)

Prerequisites

1. [Install][install-nix] or [upgrade to][upgrade-nix] the latest Nix release

```
git clone https://github.com/so-dang-cool/dt.git
cd ./dt
nix build
```

[install-nix]: https://nixos.org/manual/nix/unstable/installation/installation
[upgrade-nix]: https://nixos.org/manual/nix/unstable/installation/upgrading


## Build from source (rtx and crozbi)

Prerequisites

1. Install the [rtx](https://github.com/jdxcodes/rtx) version manager.
2. Install the [crozbi](https://github.com/so-dang-cool/crozbi) Zig installer.

```
rtx x zig@0.11 -- crozbi so-dang-cool/dt
```

## Usage as a flake

[![FlakeHub](https://img.shields.io/endpoint?url=https://flakehub.com/f/so-dang-cool/dt/badge)](https://flakehub.com/flake/so-dang-cool/dt)

Add dt to your `flake.nix`:

```nix
{
  inputs.dt.url = "https://flakehub.com/f/so-dang-cool/dt/*.tar.gz";

  outputs = { self, dt }: {
    # Use in your outputs
  };
}

```

## Docker

An experimental Docker container is available:

* https://hub.docker.com/r/booniepepper/dt

Pull:

```
$ docker pull booniepepper/dt
```

REPL

```
$ docker run -it booniepepper/dt ''
dt 1.x.x
Learn from my mistakes - someone should.
» 
```

Pipe

```
❯ seq 5 | docker run -i booniepepper/dt '[\n: ["*" print] n times nl] each'
*
**
***
****
*****
```


## Cross-compiling

The project's `build.zig` is pre-configured to compile for all
known-supported platforms.

With the project cloned:

```
zig build cross
```
