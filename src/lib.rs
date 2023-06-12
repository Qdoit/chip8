mod display;
use display::Display;

pub mod app;

pub struct CHIP8 {
    pc: usize,
    ram: [u8; 4096],
    stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    i_reg: u16,
    vx_reg: [u8; 16],
    old_behaviour_conf: OldBehaviourConfig,
}

#[derive(Debug, Clone)]
pub struct CHIP8Input {
    pub pressed_keys: [bool; 16],
    pub released_key: Option<InputKey>,
}

pub struct CHIP8Output {
    pub request_redraw: bool,
}

#[derive(Debug)]
pub struct OldBehaviourConfig {
    pub fx65: bool,
    pub fx55: bool,
    pub i_8xy6: bool,
    pub i_8xye: bool,
    pub bnnn: bool,
    pub fx1e: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[rustfmt::skip]
pub enum InputKey {
    D1 = 1, D2 = 2, D3 = 3, C = 0xC, 
    D4 = 4, D5 = 5, D6 = 6, D = 0xD, 
    D7 = 7, D8 = 8, D9 = 9, E = 0xE, 
    A = 0xA, D0 = 0, B = 0xB, F = 0xF
}

impl CHIP8 {
    pub fn new(old_behaviour_conf: OldBehaviourConfig) -> Self {
        let mut ram = [0; 4096];

        ram[0x50..=0x9F].copy_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);

        CHIP8 {
            pc: 0x200,
            ram,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 5,
            i_reg: 0,
            vx_reg: [0; 16],
            old_behaviour_conf,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.ram[0x200..(program.len() + 0x200)].copy_from_slice(program);
    }

    pub fn update(&mut self, input: CHIP8Input, display: &mut Display) -> CHIP8Output {
        let instruction =
            u16::from_be_bytes((&self.ram[self.pc..=self.pc + 1]).try_into().unwrap());
        self.pc += 2;
        let first_nibble = ((instruction & 0b1111000000000000) >> 12) as u8;
        let second_nibble = ((instruction & 0b0000111100000000) >> 8) as usize;
        let third_nibble = ((instruction & 0b0000000011110000) >> 4) as usize;
        let fourth_nibble = (instruction & 0b0000000000001111) as u8;

        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        let mut out = CHIP8Output {
            request_redraw: false,
        };
        match first_nibble {
            0x0 => match nn {
                0xE0 => {
                    display.clear_screen();
                    out.request_redraw = true;
                }
                0xEE => {
                    let address = self.stack.pop().expect("Popped with empty stack.");
                    self.pc = address as usize;
                }
                _ => panic!("unknown opcode"),
            },
            0x1 => {
                self.pc = nnn as usize;
            }
            0x2 => {
                self.stack.push(self.pc as u16);
                self.pc = nnn as usize;
            }
            0x3 => {
                if self.vx_reg[second_nibble] == nn {
                    self.pc += 2;
                }
            }
            0x4 => {
                if self.vx_reg[second_nibble] != nn {
                    self.pc += 2;
                }
            }
            0x5 => match fourth_nibble {
                0x0 => {
                    if self.vx_reg[second_nibble] == self.vx_reg[third_nibble] {
                        self.pc += 2;
                    }
                }
                _ => panic!("unknown opcode"),
            },
            0x6 => {
                self.vx_reg[second_nibble] = nn;
            }
            0x7 => {
                self.vx_reg[second_nibble] = self.vx_reg[second_nibble].wrapping_add(nn);
            }
            0x8 => match fourth_nibble {
                0x0 => self.vx_reg[second_nibble] = self.vx_reg[third_nibble],
                0x1 => self.vx_reg[second_nibble] |= self.vx_reg[third_nibble],
                0x2 => self.vx_reg[second_nibble] &= self.vx_reg[third_nibble],
                0x3 => self.vx_reg[second_nibble] ^= self.vx_reg[third_nibble],
                0x4 => {
                    let overflowed;
                    (self.vx_reg[second_nibble], overflowed) =
                        self.vx_reg[second_nibble].overflowing_add(self.vx_reg[third_nibble]);
                    self.vx_reg[0xf] = if overflowed { 1 } else { 0 };
                }
                0x5 => {
                    let (f, s) = (self.vx_reg[second_nibble], self.vx_reg[third_nibble]);
                    self.vx_reg[second_nibble] = f.wrapping_sub(s);
                    self.vx_reg[0xf] = if f > s { 1 } else { 0 };
                }
                0x6 => {
                    if self.old_behaviour_conf.i_8xy6 {
                        self.vx_reg[second_nibble] = self.vx_reg[third_nibble];
                    }
                    let bit = self.vx_reg[second_nibble] & !(0x1);
                    self.vx_reg[second_nibble] >>= 1;
                    self.vx_reg[0xF] = bit;
                }
                0x7 => {
                    let (f, s) = (self.vx_reg[third_nibble], self.vx_reg[second_nibble]);
                    self.vx_reg[second_nibble] = f.wrapping_sub(s);
                    self.vx_reg[0xf] = if f > s { 1 } else { 0 };
                }
                0xE => {
                    if self.old_behaviour_conf.i_8xye {
                        self.vx_reg[second_nibble] = self.vx_reg[third_nibble];
                    }
                    let bit = self.vx_reg[second_nibble] & 0b1000;
                    self.vx_reg[second_nibble] <<= 1;
                    self.vx_reg[0xF] = bit;
                }
                _ => panic!("unknown opcode"),
            },
            0x9 => match fourth_nibble {
                0x0 => {
                    if self.vx_reg[second_nibble] != self.vx_reg[third_nibble] {
                        self.pc += 2;
                    }
                }
                _ => panic!("unknown opcode"),
            },

            0xA => {
                self.i_reg = nnn;
            }
            0xB => {
                self.pc = (nnn
                    + if self.old_behaviour_conf.bnnn {
                        self.vx_reg[0]
                    } else {
                        self.vx_reg[second_nibble]
                    } as u16) as usize;
            }
            0xC => {
                use rand::prelude::*;
                let random: u8 = rand::thread_rng().gen();
                self.vx_reg[second_nibble] = random & nn;
            }
            0xD => {
                out.request_redraw = true;
                let mut x;
                let mut y = self.vx_reg[third_nibble] % 32;
                self.vx_reg[15] = 0;

                for i in 0..fourth_nibble as usize {
                    x = self.vx_reg[second_nibble] % 64;
                    let byte = self.ram[self.i_reg as usize + i];
                    const BYTE_COMBOS: [(u8, u8); 8] = [
                        (0b10000000, 7),
                        (0b01000000, 6),
                        (0b00100000, 5),
                        (0b00010000, 4),
                        (0b00001000, 3),
                        (0b00000100, 2),
                        (0b00000010, 1),
                        (0b00000001, 0),
                    ];
                    for combo in BYTE_COMBOS {
                        let bit = (byte & combo.0) >> combo.1;
                        if bit != 0 {
                            if display.get_pixel(x, y) {
                                display.unset_pixel(x, y);
                                self.vx_reg[15] = 1;
                            } else {
                                display.set_pixel(x, y);
                            }
                        }
                        x += 1;
                        if x > 63 {
                            break;
                        }
                    }

                    y += 1;
                    if y > 31 {
                        break;
                    }
                }
            }
            0xE => match nn {
                0x9E => {
                    if input.pressed_keys[self.vx_reg[second_nibble] as usize] {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    if !input.pressed_keys[self.vx_reg[second_nibble] as usize] {
                        self.pc += 2;
                    }
                }
                _ => panic!("unknown opcode"),
            },
            0xF => match nn {
                0x1E => {
                    // make overflow behaviour here configurable
                    let overflowing;
                    (self.i_reg, overflowing) = self
                        .i_reg
                        .overflowing_add(self.vx_reg[second_nibble] as u16);
                    if !self.old_behaviour_conf.fx1e {
                        self.vx_reg[0xF] = if overflowing { 1 } else { 0 }
                    }
                }
                0x0A => {
                    if input.released_key.is_none() {
                        self.pc -= 2;
                    } else {
                        self.vx_reg[second_nibble] = input.released_key.unwrap() as u8;
                    }
                }
                0x29 => {
                    self.i_reg = 0x50 + ((self.vx_reg[second_nibble] & 0x0F) as u16) * 5;
                }
                0x33 => {
                    let num = self.vx_reg[second_nibble];
                    self.ram[self.i_reg as usize] = num / 100;
                    self.ram[self.i_reg as usize + 1] = (num / 10) % 10;
                    self.ram[self.i_reg as usize + 2] = num % 10;
                }
                0x55 => {
                    for idx in 0..=second_nibble {
                        self.ram[self.i_reg as usize
                            + if !self.old_behaviour_conf.fx55 {
                                idx
                            } else {
                                0
                            }] = self.vx_reg[idx];
                        if self.old_behaviour_conf.fx55 {
                            self.i_reg += 1;
                        }
                    }
                }
                0x65 => {
                    for idx in 0..=second_nibble {
                        self.vx_reg[idx] = self.ram[self.i_reg as usize
                            + if !self.old_behaviour_conf.fx65 {
                                idx
                            } else {
                                0
                            }];
                        if self.old_behaviour_conf.fx65 {
                            self.i_reg += 1;
                        }
                    }
                }
                0x07 => {
                    self.vx_reg[second_nibble] = self.delay_timer;
                }
                0x15 => {
                    self.delay_timer = self.vx_reg[second_nibble];
                }
                0x18 => {
                    self.sound_timer = self.vx_reg[second_nibble];
                }
                _ => panic!("unknown opcode"),
            },
            _ => {}
        }
        out
    }
}
