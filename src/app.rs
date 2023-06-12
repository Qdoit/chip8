use std::time::{Duration, Instant};

use pixels::wgpu::Color;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::display::Display;
use crate::{CHIP8Input, InputKey, OldBehaviourConfig};

const SCALING: u64 = 10;

#[derive(Copy, Clone)]
pub struct ColorConfig {
    pub fg_on_color: (u8, u8, u8),
    pub fg_off_color: (u8, u8, u8),
    pub bg_color: (u8, u8, u8),
}

pub fn drive(
    program: &[u8],
    old_behaviour_conf: OldBehaviourConfig,
    tick_time: Duration,
    color_conf: ColorConfig,
) -> Result<(), Error> {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new((64 * SCALING) as f64, (32 * SCALING) as f64);
        WindowBuilder::new()
            .with_title("CHIP-8 Emulator")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(64, 32, surface_texture)?
    };

    let mut display = Display(pixels, color_conf);
    display.0.clear_color(Color {
        r: color_conf.bg_color.0 as f64 / 255.,
        g: color_conf.bg_color.1 as f64 / 255.,
        b: color_conf.bg_color.2 as f64 / 255.,
        a: 1.0,
    });
    display.clear_screen();
    let mut cinput = CHIP8Input {
        pressed_keys: [false; 16],
        released_key: None,
    };
    let mut chip8 = crate::CHIP8::new(old_behaviour_conf);

    let mut timer_instant = Instant::now();

    let mut loop_speed_limit_instant = Instant::now();

    chip8.load_program(program);
    event_loop.run(move |event, _, control_flow| {
        if timer_instant.elapsed() >= Duration::from_micros(16667) {
            chip8.delay_timer = chip8.delay_timer.saturating_sub(1);
            chip8.sound_timer = chip8.sound_timer.saturating_sub(1);
            if chip8.sound_timer > 0 {
                print!("{}", 7u8 as char);
            }
            timer_instant = Instant::now();
        }

        cinput.released_key = None;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                display
                    .0
                    .resize_surface(new_size.width, new_size.height)
                    .unwrap();
            }
            Event::MainEventsCleared => {
                //window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                if let Err(_err) = display.0.render() {
                    control_flow.set_exit();
                    return;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                use winit::event::ElementState;
                if input.state == ElementState::Pressed {
                    if let Some(keycode) = input.virtual_keycode {
                        match keycode {
                            VirtualKeyCode::Escape => control_flow.set_exit(),
                            VirtualKeyCode::Key1 => {
                                cinput.pressed_keys[InputKey::D1 as usize] = true;
                            }
                            VirtualKeyCode::Key2 => {
                                cinput.pressed_keys[InputKey::D2 as usize] = true;
                            }
                            VirtualKeyCode::Key3 => {
                                cinput.pressed_keys[InputKey::D3 as usize] = true;
                            }
                            VirtualKeyCode::Key4 => {
                                cinput.pressed_keys[InputKey::C as usize] = true;
                            }
                            VirtualKeyCode::Q => {
                                cinput.pressed_keys[InputKey::D4 as usize] = true;
                            }
                            VirtualKeyCode::W => {
                                cinput.pressed_keys[InputKey::D5 as usize] = true;
                            }
                            VirtualKeyCode::E => {
                                cinput.pressed_keys[InputKey::D6 as usize] = true;
                            }
                            VirtualKeyCode::R => {
                                cinput.pressed_keys[InputKey::D as usize] = true;
                            }
                            VirtualKeyCode::A => {
                                cinput.pressed_keys[InputKey::D7 as usize] = true;
                            }
                            VirtualKeyCode::S => {
                                cinput.pressed_keys[InputKey::D8 as usize] = true;
                            }
                            VirtualKeyCode::D => {
                                cinput.pressed_keys[InputKey::D9 as usize] = true;
                            }
                            VirtualKeyCode::F => {
                                cinput.pressed_keys[InputKey::E as usize] = true;
                            }
                            VirtualKeyCode::Z => {
                                cinput.pressed_keys[InputKey::A as usize] = true;
                            }
                            VirtualKeyCode::X => {
                                cinput.pressed_keys[InputKey::D0 as usize] = true;
                            }
                            VirtualKeyCode::C => {
                                cinput.pressed_keys[InputKey::B as usize] = true;
                            }
                            VirtualKeyCode::V => {
                                cinput.pressed_keys[InputKey::F as usize] = true;
                            }
                            _ => (),
                        }
                    }
                } else if input.state == ElementState::Released {
                    if let Some(keycode) = input.virtual_keycode {
                        match keycode {
                            VirtualKeyCode::Escape => control_flow.set_exit(),
                            VirtualKeyCode::Key1 => {
                                cinput.pressed_keys[InputKey::D1 as usize] = false;
                                cinput.released_key.replace(InputKey::D1);
                            }
                            VirtualKeyCode::Key2 => {
                                cinput.pressed_keys[InputKey::D2 as usize] = false;
                                cinput.released_key.replace(InputKey::D2);
                            }
                            VirtualKeyCode::Key3 => {
                                cinput.pressed_keys[InputKey::D3 as usize] = false;
                                cinput.released_key.replace(InputKey::D3);
                            }
                            VirtualKeyCode::Key4 => {
                                cinput.pressed_keys[InputKey::C as usize] = false;
                                cinput.released_key.replace(InputKey::C);
                            }
                            VirtualKeyCode::Q => {
                                cinput.pressed_keys[InputKey::D4 as usize] = false;
                                cinput.released_key.replace(InputKey::D4);
                            }
                            VirtualKeyCode::W => {
                                cinput.pressed_keys[InputKey::D5 as usize] = false;
                                cinput.released_key.replace(InputKey::D5);
                            }
                            VirtualKeyCode::E => {
                                cinput.pressed_keys[InputKey::D6 as usize] = false;
                                cinput.released_key.replace(InputKey::D6);
                            }
                            VirtualKeyCode::R => {
                                cinput.pressed_keys[InputKey::D as usize] = false;
                                cinput.released_key.replace(InputKey::D);
                            }
                            VirtualKeyCode::A => {
                                cinput.pressed_keys[InputKey::D7 as usize] = false;
                                cinput.released_key.replace(InputKey::D7);
                            }
                            VirtualKeyCode::S => {
                                cinput.pressed_keys[InputKey::D8 as usize] = false;
                                cinput.released_key.replace(InputKey::D8);
                            }
                            VirtualKeyCode::D => {
                                cinput.pressed_keys[InputKey::D9 as usize] = false;
                                cinput.released_key.replace(InputKey::D9);
                            }
                            VirtualKeyCode::F => {
                                cinput.pressed_keys[InputKey::E as usize] = false;
                                cinput.released_key.replace(InputKey::E);
                            }
                            VirtualKeyCode::Z => {
                                cinput.pressed_keys[InputKey::A as usize] = false;
                                cinput.released_key.replace(InputKey::A);
                            }
                            VirtualKeyCode::X => {
                                cinput.pressed_keys[InputKey::D0 as usize] = false;
                                cinput.released_key.replace(InputKey::D0);
                            }
                            VirtualKeyCode::C => {
                                cinput.pressed_keys[InputKey::B as usize] = false;
                                cinput.released_key.replace(InputKey::B);
                            }
                            VirtualKeyCode::V => {
                                cinput.pressed_keys[InputKey::F as usize] = false;
                                cinput.released_key.replace(InputKey::F);
                            }
                            _ => (),
                        }
                    }
                }
            }
            _ => (),
        };

        if loop_speed_limit_instant.elapsed() >= tick_time {
            let res = chip8.update(cinput.clone(), &mut display);
            if res.request_redraw {
                window.request_redraw();
            }
            loop_speed_limit_instant = Instant::now();
        }
    })
}
