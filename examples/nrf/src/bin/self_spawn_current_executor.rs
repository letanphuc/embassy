#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{info, unwrap};
use embassy_executor::executor::Spawner;
use embassy_executor::time::{Duration, Timer};
use embassy_nrf::Peripherals;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task(pool_size = 2)]
async fn my_task(n: u32) {
    Timer::after(Duration::from_secs(1)).await;
    info!("Spawning self! {}", n);
    unwrap!(Spawner::for_current_executor().await.spawn(my_task(n + 1)));
}

#[embassy_executor::main]
async fn main(spawner: Spawner, _p: Peripherals) {
    info!("Hello World!");
    unwrap!(spawner.spawn(my_task(0)));
}
