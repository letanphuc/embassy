#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_stm32::pwm::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::pwm::Channel;
use embassy_stm32::time::{khz, mhz};
use embassy_stm32::{Config, Peripherals};
use {defmt_rtt as _, panic_probe as _};

pub fn config() -> Config {
    let mut config = Config::default();
    config.rcc.sys_ck = Some(mhz(400));
    config.rcc.hclk = Some(mhz(400));
    config.rcc.pll1.q_ck = Some(mhz(100));
    config.rcc.pclk1 = Some(mhz(100));
    config.rcc.pclk2 = Some(mhz(100));
    config.rcc.pclk3 = Some(mhz(100));
    config.rcc.pclk4 = Some(mhz(100));
    config
}

#[embassy::main(config = "config()")]
async fn main(_spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    let ch1 = PwmPin::new_ch1(p.PA6);
    let mut pwm = SimplePwm::new(p.TIM3, Some(ch1), None, None, None, khz(10));
    let max = pwm.get_max_duty();
    pwm.enable(Channel::Ch1);

    info!("PWM initialized");
    info!("PWM max duty {}", max);

    loop {
        pwm.set_duty(Channel::Ch1, 0);
        Timer::after(Duration::from_millis(300)).await;
        pwm.set_duty(Channel::Ch1, max / 4);
        Timer::after(Duration::from_millis(300)).await;
        pwm.set_duty(Channel::Ch1, max / 2);
        Timer::after(Duration::from_millis(300)).await;
        pwm.set_duty(Channel::Ch1, max - 1);
        Timer::after(Duration::from_millis(300)).await;
    }
}
