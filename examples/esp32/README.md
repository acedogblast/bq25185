A simple test program for the ESP32 platform.

This uses esp-hal and only builds for the ESP32-C6 platform. This sample
can be easily adapted for any ESP32 platform. This sample will likely need
to be tweaked for the specific pins.

To build:

```cmd
cargo build
```

To flash to your device:

```cmd
cargo flash
```