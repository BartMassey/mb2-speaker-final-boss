#![no_std]
#![no_main]

use panic_rtt_target as _;

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use num_traits::float::Float;
use microbit::{Board, hal::{gpio, pwm, time, timer}};

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    #[cfg(feature="ext")]
    let speaker_pin = board.edge.e00;
    #[cfg(not(feature="ext"))]
    let speaker_pin = board.speaker_pin;

    let mut button_a = board.buttons.button_a.into_floating_input();
    let speaker_pin = speaker_pin
        .into_push_pull_output(gpio::Level::Low)
        .degrade();
    let mut timer = timer::Timer::new(board.TIMER0);
    let pwm = pwm::Pwm::new(board.PWM0);

    let div = pwm::Prescaler::Div1;
    let counter_mode = pwm::CounterMode::UpAndDown;
    pwm
        .set_output_pin(pwm::Channel::C0, speaker_pin)
        .set_prescaler(div)
        .set_counter_mode(counter_mode)
        .set_period(time::Hertz(20_000));

    let mut high = true;
    let duty = pwm.max_duty() as f32;
    let width_high = (duty * 0.1).floor() as u16;
    let width_low = (duty * 0.9).floor() as u16;
    loop {
        let width = if button_a.is_high().unwrap() {
            0
        } else if high {
            width_high
        } else {
            width_low
        };
        high = !high;
        pwm.set_duty_off_common(width);

        // 440Hz = 1e6 / 440 / 2 = 1136 us, but subtract about 30us for overhead.
        timer.delay_us(1106);
    }
}
