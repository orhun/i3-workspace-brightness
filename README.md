# i3-workspace-brightness

[![Release](https://img.shields.io/github/release/orhun/i3-workspace-brightness.svg?color=2B7DBC&style=flat-square)](https://github.com/orhun/i3-workspace-brightness/releases) [![AUR](https://img.shields.io/aur/version/i3-workspace-brightness.svg?color=2B7DBC&style=flat-square)](https://aur.archlinux.org/packages/i3-workspace-brightness/) [![crates.io](https://img.shields.io/crates/v/i3-workspace-brightness?color=2B7DBC&style=flat-square)](https://crates.io/crates/i3-workspace-brightness/)

## About

Rust port of [this](https://gist.github.com/vaibhav93/20500065786327e55c2f438a3922a7ae) Python script with a few improvements.

It allows individual brightness levels for each workspace and changes the brightness automatically while switching between them.

## Installation

### Cargo

```
cargo install i3-workspace-brightness
```

### AUR

```
yay -S i3-workspace-brightness
```

## Configuration

Add this line to your i3 config:

```
exec_always --no-startup-id i3-workspace-brightness
```

### Replacing xbacklight commands

Use `GET_BRIGHTNESS` and `SET_BRIGHTNESS` environment variables for the command to use. For example:

```
export GET_BRIGHTNESS="light"
export SET_BRIGHTNESS="light -S {}"
i3-workspace-brightness
```

or set the variables while running `i3-workspace-brightness`:

```
GET_BRIGHTNESS="light" SET_BRIGHTNESS="light -S {}" i3-workspace-brightness
```

## License

GNU General Public License ([v3](https://www.gnu.org/licenses/gpl.txt))

## Copyright

Copyright © 2020-2023, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
