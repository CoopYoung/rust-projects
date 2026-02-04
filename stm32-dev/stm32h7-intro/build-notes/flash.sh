#!/bin/zsh

export WD="/home/coop/rust-projects/stm32-dev/stm32h7-intro"
#st-flash erase ../target/thumbv7em-none-eabihf/release/blinky.bin 0x08000000
cargo build --release --bin blinky
cargo objcopy --release --bin blinky -- -O binary $WD/target/thumbv7em-none-eabihf/release/blinky.bin
st-flash write $WD/target/thumbv7em-none-eabihf/release/blinky.bin 0x08000000
