use chip8::*;

use chip8::app::ColorConfig;

fn main() {
    let (rom, old_behaviour_conf, tick_time, color_conf) = handle_args();
    app::drive(&rom, old_behaviour_conf, tick_time, color_conf).unwrap();
    println!("Hello, world!");
}

fn handle_args() -> (
    Vec<u8>,
    OldBehaviourConfig,
    std::time::Duration,
    ColorConfig,
) {
    let mut args = std::env::args();
    args.next().unwrap();
    let mut filepath = None;
    let mut oldbeh = OldBehaviourConfig {
        fx65: false,
        fx55: false,
        i_8xy6: false,
        i_8xye: false,
        bnnn: false,
        fx1e: false,
    };
    let mut bg_color: Option<(u8, u8, u8)> = None;
    let mut fg_on_color: Option<(u8, u8, u8)> = None;
    let mut fg_off_color: Option<(u8, u8, u8)> = None;
    let mut ticktime: Option<u64> = None;
    while let Some(e) = args.next() {
        let e: &str = &e;
        match e {
            "-ob" | "--old-behaviour" => {
                let instruction = match args.next() {
                    Some(e) => e.to_lowercase(),
                    None => {
                        eprintln!(
                            "Instruction name not provided for --old-behaviour.\nUSAGE:\n{}",
                            USAGE
                        );
                        std::process::exit(1);
                    }
                };

                let instruction: &str = &instruction;
                match instruction {
                    "fx65" => oldbeh.fx65 = true,
                    "fx55" => oldbeh.fx55 = true,
                    "8xye" => oldbeh.i_8xye = true,
                    "8xy6" => oldbeh.i_8xy6 = true,
                    "bnnn" => oldbeh.bnnn = true,
                    "fx1e" => oldbeh.fx1e = true,
                    e => {
                        eprintln!("{e} is not a valid instruction name. Valid names are: [FX65|FX55|8XYE|8XY6|BNNN|FX1E].");
                        std::process::exit(1)
                    }
                }
            }
            "-tt" | "--tick-time" => {
                let e = match args.next() {
                    Some(e) => e.parse::<u64>(),
                    None => {
                        eprintln!("Argument not provided for --tick-time.\nUSAGE:\n{}", USAGE);
                        std::process::exit(1);
                    }
                };

                ticktime.replace(match e {
                    Ok(e) => e,
                    Err(_) => {
                        eprintln!(
                            "Argument for --tick-time is not a number.\nUSAGE:\n{}",
                            USAGE
                        );
                        std::process::exit(1);
                    }
                });
            }
            "-bg" | "--bg-color" => {
                bg_color.replace(handle_color(&mut args, "--background-color"));
            }
            "-fg-off" | "--fg-off-color" => {
                fg_off_color.replace(handle_color(&mut args, "--fg-off-color"));
            }
            "-fg-on" | "--fg-on-color" => {
                fg_on_color.replace(handle_color(&mut args, "--fg-on-color"));
            }
            "--help" => {
                println!("{}", USAGE);
                std::process::exit(0);
            }
            name => {
                filepath.replace(name.to_owned());
            }
        }
    }
    fn handle_color(args: &mut std::env::Args, option: &str) -> (u8, u8, u8) {
        let string = match args.next() {
            Some(e) => e,
            None => {
                eprintln!("Argument not provided for {option}.\nUSAGE:\n{}", USAGE);
                std::process::exit(1);
            }
        };

        if string.len() != 6 || string.as_bytes().len() != 6 {
            eprintln!("Invalid color format for {option}.\nUSAGE\n{}", USAGE);
            std::process::exit(1);
        }
        let mut arr = [0u8; 3];
        for i in (0..6).step_by(2) {
            arr[i / 2] = match u8::from_str_radix(&string[i..i + 2], 16) {
                Ok(e) => e,
                Err(_) => {
                    eprintln!("Invalid color format for {option}.\nUSAGE\n{}", USAGE);
                    std::process::exit(1);
                }
            }
        }

        (arr[0], arr[1], arr[2])
    }
    let filepath = match filepath {
        Some(e) => e,
        None => {
            eprint!("Path to ROM not provided.\nUSAGE:\n{}", USAGE);
            std::process::exit(1)
        }
    };
    let ticktime = std::time::Duration::from_micros(ticktime.unwrap_or(1430));
    (
        match std::fs::read(&filepath) {
            Ok(e) => e,
            Err(_) => {
                eprintln!("Could not read file \"{}\".", filepath);
                std::process::exit(1)
            }
        },
        oldbeh,
        ticktime,
        ColorConfig {
            bg_color: bg_color.unwrap_or((76, 13, 179)),
            fg_on_color: fg_on_color.unwrap_or((255, 255, 255)),
            fg_off_color: fg_off_color.unwrap_or((0, 0, 0)),
        },
    )
}

const USAGE: &str = r#"
    chip8 [path to rom] [args]

Args:
    -ob,     --old-behaviour [FX65|FX55|8XY6|8XYE|BNNN|FX1E]     Use older behaviour for given instruction
    -tt,     --tick-time [number in microseconds]                Sets the minimum time a single tick (instruction loop) takes. This does not affect the timers.
    -bg,     --bg-color [color code]                             Sets the background color of the emulator. Color code format is RRBBGG (e.g. -bg FFFFFF to set it to white).
    -fg-off, --fg-off-color [color code]                         Sets the color of "off" pixels (black by default).
    -fg-on,  --fg-on-color  [color code]                         Sets the color of "on" pixels (white by default).
"#;
