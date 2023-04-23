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
use core::str::{from_utf8, Utf8Error};
use ds323x::{
    ic::DS3231, interface::I2cInterface, DateTimeAccess, Ds323x, Error, NaiveDate, NaiveDateTime,
    Timelike,
};
use numtoa::NumToA;

pub struct Clock {
    rtc: Ds323x<I2cInterface<I2c<MHz16>>, DS3231>,
}

impl Clock {
    pub fn new(twi: TWI, a4: Pin<Input<Floating>, PC4>, a5: Pin<Input<Floating>, PC5>) -> Self {
        // twi is the Two-Wire interface to components on a two-wire bus:
        // the clock line and data line. It is used to establish I2c.
        let i2c = arduino_hal::I2c::new(
            twi,
            a4.into_pull_up_input(), // use respective pins
            a5.into_pull_up_input(),
            50000,
        );
        let rtc = Ds323x::new_ds3231(i2c);
        Self { rtc }
    }

    pub fn set_time(&mut self, time: NaiveDateTime) {
        self.rtc.set_datetime(&time).unwrap();
    }

    pub fn is_running(&mut self) -> Result<bool, ds323x::Error<arduino_hal::i2c::Error, ()>> {
        self.rtc.running()
    }

    pub fn format_time(date_time: NaiveDateTime, output_buffer: &mut [u8; 8]) {
        let mut buffer_index = 0;
        let mut buf = [0u8; 16];
        // Write the hour into the output buffer.
        let hour = date_time.hour().numtoa_str(10, &mut buf);

        // If the hour is a single-digit number we need to prepend a 0.
        if hour.len() == 1 {
            '0'.encode_utf8(output_buffer);
            buffer_index += 1;
        };
        hour.chars().for_each(|c| {
            c.encode_utf8(&mut output_buffer[buffer_index..]);
            buffer_index += 1;
        });
        ':'.encode_utf8(&mut output_buffer[buffer_index..]);
        buffer_index += 1;

        let mut buf = [0u8; 16];
        let minutes = date_time.minute().numtoa_str(10, &mut buf);
        if minutes.len() == 1 {
            '0'.encode_utf8(&mut output_buffer[buffer_index..]);
            buffer_index += 1;
        }
        minutes.chars().for_each(|c| {
            c.encode_utf8(&mut output_buffer[buffer_index..]);
            buffer_index += 1;
        });
        ':'.encode_utf8(&mut output_buffer[buffer_index..]);
        buffer_index += 1;

        let mut buf = [0u8; 16];
        let seconds = date_time.second().numtoa_str(10, &mut buf);
        if seconds.len() == 1 {
            '0'.encode_utf8(&mut output_buffer[buffer_index..]);
            buffer_index += 1;
        }
        seconds.chars().for_each(|c| {
            c.encode_utf8(&mut output_buffer[buffer_index..]);
            buffer_index += 1;
        });
    }
    pub fn get_time(
        &mut self,
        output_buffer: &mut [u8; 8],
    ) -> Result<(), ds323x::Error<arduino_hal::i2c::Error, ()>> {
        let date_time = self.rtc.datetime()?;
        Self::format_time(date_time, output_buffer);
        Ok(())
    }
}
