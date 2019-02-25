# Rust and blue-pill experiments
Attach sensors to blue-pill and try to communicate using Rust.

Spin-off from the [quickstart project](https://github.com/TeXitoi/blue-pill-quickstart)

Quickstart a Rust project for the [blue pill board](https://wiki.stm32duino.com/index.php?title=Blue_Pill), or any STM32F103xx board.

![ST-Link V2 to blue pill](st-link-v2-blue-pill.jpg)

Launch openocd:

```shell
./openocd.sh
```
 
Open a new terminal, compile and flash

```shell
cd blue-pill-quickstart
cargo run
```

Now, the program is flashed, and you are on a gdb prompt. Type `c` (for continue) you can see the on board LED blinking.

## Trouble Shooting

### Wrong connection of the ST-Link

The formerly mentionned ST-Link may not have the right pin mapping as showed on its shell. If `openocd` returns `unknown code 0x9`, please check the pin mapping by removing the shell and re-connect your ST-Link with the mapping shown on the PCB.

If you're unable to remove the shell, try this pin mapping:

|pin|      |pin|       | 
|---|------|---|-------|
| 1 | RST  | 2 | SWCLK |
| 3 | SWIM | 4 | SWDIO |
| 5 | GND  | 6 | GND   |
| 7 | 3.3V | 8 | 3.3V  |
| 9 | 5.0V |10 | 5.0V  |

### Flash protected

When flashing your blue pill for the first time, flashing may fail with the following messages in the openocd console:

```
Error: stm32x device protected
Error: failed erasing sectors 0 to 23
Error: flash_erase returned -4
```

This means your blue pill's flash is protected. To unlock it, you can connect to your openocd session with:

```shell
telnet localhost 4444
```

and type the following commands:

```
reset halt
stm32f1x unlock 0
reset halt
```

## Sources

This quickstart is inspired by the [cortex-m-quickstart](https://github.com/japaric/cortex-m-quickstart) and [Discovery](https://rust-embedded.github.io/discovery/). I recommand reading them.
