# warpd-rs
A modal keyboard driven interface for mouse manipulation in Rust inspired from [warpd](https://github.com/rvaiya/warpd)

## Pre-requisites
You'll need to install:
- [Rust](https://www.rust-lang.org/tools/install)

Only X11 is supported for now

## Building
```shell
$ git clone https://github.com/MainKt/warpd-rs
$ cd warpd-rs
$ cargo build --release
$ ./target/release/warpd-rs --version
warpd-rs 0.1.0
```

## Usage
```
$ warpd-rs
Usage: warpd-rs <COMMAND>

Commands:
  normal  Run in normal mode
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## TODO
- [x] Grab keyboard
- [x] Implement pointer movements
- [ ] Restrict mouse hide when in normal mode
- [ ] Refactor the code
- [ ] Implement pointer button clicks
- [ ] Implement hint mode
