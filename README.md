# sysfs_input

<a href="https://docs.rs/sysfs_input"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A library for discovering and interacting with `sysfs` input devices. This library was born out of a desire to understand the mouse/touchscreen inputs of a linux system purely using it's filesystem (e.g. `/sys` ) to do cool things without X11. 

```
let inputs = sysfs_input::input_devices()?;
```

# Get Started

To simply flex out this library try

```
cargo run --example scan
```

se

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `sysfs_input` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
