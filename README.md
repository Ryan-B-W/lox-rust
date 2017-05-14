# Lox

Utility to work with shell histories. Rewrite of [this](https://github.com/davidnuon/lox) in Rust.

### Installing

Right now, `lox` is only available as source. To install, do the following

```
git clone https://github.com/davidnuon/lox-rust.git
cd lox-rust
cargo build
cargo install
```

## Usage

```
$ lox -h

Lox 0.1.0
David Nuon <david@davidnuon.com>

USAGE:
    lox [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -n               Display index next to command
    -t               Display timestamp next to command
    -V, --version    Prints version information
```

## Compatibility
The only supported shells are `fish` and `bash`

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
