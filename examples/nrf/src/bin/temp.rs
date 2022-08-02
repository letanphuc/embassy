#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use embassy_executor::executor::Spawner;
use embassy_executor::time::{Duration, Timer};
use embassy_nrf::temp::Temp;
use embassy_nrf::{interrupt, Peripherals};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let irq = interrupt::take!(TEMP);
    let mut temp = Temp::new(p.TEMP, irq);

    loop {
        let value = temp.read().await;
        info!("temperature: {}℃", value.to_num::<u16>());
        Timer::after(Duration::from_secs(1)).await;
    }
}
