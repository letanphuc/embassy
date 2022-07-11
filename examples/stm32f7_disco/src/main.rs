#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::peripherals::PI1;
use embassy_stm32::Peripherals;
use {defmt_rtt as _, panic_probe as _};

#[embassy::task]
async fn blinker(mut led: Output<'static, PI1>, interval: Duration) {
    loop {
        led.set_high();
        Timer::after(interval).await;
        led.set_low();
        Timer::after(interval).await;
    }
}

#[embassy::main]
async fn main(spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    let led = Output::new(p.PI1, Level::High, Speed::Low);
    unwrap!(spawner.spawn(blinker(led, Duration::from_millis(50))));

    let button = Input::new(p.PI11, Pull::Down);
    let mut button = ExtiInput::new(button, p.EXTI11);

    info!("Press the USER button...");

    loop {
        button.wait_for_rising_edge().await;
        info!("Pressed!");
        button.wait_for_falling_edge().await;
        info!("Released!");
    }
}
