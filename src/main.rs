#![no_std]
#![no_main]

use panic_rtt_target as _;

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use keytones::{self, Float};
use microbit::{Board, hal::{gpio, pwm, time, timer}};
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let speaker_pin = board.speaker_pin
        .into_push_pull_output(gpio::Level::High)
        .degrade();
    let mut timer = timer::Timer::new(board.TIMER0);
    let pwm = pwm::Pwm::new(board.PWM0);
    let mut buttons = [
        board.buttons.button_a.degrade().into_floating_input(),
        board.buttons.button_b.degrade().into_floating_input(),
    ];

    pwm
        .set_output_pin(pwm::Channel::C0, speaker_pin)
        .set_prescaler(pwm::Prescaler::Div16)
        .set_counter_mode(pwm::CounterMode::UpAndDown);

    let mut state = [false, false];
    let mut key = 69u8;
    let mut width = 50u8;
    let mut tick = 0u64;
    let mut tick_accel = 5u64;

    const MODE_OFF: u8 = 0;
    const MODE_FREQ: u8 = 1;
    const MODE_WIDTH: u8 = 2;
    const NMODES: u8 = 3;
    let mut playing = MODE_FREQ;

    loop {
        let old_key = key;
        let old_width = width;
        let old_playing = playing;

        let new_state: [bool; 2] =
            core::array::from_fn(|b| buttons[b].is_low().unwrap());
        match new_state {
            [true, false] => {
                if state != new_state || tick >= tick_accel {
                    match playing {
                        MODE_FREQ => {
                            // key 35 is min for 50% cycle.
                            // key 16 is min for working at all.
                            key = (key - 1).max(16);
                        }
                        MODE_WIDTH => {
                            width = (width - 1).max(1);
                        }
                        _ => (),
                    }
                }
                if state == new_state {
                    if tick < tick_accel {
                        tick += 1;
                    } else {
                        tick = 0;
                        tick_accel = (tick_accel - 1).max(1);
                    }
                } else {
                    tick = 0;
                    tick_accel = 5;
                }
            }
            [false, true] => {
                if state != new_state || tick >= tick_accel {
                    match playing {
                        MODE_FREQ => {
                            // key 35 is min for 50% cycle.
                            // key 16 is min for working at all.
                            key = (key + 1).min(127);
                        }
                        MODE_WIDTH => {
                            width = (width + 1).min(50);
                        }
                        _ => (),
                    }
                }
                if state == new_state {
                    if tick < tick_accel {
                        tick += 1;
                    } else {
                        tick = 0;
                        tick_accel = (tick_accel - 1).max(1);
                    }
                } else {
                    tick = 0;
                    tick_accel = 5;
                }
            }
            [true, true] => {
                if new_state != state {
                    playing = (playing + 1) % NMODES;
                    if playing == MODE_OFF {
                        pwm.disable();
                    } else {
                        pwm.enable();
                    }
                }
            }
            [false, false] => (),
        }
        let f = keytones::key_to_frequency(key).round() as u32;
        let w = (pwm.max_duty() as f32 * width as f32 / 100.0).floor() as u16;
        if playing != MODE_OFF {
            pwm
                .set_period(time::Hertz(f))
                .set_duty_on_common(w);
        }

        let changed = playing != old_playing || key != old_key || width != old_width;
        if changed {
            let mode_name = match playing {
                MODE_FREQ => "freq",
                MODE_WIDTH => "width",
                MODE_OFF => "off",
                _ => panic!("unexpected mode"),
            };
            rprintln!(
                "mode: {}, f: {} (key: {}), width: {} (d: {}, w: {})",
                mode_name, f, key, width, pwm.max_duty(), w,
            );
        }
        
        state = new_state;
        timer.delay_ms(100);
    }
}
