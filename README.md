<div align="center">
  <h1>microbit-demos</h1>
  <p>
    <strong>
      Rust demos running on the <a href="https://microbit.org/">BBC micro:bit</a>
    </strong>
  </p>
</div>

## Demos

- `leds` - Blinking LED pattern
- `serial_buttons` - Sends button state over serial
- `serial_buttons_interrupt` - Sends button state over serial, interrupts on button press/release
- `serial_echo` - Echos any serial data received
- `serial_random` - Generates random bytes and sent over serial
- `snake` - Classic snake game

https://user-images.githubusercontent.com/1206028/189038516-e71f4bc1-162b-4220-af4e-dbe14c5e3f00.mp4

## Dependencies

- `rustup target add thumbv6m-none-eabi`
- `arm-none-eabi-gdb` - GDB debugger for Arm
- `openocd` - Open On-Chip Debugger
- `picocom` or `minicom` - Terminal emulator

The [Installation](https://rust-embedded.github.io/book/intro/install.html)
guide in [The Embedded Rust Book](https://rust-embedded.github.io/book/intro/index.html)
has tips to install some of these tools on different platforms.

## Flashing and debugging demos

### Flashing

Connect BBC micro:bit via USB and flash a given demo.

```sh
bin/flash demo_name
```

The micro:bit will automatically reboot and start running the demo.

### Serial console

Some demos communicate over serial. For these demos start a console session in a
separate terminal window. The provided shell script expects the USB console
device at `/dev/ttyACM0`, update this if required. On macOS this may be something like `/dev/cu.usbmodem14412`.

```sh
bin/console
```

### Debugging

To debug a program on the micro:bit first start the debug server.

```sh
bin/debug_server
```

Start a GDB session for a given demo.

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
