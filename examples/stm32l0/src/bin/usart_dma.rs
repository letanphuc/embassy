#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::executor::Spawner;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::Peripherals;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let mut usart = Uart::new(p.USART1, p.PB7, p.PB6, p.DMA1_CH2, p.DMA1_CH3, Config::default());

    usart.write(b"Hello Embassy World!\r\n").await.unwrap();
    info!("wrote Hello, starting echo");

    let mut buf = [0; 1];
    loop {
        usart.read(&mut buf[..]).await.unwrap();
        usart.write(&buf[..]).await.unwrap();
    }
}
