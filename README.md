# Rust Bucket

This repo contains my solutions to the InventrKits '30 days lost in space'
Arduino lessons/game/project

To make things more interesting(hard) 
I will attempt to complete the course using Rust
instead of C

[instructional video playlist](https://www.youtube.com/playlist?list=PL-ykYLZSERMRDkKIkIq7DliDU5xdJEw3t)


## Dev Log

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
