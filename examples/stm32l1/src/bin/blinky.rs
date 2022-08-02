#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::executor::Spawner;
use embassy_executor::time::{Duration, Timer};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::Peripherals;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    let mut led = Output::new(p.PA12, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after(Duration::from_millis(1000)).await;

        info!("low");
        led.set_low();
        Timer::after(Duration::from_millis(1000)).await;
    }
}
