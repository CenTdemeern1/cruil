# `cruil`: Charlotte's Raw USB Input Library

> (Pronounced "cru-ill" or "cruel")

`cruil` is a library that uses raw USB HID under the hood to be able to distinguish between and read input from input devices that are usually combined by the operating system (keyboards, mice)

On supported operating systems, this also includes Bluetooth HID devices.

> [!WARNING]
> cruil is still really early in development.
>
> The entire API may change in the future.
>
> See ["Currently broken/WIP/unimplemented"](#currently-brokenwipunimplemented)
> for a list of current limitations.

## Why is this useful?

Some ideas:
- Local multi-player video games that use keyboards and mice
- Simulation software that uses multiple keyboards to create large control panels
- Software to create a large emoji keyboard, [à la Tom Scott](https://www.youtube.com/watch?v=3AtBE9BOvvk), [but without the bodge](https://www.youtube.com/watch?v=lIFE7h3m40U)
- Multi-seat software that uses multiple mice to create multiple pointers on one screen

## What cruil does NOT do

- Handle keyboard layouts (as of now, this may change in the future if necessary)
- Map keys or key combinations to typed characters (out of scope)

## Currently broken/WIP/unimplemented

- Report descriptor parsing, which breaks:
    - Linux, BSD, and other Unix support
    - A lot of mice
- Linux, BSD, and other Unix: Bluetooth
    - dependency limitation, will look into solving this in the future

### Operating system quirks

Cruil uses [hidapi](https://github.com/libusb/hidapi) under the hood, so it inherits a lot of the same limitations:
- Windows: everything should just work
- macOS: requires input monitoring permissions
- Linux, BSD, and other Unix: uses the libusb backend, so Bluetooth is unsupported
