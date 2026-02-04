#![no_std]
#![no_main]
#![deny(warnings)] //Living on the wild side
#![allow(unused_macros)]

use defmt_rtt as _;
use defmt::info;
use panic_probe as _;

use stm32h7xx_hal::{block, prelude::*, timer::Timer};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    info!("Starting blinky");

    let dp =stm32h7xx_hal::stm32::Peripherals::take().unwrap();

    let rcc =dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    let mut ld1 = gpiob.pb0.into_push_pull_output();
    let mut ld2 = gpioe.pe1.into_push_pull_output();
    let mut ld3 = gpiob.pb14.into_push_pull_output();
    ld1.set_low();
    ld2.set_high();
    ld3.set_low();
    let mut timer = Timer::tim1(dp.TIM1, ccdr.peripheral.TIM1, &ccdr.clocks);
    //let mut timer2 = Timer::tim2(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);
    timer.start(2.Hz());
    //timer2.start(4.Hz());

    info!("Entering main loop");

    loop{
        ld1.set_high();
        ld2.set_low();
        ld3.set_high();
        block!(timer.wait()).unwrap();
        ld1.set_low();
        ld2.set_high();
        ld3.set_low();
        //block!(timer2.wait()).unwrap();
    }
}
