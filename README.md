# Burner

A simple and safer command line utility for writting images to usb sicks and sd cards.

## Features

* Will only write to removable drives (--internal to override this).
* Validate input looks like a disk image or iso
* List removable drives and allow interactive selection of a drive.
* Simple backup of sd cards or usb sticks.

## Usage

Interactivly select a disk and burn the given iso to it.

```bash
burn archlinux-2017.11.01-x86_64.iso
```

Copy the iso to the given disk

```bash
burn archlinux-2017.11.01-x86_64.iso /dev/sdb
```

Copy an image to an internal disk

```bash
burn archlinux-2017.11.01-x86_64.iso --internal /dev/sda
```

List avaiable removable disks

```bash
burn list
```
