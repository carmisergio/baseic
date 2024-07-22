# baseic

[![Hits-of-Code](https://hitsofcode.com/github/carmisergio/baseic?branch=master)](https://hitsofcode.com/github/carmisergio/baseic/view?branch=master)
![Test with cargo](https://github.com/carmisergio/baseic/actions/workflows/test.yml/badge.svg)

`baseic` is a simple value conversion tool written for minimum interruption. As of now, it provides conversion to and from only numeric bases, but expect to see ASCII characters, signed formats and more in the future.

## Installation

### From source

`baseic` has been developed with Linux users in mind, but there is no reason it should not work perfectly on MacOS and other Unix-like systems as well.

Obtain the source code either by cloning this repository,

```bash
git clone https://github.com/carmisergio/baseic.git
```

or downloading and extracting the source code from one of the releases.

`baseic` is written in the [Rust Programming Language](https://www.rust-lang.org/), and can be built using `cargo`.

Once inside the directory, run the following command to download all dependencies and build the `baseic` binary.

```bash
cargo build --release
```

You can now run `baseic` as `target/release/baseic`, and place this binary wherever you desire.

You can also use `cargo` to install baseic to `~/.cargo/bin` by running `cargo install --path .`

### From binary release

We only provide binary executables for Linux x64 at the moment.

Simply download an executable binary from the releases page and run it.

### From `crates.io`

If you already have `cargo` on your system, you can install `baseic` from the `crates.io` repository by running

```bash
cargo install baseic
```

This will place the compiled binary in `~/.cargo/bin`

<!-- ### From your system's package manager
#### Arch Linux
`baseic` can be installed from the [AUR](https://aur.archlinux.org/) -->

## Usage

```
Usage: baseic [-h] [<input converter>] <value> [<output converters>]

Options:
  -h: display this message

Input converters:
  DEC: decimal
  BIN: binary
  HEX: hexadecimal
  OCT: octal
  ASCII: ascii character

Output converters:
  DEC: decimal
  BIN: binary
  HEX: hexadecimal
  OCT: octal
  ASCII: ascii character

Example: baseic dec 1234 bin hex
```

`baseic` is designed to be as quick as possible to use.

For example, to see all possible representations of the number 42, run

```bash
$ baseic 42
from decimal:
  hexadecimal: 2A
       binary: 101010
        octal: 52
from hexadecimal:
      decimal: 66
       binary: 1000010
        octal: 102
from octal:
      decimal: 34
  hexadecimal: 22
       binary: 100010
```

As you can see, `baseic` has correctly identified that 42 is valid in decimal, hexadecimal, and octal, and then provided the respective representations in all formats except the input.

If you want to specify the input format, you can do it by adding its name before the value, eg.

```
baseic hex 42
```

will only treat 42 as a hexadecimal number.

Similarly, if you want to get the output in one or more specific formats, you can specify their names after the value to be converted, eg.

```
baseic 42 dec bin ascii
```

This will convert 42 from all input formats for which it is valid to decimal, binary, and ascii character.

## Configuration

`baseic` stores its configuration file in `~/.config/baseic/config.toml` in [TOML](https://toml.io/en/) format.

When no configuration file is found, or when some configuration keys are not present in the file, the default values are used.

```toml
# baseic base conversion tool config file

# Set output converters to be executed when none are specified
# Allowed values: "DEC", "BIN", "HEX", "OCT"
# default_outconvs = ["DEC", "BIN", "HEX", "OCT"]

# Set input converters to be executed when not specified
# Allowed values: "DEC", "BIN", "HEX", "OCT"
# default_inconvs = ["DEC", "BIN", "HEX", "OCT"]
```

To set a configuration key, simply uncomment the desired line and modify its value.
