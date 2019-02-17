//! Blinks an LED

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;
extern crate onewire;

// #[macro_use(block)]
// extern crate nb;

use hal::prelude::*;
use hal::delay::Delay;
use hal::stm32;
//use hal::timer::Timer;
use rt::{entry, exception, ExceptionFrame};

extern crate cortex_m_semihosting;

use cortex_m_semihosting::hio;
use core::fmt::Write;

use onewire::DS18B20;
use onewire::ds18b20;


#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut stdout = hio::hstdout().unwrap();
    write!(stdout, "Rust on embedded is 1!\n").unwrap();

    // Try a different clock configuration
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let clocks = rcc.cfgr
    //     .sysclk(64.mhz())
    //     .pclk1(32.mhz())
    //     .freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    


    let mut onewire_pin = gpiob.pb12.into_open_drain_output(&mut gpiob.crh).downgrade();
    let mut delay = Delay::new(cp.SYST, clocks);
    
    let mut _onewire = onewire::OneWire::new(&mut onewire_pin, false);
    let mut search = onewire::DeviceSearch::new();
    while let Some(device) = _onewire.search_next(&mut search ,&mut delay).unwrap(){
        match device.address[0]{
            ds18b20::FAMILY_CODE => {
                let ds18b20 = DS18B20::new(device).unwrap();
                let resolution = ds18b20.measure_temperature(&mut _onewire, &mut delay).unwrap();
                delay.delay_ms(resolution.time_ms());
                loop{
                  
                    ds18b20.measure_temperature(&mut _onewire, &mut delay).unwrap();
                    write!(stdout, "Temperature: {}\n", ds18b20.read_temperature(&mut _onewire, &mut delay).unwrap()).unwrap();
                }    
            },
            _ => {
                write!(stdout, "Can't read temperature!").unwrap();

            }


            }
        }

    

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);


    //let mut timer = Timer::syst(cp.SYST, 10.hz(), clocks);
    // Try a different timer (even SYST)
    loop {
        // block!(timer.wait()).unwrap();
        // led.set_high();
        // block!(timer.wait()).unwrap();
        // led.set_low();
        led.set_high();
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
