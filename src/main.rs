#![no_std]
#![no_main]
#![feature(let_else)]

extern crate ag_lcd;
extern crate arduino_hal;
extern crate ds323x;
extern crate embedded_hal;
extern crate numtoa;

mod clock;
mod controller;
mod display;
mod keypad;
mod time_selector;

use ag_lcd::{Blink, Cursor, Display, LcdDisplay, Lines};
use clock::Clock;
use controller::Controller;
use core::panic::PanicInfo;
use display::ShieldDisplay;
use ds323x::NaiveDate;
use keypad::Keypad;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    loop {
        let peripherals = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(peripherals);

        let mosi = pins.d11.into_output();
        let sclk = pins.d13.into_output();
        let miso = pins.d12.into_pull_up_input();
        let cs = pins.d10.into_output();

        let mut spi =
            arduino_hal::Spi::new(peripherals.SPI, sclk, mosi, miso, cs, Default::default());

        // Initialise the LCD display
        let delay = arduino_hal::Delay::new();
        let rs = pins.d8.into_output().downgrade();
        let en = pins.d9.into_output().downgrade();
        let d4 = pins.d4.into_output().downgrade();
        let d5 = pins.d5.into_output().downgrade();
        let d6 = pins.d6.into_output().downgrade();
        let d7 = pins.d7.into_output().downgrade();
        let mut display: ShieldDisplay = ShieldDisplay::new(
            LcdDisplay::new(rs, en, delay)
                .with_half_bus(d4, d5, d6, d7)
                .with_display(Display::On)
                .with_blink(Blink::Off)
                .with_lines(Lines::TwoLines)
                .with_cursor(Cursor::Off)
                .build(),
        );
        let mut controller = Controller::new(Keypad::new(peripherals.ADC, pins.a0));

        let date_time = NaiveDate::from_ymd_opt(2023, 4, 23)
            .unwrap()
            .and_hms_opt(9, 55, 58)
            .unwrap();
        let mut clock = Clock::new(peripherals.TWI, pins.a4, pins.a5);
        //clock.set_time(date_time);
        display.print_first_line("Test message");

        let mut counter: u8 = 0;
        let mut buf = [0u8; 8];
        ShieldDisplay::format_time(date_time, &mut buf);
        display.clear();
        display.print_time_centered(date_time);
        display.print_second_line("Press a button.");
        loop {
            //let date_result = clock.get_time(&mut buf);
            /*
            match date_result {
                Ok(datetime) => ShieldDisplay::format_time(datetime, &mut buf),
                Err(_) => ,
            }*/

            let current_selection = controller.get_stored_input();
            let new_reading = controller.read_persistent_input();
            let Some(selection) = current_selection else { continue; };
            let Some(input) = new_reading else { continue; };

            if selection != input {
                // If we have detected a change in the input, only then do we
                // want to clear the display and update the selection.
                display.clear();
                display.print_second_line(input.to_string());
            }
        }
    }
}
