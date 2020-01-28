# Piano Sensor Firmware

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## Building

Build the application

``` console
$ cargo build
```


## Flashing

using Cargo:

```
cargo run
```

See .cargo/config for `runner` command.


or using Bobbin:

```
bobbin load --bin firmware
```

Or using vscode: Run the debugger > `Build&Launch`

## Run with semihosting

```
cargo run --features "use_semihosting"
```

