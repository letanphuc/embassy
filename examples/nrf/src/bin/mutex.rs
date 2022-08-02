#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{info, unwrap};
use embassy_executor::executor::Spawner;
use embassy_executor::time::{Duration, Timer};
use embassy_nrf::Peripherals;
use embassy_util::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_util::mutex::Mutex;
use {defmt_rtt as _, panic_probe as _};

static MUTEX: Mutex<ThreadModeRawMutex, u32> = Mutex::new(0);

#[embassy_executor::task]
async fn my_task() {
    loop {
        {
            let mut m = MUTEX.lock().await;
            info!("start long operation");
            *m += 1000;

            // Hold the mutex for a long time.
            Timer::after(Duration::from_secs(1)).await;
            info!("end long operation: count = {}", *m);
        }

        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner, _p: Peripherals) {
    unwrap!(spawner.spawn(my_task()));

    loop {
        Timer::after(Duration::from_millis(300)).await;
        let mut m = MUTEX.lock().await;
        *m += 1;
        info!("short operation: count = {}", *m);
    }
}
