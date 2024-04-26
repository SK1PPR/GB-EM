# GB-EM

A Gameboy Classic and Gameboy Color emulator written in Rust

## Building

1. Clone the repository into your desired directory
2. Change to the project directory
3. You can either run the shell script using `./run.sh ` or run `cargo build --release`
4. Running the emulator can be done by `cargo run --release` or by changing directory to `target/release`.
5. You can add this binary to `~/.local/bin/` in linux or  `PATH` in windows.
6. You can run test roms and sample games added in `/test-roms`. You can run any `.gb` and `.gbc` roms on this emulator. (**Note:** Make sure to extract the ROMs before running).

Then you can explore the ability of the emulator by `gb_em --help`. Which outputs 

```
A Gameboy Colour emulator written in Rust

Usage: eb_em [OPTIONS] <filename>

Arguments:
  <filename>  Sets the ROM file to load

Options:
  -s, --serial         Prints the data from the serial port to stdout
  -p, --printer        Emulates a gameboy printer
  -c, --classic        Forces the emulator to run in classic Gameboy mode
  -x, --scale <scale>  Sets the scale of the interface. Default: 2
  -a, --audio          Enables audio
      --skip-checksum  Skips verification of the cartridge checksum
      --test-mode      Starts the emulator in a special test mode
  -h, --help           Print help
  -V, --version        Print version
```

Now you can look below for the Keybindings section below.

## Keybindings

### Gameplay Keybindings

| Key on Keyboard    | Emulator Key       |
| ------------------ | ------------------ |
| Z                  | A                  |
| X                  | B                  |
| Up/Down/Left/Right | Up/Down/Left/Right |
| Space              | Select             |
| Return/Enter       | Start              |

### General Keybindings

| Key on Keyboard   | Emulator Action                     |
| ----------------- | ----------------------------------- |
| 1                 | Switch to 1:1 scale                 |
| R                 | Restore scale given on command line |
| Left Shift (Hold) | Unrestricted Speed Mode             |
| T                 | Change pixel interpolation          |


## Implemented

* CPU
  - All instructions correct
  - All timings correct
  - Double speed mode
* GPU
  - Normal mode
  - Color mode
* Keypad
* Timer
* Audio
* MMU
  - MBC-less
  - MBC1
  - MBC3 (with RTC)
  - MBC5
  - save games
* Printing

## Future scope

- GameBoy Advance support
- Making the emulator cycle-accurate
- Adding better support for sound
- Adding UI support for ROM opening

## Test mode
The test mode, activated with the `--test-mode` flag, provides some functionality for running
[GBEmulatorShootout](https://github.com/daid/GBEmulatorShootout). This is still under development.

## Special thanks to

* https://github.com/mvdnes/rboy.git
* http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU
* http://bgb.bircd.org/pandocs.htm
* https://github.com/alexcrichton/jba (The Rust branch)
* https://gbdev.io/pandocs/