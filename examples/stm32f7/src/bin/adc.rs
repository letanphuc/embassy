#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::executor::Spawner;
use embassy_executor::time::{Delay, Duration, Timer};
use embassy_stm32::adc::Adc;
use embassy_stm32::Peripherals;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    let mut adc = Adc::new(p.ADC1, &mut Delay);
    let mut pin = p.PA3;

    loop {
        let v = adc.read(&mut pin);
        info!("--> {} - {} mV", v, adc.to_millivolts(v));
        Timer::after(Duration::from_millis(100)).await;
    }
}
