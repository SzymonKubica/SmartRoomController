#![no_std]
#![no_main]

use core::panic::PanicInfo;


extern crate ds323x;
extern crate embedded_hal;
extern crate ag_lcd;
use ag_lcd::{Display, Blink, Cursor, LcdDisplay};


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    loop {

    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let delay = arduino_hal::Delay::new();

    let rs = pins.d8.into_output().downgrade();
    let en = pins.d9.into_output().downgrade();
    let d4 = pins.d4.into_output().downgrade();
    let d5 = pins.d5.into_output().downgrade();
    let d6 = pins.d6.into_output().downgrade();
    let d7 = pins.d7.into_output().downgrade();

    let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
        .with_half_bus(d4, d5, d6, d7)
        .with_display(Display::On)
        .with_blink(Blink::On)
        .with_cursor(Cursor::On)
        .build();

    lcd.set_cursor(Cursor::Off);
    lcd.set_blink(Blink::Off);

    lcd.print("Test message!");

       loop {}
   }
}

