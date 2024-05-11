# comet
A modal keyboard driven interface for mouse manipulation in Rust inspired from [warpd](https://github.com/rvaiya/warpd)

## Pre-requisites
You'll need to install:
- [Rust](https://www.rust-lang.org/tools/install)

Only X11 is supported for now

## Building
```shell
$ git clone https://github.com/MainKt/comet
$ cd comet
$ cargo build --release
$ ./target/release/comet --version
comet 0.1.0
```

## Usage
```
$ comet
Usage: comet <COMMAND>

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
- [x] Implement pointer button clicks
- [ ] Refactor the code
- [ ] Implement hint mode
