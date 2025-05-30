#!/usr/bin/env just --justfile

esp_export := "~/export-esp.sh"

list:
  just --list

# Build except firmware (except firmware)
build:
  cargo build

# Buids in release mode (except firmware)
release:
  cargo build --release

# Runs driver
run:
  cargo run --package deejx-driver --bin deejx-driver

# Builds and flashes firmware
flash:
  @cd firmware
  . {{esp_export}}
  cargo build
  espflash flash ./target/xtensa-esp32-none-elf/debug/firmware
