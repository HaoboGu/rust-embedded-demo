#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32h7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // 获取cortex核心外设和stm32h7的所有外设
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Power 设置
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();
    
    // 初始化RCC
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(200.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // 设置LED对应的GPIO
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let mut led = gpioe.pe3.into_push_pull_output();

    // cortex-m已经实现好了delay函数，直接拿到，下面使用
    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        // 点灯
        led.toggle();
        // 延时500ms
        delay.delay_ms(500_u16);
    }
}
