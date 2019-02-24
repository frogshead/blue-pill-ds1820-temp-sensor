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

use hal::prelude::*;
use hal::delay::Delay;
use hal::stm32;
use rt::{entry, exception, ExceptionFrame};

extern crate cortex_m_semihosting;

use cortex_m_semihosting::hio;
use core::fmt::Write;

use onewire::DS18B20;



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


    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    


    let mut onewire_pin = gpiob.pb12.into_open_drain_output(&mut gpiob.crh).downgrade();
    let mut delay = Delay::new(cp.SYST, clocks);
    
    let mut _onewire = onewire::OneWire::new(&mut onewire_pin, false);
    let mut search = onewire::DeviceSearch::new_for_family(0x28);
    let device =  _onewire.search_next(&mut search ,&mut delay).unwrap();
    let ds = DS18B20::new(device.unwrap()).unwrap();

    

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    loop {
        delay.delay_ms(500 as u16);
        ds.measure_temperature(&mut _onewire, &mut delay).unwrap();
        let temp: u16 = ds.read_temperature(&mut _onewire, &mut delay).unwrap();
        let t = onewire::ds18b20::split_temp(temp);
        write!(stdout, "Temperature: {0}.{1} \n" , t.0, t.1).unwrap();
        led.set_high();
        delay.delay_ms(500 as u16);
        
        led.set_low();

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
