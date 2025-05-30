#!/usr/bin/env just --justfile

esp_export := "~/export-esp.sh"

list:
  just --list

# Build project (except firmware)
build:
  cargo build

# Buid project in release mode (except firmware)
release:
  cargo build --release

# Runs driver
run:
  cargo run --package deejx-driver --bin deejx-driver

# Build and flash firmware
flash:
  @cd firmware
  . {{esp_export}}
  cargo build
  espflash flash ./target/xtensa-esp32-none-elf/debug/firmware

# Build docs and prepare for publishing
build-docs:
  padoc build ./docs @@@

# Publish docs
publish-docs:
  padoc publish ./docs @@@