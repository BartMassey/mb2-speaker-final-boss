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
        .set_prescaler(pwm::Prescaler::Div2)
        .set_counter_mode(pwm::CounterMode::UpAndDown);

    let mut state = [false, false];
    let mut key = 69u8;
    let mut playing = true;
    let mut tick = 0u64;
    let mut tick_accel = 5u64;

    loop {
        let old_key = key;
        let new_state: [bool; 2] =
            core::array::from_fn(|b| buttons[b].is_low().unwrap());
        match new_state {
            [true, false] => {
                if state == new_state {
                    if tick < tick_accel {
                        tick += 1;
                    } else {
                        key = key.saturating_sub(1);
                        tick = 0;
                        tick_accel = (tick_accel - 1).max(1);
                    }
                } else {
                    key = key.saturating_sub(1);
                    tick = 0;
                    tick_accel = 5;
                }
            }
            [false, true] => {
                if state == new_state {
                    if tick < tick_accel {
                        tick += 1;
                    } else {
                        key = (key + 1).min(127);
                        tick = 0;
                        tick_accel = (tick_accel - 1).max(1);
                    }
                } else {
                    key = (key + 1).min(127);
                    tick = 0;
                    tick_accel = 5;
                }
            }
            [true, true] => {
                if new_state != state {
                    playing = !playing;
                    if playing {
                        pwm.enable();
                    } else {
                        pwm.disable();
                    }
                }
            }
            [false, false] => (),
        }
        if state != new_state || key != old_key {
            rprintln!(
                "playing: {:?}, buttons: {:?}, key: {}",
                playing, new_state, key,
            );
        }
        state = new_state;

        if playing {
            let f = keytones::key_to_frequency(key).round() as u32;
            pwm
                .set_period(time::Hertz(f))
                .set_duty_on_common(pwm.max_duty() / 2);
        }
        
        timer.delay_ms(100);
    }
}
