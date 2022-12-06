#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::buffered_uarte::{BufferedUarte, State};
use embassy_nrf::{interrupt, uarte};
use embedded_io::asynch::{BufRead, Write};
use futures::pin_mut;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;

    let mut tx_buffer = [0u8; 4096];
    let mut rx_buffer = [0u8; 4096];

    let irq = interrupt::take!(UARTE0_UART0);
    let mut state = State::new();
    // Please note - important to have hardware flow control (https://github.com/embassy-rs/embassy/issues/536)
    let u = BufferedUarte::new(
        &mut state,
        p.UARTE0,
        p.TIMER0,
        p.PPI_CH0,
        p.PPI_CH1,
        irq,
        p.P0_08,
        p.P0_06,
        p.P0_07,
        p.P0_05,
        config,
        &mut rx_buffer,
        &mut tx_buffer,
    );
    pin_mut!(u);

    info!("uarte initialized!");

    unwrap!(u.write_all(b"Hello!\r\n").await);
    info!("wrote hello in uart!");

    loop {
        info!("reading...");
        let buf = unwrap!(u.fill_buf().await);
        info!("read done, got {}", buf);

        // Read bytes have to be explicitly consumed, otherwise fill_buf() will return them again
        let n = buf.len();
        u.consume(n);
    }
}
