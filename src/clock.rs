use core::str::from_utf8;
use arduino_hal::{
    clock::MHz16,
    hal::{
        port::{PC4, PC5},
        I2c,
    },
    pac::TWI,
    port::{
        mode::{Floating, Input},
        Pin,
    },
};
use ds323x::{ic::DS3231, interface::I2cInterface, DateTimeAccess, Ds323x, Timelike};
use numtoa::NumToA;

pub struct Clock {
    rtc: Ds323x<I2cInterface<I2c<MHz16>>, DS3231>,
}

impl Clock {
    pub fn new(twi: TWI, a4: Pin<Input<Floating>, PC4>, a5: Pin<Input<Floating>, PC5>) -> Self{
        // twi is the Two-Wire interface to components on a two-wire bus:
        // the clock line and data line. It is used to establish I2c.
        let i2c = arduino_hal::I2c::new(
            twi,
            a4.into_pull_up_input(), // use respective pins
            a5.into_pull_up_input(),
            50000,
        );
        let mut rtc = Ds323x::new_ds3231(i2c);
        Self {rtc}
    }

    pub fn get_date(&mut self) -> Option<&str> {
        let date = self.rtc.datetime();

        match date {
            Ok(date_time) => {
                let mut output_buffer = [0u8; 16];
                let mut buf = [0u8; 16];
                // Write the hour into the output buffer.
                let hour = date_time.hour().numtoa_str(10, &mut buf);

                // If the hour is a single-digit number we need to prepend a 0.
                if hour.len() == 1 {
                    '0'.encode_utf8(&mut output_buffer);
                }
                hour.chars().fold(output_buffer, |mut output_buffer, c| c.encode_utf8(&mut output_buffer));
                ':'.encode_utf8(&mut output_buffer);

                let mut buf = [0u8; 16];
                let minutes = date_time.minute().numtoa_str(10, &mut buf);
                if minutes.len() == 1 {
                    '0'.encode_utf8(&mut output_buffer);
                }
                minutes.chars().map(|c| c.encode_utf8(&mut output_buffer));
                ':'.encode_utf8(&mut output_buffer);

                let mut buf = [0u8; 16];
                let seconds = date_time.second().numtoa_str(10, &mut buf);
                if seconds.len() == 1 {
                    '0'.encode_utf8(&mut output_buffer);
                }
                seconds.chars().map(|c| c.encode_utf8(&mut output_buffer));
                match from_utf8(&output_buffer) {
                    Ok(assembled_string) => Some(assembled_string),
                    Err(_) => None,
                }
            }
            Err(_) => {
                None
            }
        }
    }
}
