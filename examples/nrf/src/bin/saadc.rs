#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use embassy_executor::executor::Spawner;
use embassy_executor::time::{Duration, Timer};
use embassy_nrf::saadc::{ChannelConfig, Config, Saadc};
use embassy_nrf::{interrupt, Peripherals};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner, mut p: Peripherals) {
    let config = Config::default();
    let channel_config = ChannelConfig::single_ended(&mut p.P0_02);
    let mut saadc = Saadc::new(p.SAADC, interrupt::take!(SAADC), config, [channel_config]);

    loop {
        let mut buf = [0; 1];
        saadc.sample(&mut buf).await;
        info!("sample: {=i16}", &buf[0]);
        Timer::after(Duration::from_millis(100)).await;
    }
}
