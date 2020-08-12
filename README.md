# `wio-terminal`

> A Board Support Package (BSP) which provides a type-safe API for the Seeed Studio [Wio Terminal](https://www.seeedstudio.com/Wio-Terminal-p-4509.html).

[![Build Status](https://travis-ci.org/jessebraham/wio-terminal.svg?branch=master)](https://travis-ci.org/jessebraham/wio-terminal)
[![Crates.io](https://img.shields.io/crates/v/wio-terminal.svg)](https://crates.io/crates/wio-terminal)
[![Docs](https://docs.rs/wio-terminal/badge.svg)](https://docs.rs/wio-terminal/)
![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue)

This project is made possible thanks to the following crates:

* [atsamd-hal](https://github.com/atsamd-rs/atsamd)
* [embedded-graphics](https://github.com/jamwaffles/embedded-graphics)
* [ili9341-rs](https://github.com/yuri91/ili9341-rs)
* [lis3dh-rs](https://github.com/BenBergman/lis3dh-rs)

## [Documentation]

[Documentation]: https://docs.rs/wio-terminal/

## Resources

* [Wio Terminal product page](https://www.seeedstudio.com/Wio-Terminal-p-4509.html)
* [Wio Terminal wiki](https://wiki.seeedstudio.com/Wio-Terminal-Light/)
* [Wio Terminal user manual](https://files.seeedstudio.com/wiki/Wio-Terminal/res/Wio-Terminal-User-Manual.pdf)

## Examples

To build the examples:

```bash
$ git clone https://github.com/jessebraham/wio-terminal.git
$ cd wio-terminal/
$ # you can build all the examples at once:
$ cargo build --examples
$ # ...or specify one at a time:
$ cargo build --example=blinky
```

__TODO:__ write instructions for flashing firmware to the device

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
