# Adafruit Feather RP2040 RFM95 Quickstart
This repo contains a barebones template for writing Rust firmware for the [Adafruit Feather RP2040 RFM95 board](https://www.adafruit.com/product/5714).

## Features
* [Embassy](https://embassy.dev/)
* USB logging (the board doesn't expose the SWD pins on the RP2040)

## Initial Setup
1. `$ cargo generate --git https://github.com/ardentTech/adafruit-feather-rp2040-rfm95-quickstart.git`
2. Set up a serial port communication program on your host (e.g. [minicom](https://github.com/Distrotech/minicom))

## Commands
* Build: `$ cargo build --release`
* Flash: `$ cargo run --release`

