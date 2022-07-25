# tabletsettings

Calibrate the wacom device to a screen or window on sway.

## Usage

Devices, monitors, and windows appear exactly as you'd expect it to.
This is done automatically.
Select a device and either a monitor or window to map the tablet to.
If you wish to keep the aspect ratio, select the checkbox for that and it will adjust the width or height on the tablet to keep the aspect ratio.

Because sway is a tiling window manager, it's better to set the app as a floating window if the application you want to set it to is in the same monitor.

![Alt text](https://github.com/avargas05/tabletsettings/raw/main/data/screenshots/main.png "Main window")

## Dependencies
- libwacom
- gtk4
- meson
- ninja
- rust
- pkgconf (base-devel)

## Installation

### AUR
https://aur.archlinux.org/packages/tabletsettings-git/

### Manually
~~~bash
git clone https://github.com/avargas05/tabletsettings.git
cd tabletsettings
meson setup --buildtype release --prefix /usr bin
ninja -C bin
sudo ninja -C bin install
~~~
