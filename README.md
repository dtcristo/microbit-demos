# microbit-demos

Rust demos running on the BBC micro:bit.

## Dependencies

- BBC micro:bit device
- `thumbv6m-none-eabi` target for Rust
- `arm-none-eabi` and `arm-none-eabi-gdb` - Arm cross-compiler toolchain
- `openocd` - Open On-Chip Debugger
- `picocom` or `minicom` - Terminal emulator

Follow

## Flashing and debugging demos

### Flashing

Connect BBC micro:bit via USB and flash a given demo. The micro:bit will
automatically reboot and start running the demo.

```sh
bin/flash demo_name
```

### Serial console

Some demos communicate over serial. For these demos start a console session in a
separate terminal window. The provided shell script expects the serial device at
`/dev/ttyACM0`.

```sh
bin/console
```

### Debugging

To debug a program on the micro:bit first start the debug server.

```sh
bin/debug_server
```

Start a debug session for a given demo.

```sh
bin/debug demo_name
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
