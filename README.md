# Remote Data Logger
Remote data logger project for use in engineering development and debugging of electronics in the field

## Feature List

### Hardware
- ESP32-C3 circuit
- 5 ADC
- 16 GPIO
- USB-C
- uSD card
- 44mm x 33mm

![pcb](./pics/pcb_layout.png)

### Firmware
- <span style="color:green">Written in Rust</span>
- <span style="color:green">Utilize [static](https://github.com/mdeloof/statig) heirarchical state machine</span>
- <span style="color:green">Multi-threaded</span>
- <span style="color:green">ADC reading</span>
- <span style="color:red">UART CLI</span>
- <span style="color:red">Store values to uSD card
- <span style="color:red">Connect to WiFi</span>
- <span style="color:red">Send message with MQTT</span>

### Software
- <span style="color:green">Graph readings from ESP32-C3 through UART</span>
    - [Serial Console Oscilloscope](https://github.com/shanemmattner/Serial_Console_Oscilloscope)


## Notes

### Git
Update submodules manually:
```
git submodule update  --init --recursive
```

Configure repo to use custom `.githooks` path and make files executable:
```
git config --local core.hooksPath .githooks/
chmod -R +x .githooks
```

## KiCAD Hotkeys
- `e` while routing:  change parameters of trace
- `b` re-pour copper fills
    - `ctrl + b` to remove all fills
- `u` select entire track
- `v` via when routing
    - switch to bottom when not routing
- `c` Design rule check panel
    - custom hotkey
