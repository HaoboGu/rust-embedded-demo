#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{AnyPin, Input, Level, Output, Pin, Pull},
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

// Declare async tasks
#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, embassy_stm32::gpio::Speed::High);

    loop {
        // Timekeeping is globally available, no need to mess with hardware timers.
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(150).await;
    }
}

// Main is itself an async task as well.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize the embassy-stm32 HAL.
    let p = embassy_stm32::init(Default::default());

    // Spawned tasks run in the background, concurrently.
    spawner.spawn(blink(p.PE3.degrade())).unwrap();

    let mut button = ExtiInput::new(Input::new(p.PC13, Pull::Down), p.EXTI13);
    loop {
        // Asynchronously wait for GPIO events, allowing other tasks
        // to run, or the core to sleep.
        button.wait_for_rising_edge().await;
        info!("Button pressed!");
        button.wait_for_falling_edge().await;
        info!("Button released!");
    }
}
