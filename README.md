# Deejx
deejx is an **open-source hardware volume mixer** and **simple midi controller** for Windows and Linux PCs.
Use real-life sliders or knobs (like a DJ!)
to seamlessly control the volumes of different apps (such as your music player, the game you're playing and your voice chat session) without having to stop what you're doing.

deejx is a alternative spin of [original deej](https://github.com/omriharel/deej) project.

> [!WARNING]
> Project is in early stages. Documentation and some features are not available yet. 

## Features
Core features:
- Bind apps to different sliders
    - Bind multiple apps per slider (i.e. one slider for all your games)
    - Bind the master channel
    - ~~Bind "system sounds" (on Windows)~~ *(work in progress)*
    - ~~Bind specific audio devices by name (on Windows)~~ *(work in progress)*
    - ~~Bind currently active app (on Windows)~~ *(work in progress)*
    - Bind all other unassigned apps
- ~~Control your microphone's input level~~ *(work in progress)*
- Helpful notifications to let you know if something isn't working

Differences from original project:
- Written in rust instead of go
- **Lightweight, also feature rich**
- - Uses < 4MB of storage and memory *(tested on Linux system for version 0.1.0)*
- - **Midi virtual device support** *(work in progress)*
- Based on packets instead of strings
- Uses XDG config path for linux
- Tray icon is optional and runs as separate app *(work in progress)*
- Chaining devices to support 255 sliders *(work in progress)*
- **Supports ESP32 as hardware**
- **Flash once, add sliders when you want** *(work in progress)*
- **Optional Easy configuration UI app** *(work in progress)*
- Installer with a portable version

## Building your own

For detail instruction and instrucion videos see:
- Official guide at [prefex.dev/deejx](https://prefex.dev/deejx)
- Official guide at [prefex.dev/deejx/flash](https://prefex.dev/deejx/flash)
- *(have you made a video tutorial or text guide or something else. Make an PR for this readme section!)*

## How it works

### Hardware

- The sliders are connected to 2 (or as many as you like) analog pins on an ESP32 board. They're powered from the board's 3.3V output (see schematic)
- The board connects via a USB cable to the PC

#### Schematic

![Hardware schematic](assets/schematic.png)

### Software

- The code running on the Arduino/ESP32 board is a [Rust program](./firmware/src/bin/main.rs) constantly writing current slider values over its serial interface
- The PC runs a lightweight [Rust client](./driver/src/main.rs) in the background. This client reads the serial stream and adjusts app volumes according to the given configuration file
- The additional [Rust tray / UI companion](./ui) is optional and uses light slint framework. No electron.
- The [Installer](./installer) is written in C# Avalonia with introduction wizard.

## Slider mapping (configuration)

deejx uses a simple YAML-formatted configuration file named [`profile.deejx.yml`](./example.deejx.yml), placed alongside the deejx executable on windows or in `%appdata%/deejx`.\
For linux config is inside XDG config path.

The config file determines which applications (and devices) are mapped to which sliders, and which parameters to use for the connection to the Arduino/ESP32 board, as well as other user preferences.

**This file auto-reloads when its contents are changed, so you can change application mappings on-the-fly without restarting deejx.**

It looks like this:
```yaml
# [...]
mappings:
  - pin: 35
    master: 0
    inverted: false
  - pin: 34
    app: "spotify"
    inverted: false
  - pin: 33
    midi: 10
    inverted: false
  - pin: 32
    inverted: false

# settings for connecting to the esp32/arduino board
serial: /dev/ttyUSB0
baud_rate: 9600
timeout: 1000

# adjust the amount of signal noise reduction depending on your hardware quality
# supported values are "high" (excellent hardware), "default" (regular hardware) or "low" (bad, noisy hardware)
quality: default
```

## Community

This project is quite new and creating Discord, XMPP or IRC and moderating it is not my dream.

### Contributing

Please see [`docs/CONTRIBUTING.md`](./docs/CONTRIBUTING.md).

## License

deej is released under the [MIT license](./LICENSE).