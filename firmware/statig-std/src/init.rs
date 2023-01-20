use esp_idf_hal::adc::PoweredAdc;
use esp_idf_hal::adc::{self, Atten11dB};
use esp_idf_hal::prelude::*;
use esp_idf_hal::units::FromValueType;
use esp_idf_hal::{gpio, i2c};

use sx1509;

pub struct Board {
    pub i2c1: i2c::Master<i2c::I2C0, gpio::Gpio1<gpio::InputOutput>, gpio::Gpio10<gpio::Output>>,
    pub adc1: PoweredAdc<adc::ADC1>,
    pub adc2: PoweredAdc<adc::ADC2>,
    pub adc1_ch0: gpio::Gpio0<Atten11dB<adc::ADC1>>,
    // pub adc1_ch1: gpio::Gpio1<Atten11dB<adc::ADC1>>,
    pub adc1_ch2: gpio::Gpio2<Atten11dB<adc::ADC1>>,
    pub adc1_ch3: gpio::Gpio3<Atten11dB<adc::ADC1>>,
    pub adc1_ch4: gpio::Gpio4<Atten11dB<adc::ADC1>>,
    pub adc2_ch0: gpio::Gpio5<Atten11dB<adc::ADC2>>,
    pub gpio_exp: sx1509::Sx1509<
        i2c::Master<i2c::I2C0, gpio::Gpio1<gpio::InputOutput>, gpio::Gpio10<gpio::Output>>,
    >,
    pub psh_btn: gpio::Gpio9<gpio::Input>,
    pub led: gpio::Gpio8<gpio::Output>,
}

impl Board {
    pub fn init(p: Peripherals) -> Board {
        // GPIO
        let psh_btn = p.pins.gpio9.into_input().unwrap();
        let led = p.pins.gpio8.into_output().unwrap();

        // I2C
        let sda = p.pins.gpio1.into_input_output().unwrap();
        let scl = p.pins.gpio10.into_output().unwrap();
        let i2c = p.i2c0;
        let config = <i2c::config::MasterConfig as Default>::default().baudrate(400.kHz().into());
        let mut i2c1 =
            i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config).unwrap();

        // GPIO expander
        let mut expander = sx1509::Sx1509::new(&mut i2c1, sx1509::DEFAULT_ADDRESS);
        match expander.borrow(&mut i2c1).software_reset() {
            Ok(_) => log::info!("SX1509 reset"),
            Err(_) => log::info!("SX1509 error resetting"),
        }
        match expander.borrow(&mut i2c1).set_bank_a_direction(0) {
            Ok(_) => log::info!("SX1509 set bank A success"),
            Err(_) => log::info!("SX1509 error set bank A"),
        }
        match expander.borrow(&mut i2c1).set_bank_b_direction(0xFF) {
            Ok(_) => log::info!("SX1509 set bank B success"),
            Err(_) => log::info!("SX1509 error set bank B"),
        }

        // ADC
        let config = adc::config::Config::new().calibration(true);

        let adc1_ch0 = p.pins.gpio0.into_analog_atten_11db().unwrap();
        // let adc1_ch1 = p.pins.gpio1.into_analog_atten_11db().unwrap();
        let adc1_ch2 = p.pins.gpio2.into_analog_atten_11db().unwrap();
        let adc1_ch3 = p.pins.gpio3.into_analog_atten_11db().unwrap();
        let adc1_ch4 = p.pins.gpio4.into_analog_atten_11db().unwrap();
        let adc2_ch0 = p.pins.gpio5.into_analog_atten_11db().unwrap();

        let adc1 = PoweredAdc::new(p.adc1, config).unwrap();
        let adc2 = PoweredAdc::new(p.adc2, config).unwrap();

        Board {
            i2c1: i2c1,
            adc1: adc1,
            adc2: adc2,
            adc1_ch0: adc1_ch0,
            // adc1_ch1: adc1_ch1,
            adc1_ch2: adc1_ch2,
            adc1_ch3: adc1_ch3,
            adc1_ch4: adc1_ch4,
            adc2_ch0: adc2_ch0,
            gpio_exp: expander,
            psh_btn: psh_btn,
            led: led,
        }
    }
}
