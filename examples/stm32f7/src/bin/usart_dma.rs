#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;

use defmt::*;
use embassy_executor::executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::Peripherals;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let config = Config::default();
    let mut usart = Uart::new(p.UART7, p.PA8, p.PA15, p.DMA1_CH1, NoDma, config);

    for n in 0u32.. {
        let mut s: String<128> = String::new();
        core::write!(&mut s, "Hello DMA World {}!\r\n", n).unwrap();

        unwrap!(usart.write(s.as_bytes()).await);

        info!("wrote DMA");
    }
}
