#![no_std]
#![no_main]

use panic_rtt_target as _;

use core::f32::consts::PI;
use cortex_m_rt::entry;
use libm::sinf;
use microbit::{Board, hal::{gpio, pwm, time}, pac::interrupt};
use rtt_target::{rtt_init_print, rprintln};

/// This has to be in RAM for the PWM unit to access it. It
/// needs to be a 16-bit buffer, independent of sample resolution.
static mut BUFFERS: [[u16; BLOCK_SIZE]; 2] = [[0; BLOCK_SIZE]; 2];

/// Sample rate in samples/sec.
const SAMPLE_RATE: u32 = 20_000;

/// This number is chosen carefully: 20KHz sample rate,
/// 800Hz and 1000Hz are a perfect third, 4::5.
/// After 25 cycles of of 800Hz and 20 cycles of 1000Hz,
/// the joint cycle will be complete.
const BLOCK_SIZE: usize = 20 * SAMPLE_RATE as usize / 1000;

// Safety: `a` must point to valid storage.
unsafe fn fill_array(duty: f32, a: *mut [u16; BLOCK_SIZE]) {
    let a = a as *mut u16;
    for t in 0..BLOCK_SIZE {
        let x1 = 0.5 * sinf(2.0 * PI * (1.0 / 25.0) * t as f32);
        let x2 = 0.5 * sinf(2.0 * PI * (1.0 / 20.0) * t as f32);
        let x = 0.8 * 0.5 * (x1 + x2 + 1.0) * duty;
        unsafe {
            let p = a.add(t);
            *p = x as u16;
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    #[cfg(feature="ext")]
    let speaker_pin = board.edge.e00;
    #[cfg(not(feature="ext"))]
    let speaker_pin = board.speaker_pin;

    let speaker_pin = speaker_pin
        .into_push_pull_output(gpio::Level::Low)
        .degrade();

    let event_seq0_end = pwm::PwmEvent::SeqEnd(pwm::Seq::Seq0);
    let event_seq1_end = pwm::PwmEvent::SeqEnd(pwm::Seq::Seq1);
    let pwm = pwm::Pwm::new(board.PWM0);
    pwm
        // output the waveform on the speaker pin
        .set_output_pin(pwm::Channel::C0, speaker_pin)
        // Prescaler set for 16MHz.
        .set_prescaler(pwm::Prescaler::Div1)
        // Configure for up counter mode.
        .set_counter_mode(pwm::CounterMode::Up)
        // Read duty cycle values from sequence.
        .set_load_mode(pwm::LoadMode::Common)
        // Be sure to be advancing the thing.
        .set_step_mode(pwm::StepMode::Auto)
        // Implicitly set maximum duty cycle = PWM period in ticks.
        .set_period(time::Hertz(SAMPLE_RATE))
        // Set no delay between samples.
        .set_seq_refresh(pwm::Seq::Seq0, 0)
        // Set no delay at end of sequence.
        .set_seq_end_delay(pwm::Seq::Seq0, 0)
        // Set no delay between samples.
        .set_seq_refresh(pwm::Seq::Seq1, 0)
        // Set no delay at end of sequence.
        .set_seq_end_delay(pwm::Seq::Seq1, 0)
        // Enable sample channel.
        .enable_channel(pwm::Channel::C0)
        // Enable sample group.
        .enable_group(pwm::Group::G0)
        // Keep playing forever.
        .loop_inf()
        // Interrupt when done with seq0.
        .enable_interrupt(event_seq0_end)
        // Interrupt when done with seq1.
        .enable_interrupt(event_seq1_end)
        // Enable PWM.
        .enable();

    let duty = pwm.max_duty() as f32;
    rprintln!("{} {}", BLOCK_SIZE, duty);

    let dma = cortex_m::interrupt::free(|_cs| {
        /* Enable PWM interrupts */
        //unsafe { pac::NVIC::unmask(pac::Interrupt::PWM1) };
        //pac::NVIC::unpend(pac::Interrupt::PWM1);
        //rprintln!("unmasked");

        // The `unsafe`s here are to assure the Rust compiler
        // that nothing else is going to mess with this buffer
        // while a mutable reference is held.
        //
        // Safety: Because we are single-threaded, the only
        // thing that can access `SAMPS` once created is the HW
        // PWM unit, and it will be read-only access.

        pwm.reset_event(event_seq0_end);
        pwm.reset_event(event_seq1_end);
        unsafe { 
            #[allow(clippy::needless_range_loop)]
            for i in 0..=1 {
                fill_array(duty, &raw mut BUFFERS[i]);
            }

            // Start the wave.
            pwm.load(Some(&BUFFERS[0]), Some(&BUFFERS[1]), true).unwrap()
        }
    });


    loop {
        while !dma.is_event_triggered(event_seq0_end) && !dma.is_event_triggered(event_seq1_end) {};
        //asm::wfi();
        //rprintln!("awake");
        if dma.is_event_triggered(event_seq0_end) {
            dma.reset_event(event_seq0_end);
        }
        if dma.is_event_triggered(event_seq1_end) {
            dma.reset_event(event_seq1_end);
        }
    }
}

#[interrupt]
fn PWM0() {
    static mut INTERRUPTED: bool = false;
    if !*INTERRUPTED {
        //rprintln!("interrupt");
        *INTERRUPTED = true;
    }
}

