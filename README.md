# `cruil`: Charlotte's Raw USB Input Library

> (Pronounced "cru-ill" or "cruel")

`cruil` is a library that uses raw USB HID under the hood to be able to distinguish between and read input from input devices that are usually combined by the operating system (keyboards, mice)

On supported operating systems, this also includes Bluetooth HID devices.

## Why is this useful?

Some ideas:
- Local multi-player video games that use keyboards and mice
- Simulation software that uses multiple keyboards to create large control panels
- Software to create a large emoji keyboard, [à la Tom Scott](https://www.youtube.com/watch?v=3AtBE9BOvvk), [but without the bodge](https://www.youtube.com/watch?v=lIFE7h3m40U)
- Multi-seat software that uses multiple mice to create multiple pointers on one screen

## Operating system quirks

Cruil uses [hidapi](https://github.com/libusb/hidapi) under the hood, so it inherits a lot of the same limitations:
- Windows: everything should just work
- macOS: requires input monitoring permissions
- Linux, BSD, and other Unix: uses the libusb backend, so Bluetooth is unsupported

## What this does NOT do

- Handle keyboard layouts (as of now, this may change in the future if necessary)
- Map keys or key combinations to typed characters
