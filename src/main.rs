#![no_std]
#![no_main]

use core::{panic::PanicInfo, str::from_utf8};

extern crate ag_lcd;
extern crate arduino_hal;
extern crate ds323x;
extern crate embedded_hal;
extern crate numtoa;
use ag_lcd::{Blink, Cursor, Display, LcdDisplay, Lines};
use clock::Clock;
use ds323x::{DateTimeAccess, Ds323x, NaiveDate, Timelike};
use keypad::{Keypad, KeypadInput};
use numtoa::NumToA;

mod clock;
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

        let mut spi =
            arduino_hal::Spi::new(peripherals.SPI, sclk, mosi, miso, cs, Default::default());
        let mut lcd: LcdDisplay<_, _> = LcdDisplay::new(rs, en, delay)
            .with_half_bus(d4, d5, d6, d7)
            .with_display(Display::On)
            .with_blink(Blink::On)
            .with_lines(Lines::TwoLines)
            .with_cursor(Cursor::On)
            .build();

        let mut keypad = Keypad::new(peripherals.ADC, pins.a0);

        let datetime = NaiveDate::from_ymd_opt(2023, 4, 23)
            .unwrap()
            .and_hms_opt(9, 55, 58)
            .unwrap();
        let mut clock = Clock::new(peripherals.TWI, pins.a4, pins.a5);
        //clock.set_time(datetime);
        lcd.print("Test message.");

        let mut input: Option<KeypadInput> = None;
        let mut counter: u8 = 0;
        loop {
            let mut buf = [0u8; 8];
            let date_result = clock.get_time(&mut buf);
            if counter == 0 {
                lcd.clear();
                if let Err(_) = date_result {
                    lcd.print("Error");
                } else {
                    match from_utf8(&buf) {
                        Ok(output_str) => lcd.print(output_str),
                        Err(_) => lcd.print("Error str "),
                    }
                }

                if let Some(value) = input {
                    lcd.set_position(0, 1);
                    lcd.print(value.to_string());
                } else {
                    lcd.set_position(0, 1);
                    lcd.print("Press a button.");
                }
            }
            // If input detected update the selected value.
            if let Some(value) = keypad.get_input() {
                match input {
                    Some(current_value) => {
                        if value != current_value {
                            input = Some(value);
                        }
                    }
                    None => {
                        input = Some(value);
                    }
                }
            }
            counter = (counter + 1) % 10;

            arduino_hal::delay_ms(100);
        }
    }
}
