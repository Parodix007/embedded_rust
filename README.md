# Rust for embedded
This repo contains code of basic rust embedded programs. 

## Enviroment 
* STM32F3DISCOVERY board
* Rust with:
	- openOCD
	- `arm-none-eabi-gdb`
	- cargo-binutils

## Handy commands
* In the project direcotry: `cargo build --target thumbv7em-none-eabihf`
* In the /tmp directory: `openocd \
  -f interface/stlink-v2-1.cfg \
  -f target/stm32f3x.cfg`
* In the project directory: `arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/*project_name*`
