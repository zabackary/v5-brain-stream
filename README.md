# V5 Brain Stream

V5 Brain Stream streams the VEX V5 Brain screen to a fixed-size desktop
application by continually screenshotting the interface.

It uses the `vex-v5-serial` crate to communicate with the VEX V5 Brain and the
`iced` crate for the GUI. It is intended to be used in video production
environments where the VEX V5 Brain screen needs to be displayed on a broadcast
or recording.

## Installation

To install the V5 Brain Stream application, you need to have Rust and Cargo
installed. You can install Rust and Cargo by following the instructions on the
[official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can install the binary as follows:

```sh
cargo install v5-brain-stream
```

This will build from source the `v5-brain-stream` binary on your system, which
you can then run from the command line.

## Relationship to vex-v5-serial

V5 Brain Stream uses the `vex-v5-serial` crate to communicate with the VEX V5
Brain. The core logic of this application is similar to `cargo-v5`'s
`cargo v5 screenshot` command.

Thank you vexide contributors for making this possible.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file
for details.
