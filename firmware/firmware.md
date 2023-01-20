# Firmware

Document to track firmware development


## Steps to set up a new ESP32 Rust project
1. [Install](https://github.com/esp-rs/rust-build) Rust and Xtensa build tools
    - Make sure to `sudo chmod +x export-esp.sh`
2. Start a project using the [esp-idf-template](https://github.com/esp-rs/esp-idf-template) from the private repo home `dir`. I chose all the default options
```
cargo generate --vcs none --git https://github.com/esp-rs/esp-idf-template cargo
```
3. Build the `Hello World` program by running `cargo build` in the new project dir. This will take a while to build the first time:
```
cd esp32-rust
cargo build
...
Finished dev [optimized + debuginfo] target(s) in 6m 40s
```
4. Flash the ESP32 with the build artifact:
```
espflash /dev/ttyACM0 target/riscv32imc-esp-espidf/debug/remote-data-logger
```
5. Connect to ESP32 and monitor
```
espmonitor /dev/ttyACM0
```

## TODO
- `Panic` if `pin 18/19` is reconfigured.  These pins are needed to download to the ESP32-C3 per [this](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-guides/usb-serial-jtag-console.html) document


