# Rust Bucket

This repo contains my solutions to the InventrKits '30 days lost in space'
Arduino lessons/game/project

To make things more interesting(hard) 
I will attempt to complete the course using Rust
instead of C

[instructional video playlist](https://www.youtube.com/playlist?list=PL-ykYLZSERMRDkKIkIq7DliDU5xdJEw3t)


## Dev Log

20250122
- Assembled dip-switch circuit
- Added a switch-control function to manage the cabin light
- Test: the switch now controls the cabin light!
- Day 3 complete!

20250121
- assembled LED circuit
- verified circuit with 5v input
- move source code out of the redundant rustbucket folder
- Discovery! the onboard LED is controlled from pin d13, 
and we can use it to drive our external LED as well.
- activated external LED! driven by pin d12.
- the onboard and external lights now blink in sequence,
in the future, lets see about driving them asynchronously, or in parallel
- Refactor: Extracted the led control code into a function 
that can operate on any OutputPin
- Day 2 complete!
- Completed 3 exercise with partner

20250120
- Begin Day 2 
- Assisted partner in building an LED circuit
- she successfully controlled the LED using her C code

20250119
- completed Day 1
- successfully flashed blinky lite Rust code, verified with partner.

20250118
- Add environment variables to the conda env:
  - `~/miniconda3/envs/rust-python/etc/conda/activate.d/rust.sh`
    ```bash
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=$HOME/miniconda3/envs/rust-python/bin/x86_64-conda-linux-gnu-cc
    ```
```
> cat ~/miniconda3/envs/rust-python/etc/conda/activate.d/env_vars.sh 
#export RUST_SRC_PATH=$HOME/miniconda3/envs/rust-python/lib/librust/src/rust/src
export CARGO_HOME=$HOME/miniconda3/envs/rust-python/
export RUSTUP_HOME=$HOME/miniconda3/envs/rust-python/.rustup
```

- Install rustup in the conda env
- use custom install options
- check the install paths are in the conda environment
```
  default host triple: x86_64-unknown-linux-gnu
  default toolchain: nightly
  profile: default
  modify PATH variable: no
```

```
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --no-modify-path --default-toolchain nightly --profile default
```
- Install ravedude: `cargo install ravedude`
- `sudo usermod -aG dialout $USER`

- test flash the board: `ravedude /dev/ttyACM0 target/avr-atmega328p/release/rustbucket.elf
- attempts to flash using ravedude failed with error 'board not found'
- flash using avrdude was successful: `avrdude -p atmega328p -c arduino -P /dev/ttyACM0 -U flash:w:target/avr-atmega328p/release/rustbucket.elf:e`

20250117
- Setup IDE. I preffer to setup dev dependencies in isolated Conda environments,
however, Rust makes this fairly difficult, and I may be forced to use system-level Rust for now :(
- Following [A complete guide to running Rust on Arduino](https://blog.logrocket.com/complete-guide-running-rust-arduino)
- Install the AVR toolchain system dependencies `sudo apt install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential`, 
not available in conda, *may need to install an updated avrdude using conda if apt is out of date*
worry about isolated dev env later.
- Instructions for adding the Arduino target `rustup target add avr-unknown-gnu-atmega328` fails
- `avrdude` is a cargo-generate template for avr-hal projects.
- A Hardware Abstraction Layer (HAL) is required to run Rust on AVR microcontrollers and other common boards.
- stable does not support the Arduino hardware target: `rustup toolchain install nightly`
- `cargo install ravedude`
- `cargo install cargo-generate`
- In the interest of getting this rust bucket up and running asap, we will use cargo-generate as advised... 
hopefully it will not make too much of a mess.
- `cargo generate --git https://github.com/LundyBernard/avr-hal-template`
- This generated a new file in the working directory, rustbucket/rustbucket/... 
i'll deal with it for now, as I may need multiple sub-projects
