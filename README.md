# chip8emu_rs

[![GitHub last commit](https://img.shields.io/github/last-commit/DasStone/chip8emu_rs)](https://github.com/DasStone/chip8emu_rs)
[![License](https://img.shields.io/github/license/DasStone/chip8emu_rs)](https://github.com/DasStone/chip8emu_rs/blob/master/LICENSE)

A simple chip8 emulator.

![Invaders](images/Invaders.PNG)
![Pong](images/Pong.PNG)

## Help

```
chip8emu_rs 0.1.0
Adrian Stein <adrian.stein@tum.de>
Chip8 emulator

USAGE:
    chip8emu_rs.exe [FLAGS] [OPTIONS] <ROM> 

FLAGS:
    -m, --mute       Mutes emulator audio   
    -h, --help       Prints help information
    -v, --version    Prints version information

OPTIONS:
    -c, --clock <CLOCK>    Sets CPU clock speed (in Hz). Valid Range: [500, 1000]. Default is 600.
    -s, --scale <SCALE>    Scales pixel size. Valid Range: [1, 100]. Default is 10.
    -t, --theme <THEME>    Color Theme: r, g, b, br, bg, bb, bw. Default is bw.

ARGS:
    <ROM>    Filename of the chip8-program

Quit the emulator at any time by pressing <ESC>. Restart by pressing <SPACE>.
Input mapping:
Emulator     Chip8
+-+-+-+-+    +-+-+-+-+
|1|2|3|4|    |1|2|3|C|
|Q|W|E|R|    |4|5|6|D|
|A|S|D|F|    |7|8|9|E|
|Z|X|C|V|    |A|0|B|F|
+-+-+-+-+    +-+-+-+-+
```

## Helpfull Resources

I can highly recommend writing an emulator yourself. Chip8 seems to be a good system for people getting into emulation, due to it's simplicity. The following resources might help you.

Chip8 technical details:

- ["Guide to making a Chip-8 emulator" by Tobias V. Langhoff](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- ["Chip-8 Technical Reference" by Cowgod](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

Chip8 test-rom:

- [Chip8 test rom by corax89](https://github.com/corax89/chip8-test-rom)

(You will be able to find many more ressources (test-roms, games, documentation, etc.) by simply searching online (github, wikipedia, blogs, etc.))

## TODO

1. **Reliability of "beeps"**
The system has usually ca. $\frac{1}{60}sec$ time in order to produce a beep (which is on the edge of being to short for SDL2 to handle). Currently this time is extendend by a simple counter, but this can lead to individual beeps (in quick succession) being mashed together into one long beep (given the emulated cpu runs fast enough).  
2. **Custom error message on "illegal" instructions**
The emulator will ```panic``` if an instruction tries to access illegal parts of the emulator (e.g. an out of bounds memory adress). This is in general a non issue when playing games (due to them usually being correct). However I would like to implement a custom error message when this happens in order to help developers trying to create chip8 games using this emulator.
3. **Better testing**
Currently the system was only tested by running chip8 test-roms and games. Unit tests had been omitted due to the simplicity of many instructions and the monumental task of implementing tests for every part of the system.
4. **Fullscreen support and dynamic resizing**
5. **Decouple CPU and TIMER timings**
The cpu currently is somewhat dependent on the fixed 60Hz of the timers. This means that the cpu cannot run under 60Hz.
