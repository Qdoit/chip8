# A CHIP-8 interpreter

A minimal CHIP-8 interpreter written in Rust. 

![ibm logo in black and white](ibmlogo.png?raw=true)
![pumpkin dress up splash screen in nord theme colors](pumpkindressup.png?raw=true)
![breakout in gruvbox colors](breakout.png?raw=true)

Windowing is handled by `winit`, rendering by `pixels` and RNG by `rand`. 

### Building
  Just run `cargo build --release`

### Usage
```
    chip8 [path to rom] [args]

Args:
    -ob,     --old-behaviour [FX65|FX55|8XY6|8XYE|BNNN|FX1E]     Use older behaviour for given instruction
    -tt,     --tick-time [number in microseconds]                Sets the minimum time a single tick (instruction loop) takes. This does not affect the timers.
    -bg,     --bg-color [color code]                             Sets the background color of the emulator. Color code format is RRBBGG (e.g. -bg FFFFFF to set it to white).
    -fg-off, --fg-off-color [color code]                         Sets the color of "off" pixels (black by default).
    -fg-on,  --fg-on-color  [color code]                         Sets the color of "on" pixels (white by default).
```

### TODO:
[ ] SCHIP-48 support

[ ] XO-CHIP support

[ ] switch to `softbuffer` for rendering

[ ] config file support

[ ] input remapping

[ ] debug support

### Credits

br8kout sample by SharpenedSpoon
pumpkin dressup sample by SystemLogoff
