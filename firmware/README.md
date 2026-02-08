# Bootlet-Switch Firmware

The firmware is designed for STM32F103C8T6, commonly known as "Blue Pill" board.

## Setup
### Requirements
#### Hardware
- STM32F103C8 (Blue Pill) board
- Toggle switch (SPST)
- ST-Link V2 programmer
- USB cable that connects directly to the Blue Pill board (May vary based on the board version: Micro USB or USB-C)
#### Software
- `rustup` - for managing Rust toolchains
- `probe-rs-cli` - for flashing the firmware

### Wiring
![Wiring Diagram](./assets/wiring.png)

Wiring is super simple. Connect one terminal of the toggle switch to pin **PA0**(often labeled **A0**) on the Blue Pill board. Connect the other terminal to the ground (often labeled as **GND** or **G**).

> [!NOTE]
> The Blue Pill board has one **PA0** and multiple **GND** pins. You can use any of the **GND** pins available on the board.

### Flashing
#### Connecting ST-Link 
Connect the ST-Link V2 programmer to the Blue Pill board as follows:
| ST-Link V2 Pin | Blue Pill Pin | Blue Pill Pin Location |
|----------------|----------------|-----------------------|
| VCC            | 3.3V           | Short side of the board  |
| GND            | GND            | Short side of the board  |
| SWDIO          | SWDIO (or SWO) | Short side of the board  |
| SWCLK          | SWCLK          | Short side of the board  |
| NRST           | NRST (or R)    | Long side of the board   |

> [!NOTE]
> You have to manually connect the pins using jumper wires. You can search online for "ST-Link V2 to Blue Pill wiring" for reference images.

#### Building and Flashing the Firmware
Assuming your console is in the `bootlet-switch/firmware` directory, follow these steps:
1. Build the firmware using Cargo:
   ```bash
   cargo build --release
   ```
2. Flash the firmware to the Blue Pill board using `probe-rs-cli`:
   ```bash
   probe-rs run --chip STM32F103C8 ../target/thumbv7m-none-eabi/release/bootlet-switch --protocol swd
   ```
3. If the flashing goes normally, you should see two progress bars called **Erasing** and **Programming**. After that, you should see a message including this: `[INFO ] Starting bootlet-switch`. If you see that message, press `Ctrl+C` to exit.
4. Congratulations! You have successfully flashed the firmware to your Blue Pill board.
5. Disconnect the ST-Link V2 programmer from the Blue Pill board.

> [!NOTE]
> Note that the `target` directory is created in the root of the project, not in the `firmware` directory, which you are in. Documented commands are adjusted to reflect that, so you don't have to change directories.

#### Checking the Functionality
1. Connect the Blue Pill board to your computer using a USB cable.
2. Go to `bootlet-switch-lib` directory in the project root.
3. Run the following command to check the switch state:
    ```bash
    cargo run
    ```
4. Tick the switch and run the command again. You should see the state change from `true` to `false` or vice versa.
5. If the state changes correctly, congratulations! Bootlet-Switch is working as expected.
