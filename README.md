# sysfs_input

A library for discovering and interacting with `sysfs` input devices. This library was born out of a desire to understand the mouse/touchscreen inputs of a linux system purely using it's filesystem (e.g. `/sys` ) to do cool things without X11. 

```
let inputs = sysfs_input::input_devices()?;
```

# Get Started

To simply flex out this library try

```
cargo run --example scan
```
