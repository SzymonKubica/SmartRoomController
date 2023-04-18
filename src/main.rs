#![no_std]
#![no_main]

use core::panic::PanicInfo;


extern crate ds323x;
extern crate embedded_hal;
extern crate arduino_hal;
extern crate ag_lcd;
extern crate numtoa;
use ag_lcd::{Display, Blink, Cursor, LcdDisplay};
use ds323x::{Ds323x, NaiveDate, DateTimeAccess, Timelike};
use keypad::Keypad;
use numtoa::NumToA;
mod keypad;


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
    let d3 = pins.d3.into_output().downgrade();
    let d4 = pins.d4.into_output().downgrade();
    let d5 = pins.d5.into_output().downgrade();
    let d6 = pins.d6.into_output().downgrade();
    let d7 = pins.d7.into_output().downgrade();

    let mosi = pins.d11.into_output();
    let sclk = pins.d13.into_output();
    let miso = pins.d12.into_pull_up_input();
    let cs = pins.d10.into_output();
    let d2 = pins.d2.into_floating_input(); // Pin for the lcd backlight

    let mut spi = arduino_hal::Spi::new(peripherals.SPI, sclk, mosi, miso, cs, Default::default());
    let mut i2c = arduino_hal::I2c::new(
        peripherals.TWI, //
        pins.a4.into_pull_up_input(), // use respective pins
        pins.a5.into_pull_up_input(),
        50000,
    );
    let mut rtc = Ds323x::new_ds3231(i2c);
    let datetime = NaiveDate::from_ymd_opt(2023, 4, 18).unwrap().and_hms_opt(12, 55, 58).unwrap();
    //rtc.set_datetime(&datetime).unwrap();
    let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
        .with_half_bus(d4, d5, d6, d7)
        .with_display(Display::On)
        .with_blink(Blink::On)
        .with_cursor(Cursor::On)
        .build();

    lcd.print("Test message!");

    let mut adc = arduino_hal::Adc::new(peripherals.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut adc);
    let mut keypad = Keypad::new(a0, adc);

    loop {
        let selection = keypad.get_input();
        if let Some(input) = selection {
           lcd.clear();
           let date = rtc.datetime();
           match date {
            Ok(date_time) => {
                let hours = date_time.hour();
                let minutes = date_time.minute();
                let seconds = date_time.second();
                let mut buf = [0u8;16];
                let hours = hours.numtoa_str(10, &mut buf);
                let mut buf = [0u8;16];
                let minutes = minutes.numtoa_str(10, &mut buf);
                let mut buf = [0u8;16];
                let seconds = seconds.numtoa_str(10, &mut buf);
                lcd.print("****");
                if let 1 = hours.len() {
                 lcd.print("0");
                }
                lcd.print(hours);
                if let 1 = minutes.len() {
                 lcd.print(":0");
                } else {
                 lcd.print(":");
                }
                lcd.print(minutes);
                if let 1 = seconds.len() {
                 lcd.print(":0");
                } else {
                 lcd.print(":");
                }
                lcd.print(seconds);
                lcd.print("****");
            },
            Err(error) => {
                lcd.print("error");
            },
           }
           lcd.set_position(0, 2);
           lcd.print(input.to_string());
        }

        arduino_hal::delay_ms(100);
    }
   }
}

