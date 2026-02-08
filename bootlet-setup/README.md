# Bootlet Setup for BootletSwitch

Bootlet requires a specific setup to function correctly with BootletSwitch. Follow the instructions below to ensure proper configuration.

## Prerequisites
- `make`
- Rust with `x86_64-unknown-linux-musl` target installed

## Setup Instructions
Run
```bash
make
```

This command will produce `bootlet.img`. You can write this image to the EFI partition on your device.
