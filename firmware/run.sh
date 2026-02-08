#!/usr/bin/env bash

cargo build --release
probe-rs run --chip STM32F103C8 ../target/thumbv7m-none-eabi/release/bootlet-switch --protocol swd
