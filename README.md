# swp (wip)

_Swap two paths, atomically or as close as._

This tool uses (in order of preference):

 - [x] the [`renameat2`] syscall on Linux 3.15+; (**atomic**)
 - [ ] the [`renamex_np`] function on macOS and APFS; (**atomic**)
 - [ ] the [`exchangedata`] syscall on macOS; (**atomic**)
 - [ ] the [`renameat2`] syscall on [WSL2], assuming it supports that when released; (**atomic**)
 - [ ] the [`MoveFileTransacted`] function on Windows systems that support [Transactional NTFS]; (**atomic**)
 - [ ] as first fallback, for small files, memory-mapping both files and swapping their contents;
 - [ ] as second fallback, hardlinks and renaming;
 - [ ] as third fallback, renames only.

[`renameat2`]: https://manpages.debian.org/testing/manpages-dev/renameat2.2.en.html
[`renamex_np`]: https://www.manpagez.com/man/2/renamex_np/
[`exchangedata`]: https://www.manpagez.com/man/2/exchangedata/
[WSL2]: https://devblogs.microsoft.com/commandline/announcing-wsl-2/
[`MoveFileTransacted`]: https://docs.microsoft.com/en-us/windows/desktop/api/winbase/nf-winbase-movefiletransacteda
[Transactional NTFS]: https://docs.microsoft.com/en-nz/windows/desktop/FileIO/transactional-ntfs-portal

## Install

From source:

```
cargo install swp
```

Or use [a prebuilt release](https://github.com/passcod/swp/releases).

## Usage

```bash
$ swp filea fileb

# Use -v to print out the paths on success:
$ swp filea fileb
filea <-> fileb
```

## About

Made by [Félix Saparelli](https://passcod.name).

Prior art:

 - [renameat2 tool](https://gist.github.com/eatnumber1/f97ac7dad7b1f5a9721f): linux only
 - [swap](https://crates.io/crates/swap): linux only
 - [fs-swap](https://crates.io/crates/fs-swap): platform support, errors if unsupported

Licensed under [Artistic 2.0](./LICENSE).
