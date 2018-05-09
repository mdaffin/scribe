# Scribe

An interactive command line tool for writing images to removable media such as
Raspberry Pi images to SD Cards or Linux distro ISOs to USB flash drives.

## Motivation

There are many tools out there to write an image to a disk, `dd`, `cp`, `pv`,
`ddrescue`. They all, however, have the same problem: it is very easy to screw
up with them as they have no safety measure. They are designed to write in both
directions, to disks or files, to internal drives or removable ones - this
makes it very easy to select the wrong drive or you have to keep double
checking where you are actually writing. Tools like [Etcher] solve these
problems and make it both easier and safer to do, but it is a GUI application -
there is missing a command line equivalent. This is the role scribe is designed
to fill.

[Etcher]: https://etcher.io/

## Installation

Currently requires [rust and cargo] to build and install, once you have that
installed simply clone the project then run:

```bash
cargo install
```

[rust and cargo]: https://www.rust-lang.org/en-US/install.html

## Quick Start

Scribe is designed for interactive use, just pass it the img you want to burn
and it will prompt you with a selection of removable devices to chose from.

```bash
$ scribe some.img
> /dev/sda Generic  USB  SD Reader    7.5GiB
  /dev/sdb Kingston DataTraveler 2.0   15GiB
Writing 'some.img' to '/dev/sda'. This can take some time
Finished. The device can now be safely removed.
```
