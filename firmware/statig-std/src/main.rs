use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use core::fmt::Debug;
use embedded_hal::{adc::OneShot, digital::v2::OutputPin};
use esp_idf_hal::{
    adc::{self, Atten11dB, PoweredAdc},
    gpio,
    prelude::*,
};

use statig::prelude::*;
// use statig::StateOrSuperstate;

use std::thread;
use std::time::Duration;

mod init;

static BLINKY_STACK_SIZE: usize = 5000;
static ADC_READER_STACK_SIZE: usize = 5000;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let board = init::Board::init(peripherals);

    let mut state_machine = Blinky { board }.state_machine().init();


    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || {
            loop{
            thread::sleep(Duration::from_millis(2000));
            state_machine.handle(&Event::TimerElapsed);
            thread::sleep(Duration::from_millis(2000));
            state_machine.handle(&Event::TimerElapsed);
            }
        }).unwrap();


//    let _adc_thread = std::thread::Builder::new().stack_size(ADC_READER_STACK_SIZE).spawn(move||{
//        loop{
//            thread::sleep(Duration::from_millis(100));
//            println!("adc thread");
//        }
//    }).unwrap();

    loop {
        // Don't let the idle task starve and trigger warnings from the watchdog.
        std::thread::sleep(Duration::from_millis(100));
    }
}
// #[derive(Debug, Default)]
pub struct Blinky {
    board: init::Board,
}

// The event that will be handled by the state machine.
#[derive(Debug)]
pub enum Event {
    TimerElapsed,
}

#[state_machine(
    initial = "State::led_on()",
    state(derive(Debug)),
    on_transition = "Self::on_transition"
)]
impl Blinky {
    #[action]
    fn enter_on(&mut self) {
        self.board.led.set_high().unwrap();
    }

    #[state(entry_action = "enter_on")]
    fn led_on(&mut self, event: &Event) -> Response<State> {
        //let mut adc_reading = self.board.adc1.read(&mut self.board.adc1_ch0).unwrap();
        //println!("ADC1_0: {}", adc_reading);
        //// adc_reading = self.board.adc1.read(&mut self.board.adc1_ch1).unwrap();
        //// println!("ADC1_1: {}", adc_reading);
        //adc_reading = self.board.adc1.read(&mut self.board.adc1_ch2).unwrap();
        //println!("ADC1_2: {}", adc_reading);
        //adc_reading = self.board.adc1.read(&mut self.board.adc1_ch3).unwrap();
        //println!("ADC1_3: {}", adc_reading);
        //adc_reading = self.board.adc1.read(&mut self.board.adc1_ch4).unwrap();
        //println!("ADC1_4: {}", adc_reading);
        //adc_reading = self.board.adc2.read(&mut self.board.adc2_ch0).unwrap();
        //println!("ADC2_0: {}", adc_reading);
        match event {
            // When we receive a `TimerElapsed` event we transition to the `led_off` state.
            Event::TimerElapsed => Transition(State::led_off()),
            // Other events are deferred to the superstate, in this case `blinking`.
            _ => Super,
        }
    }

    #[action]
    fn enter_off(&mut self) {
        self.board.led.set_low().unwrap();
    }

    #[state(entry_action = "enter_off")]
    fn led_off(&mut self, event: &Event) -> Response<State> {

        match event {
            Event::TimerElapsed => Transition(State::led_on()),
            _ => Super,
        }
    }

}

impl Blinky {
    // The `on_transition` callback that will be called after every transition.
    fn on_transition(&mut self, source: &State, target: &State) {
        println!("transitioned from `{:?}` to `{:?}`", source, target);
    }
}
