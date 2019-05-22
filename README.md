# swap

**Linux only, from kernel 3.15+.**

_Atomically swap two paths._

This tool uses the [`renameat2`](https://manpages.debian.org/testing/manpages-dev/renameat2.2.en.html) syscall to swap two paths in one single command.

## Install

From source:

```
cargo install swap
```

Or use [a prebuilt release](https://github.com/passcod/swap/releases).

## Usage

```bash
$ swap filea fileb

# Use -v to print out the paths on success:
$ swap filea fileb
filea <-> fileb
```

## About

Made by [Félix Saparelli](https://passcod.name).

Licensed under [Artistic 2.0](./LICENSE).
