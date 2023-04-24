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
use ds323x::{
    ic::DS3231, interface::I2cInterface, DateTimeAccess, Ds323x,  NaiveDateTime,
};

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

    pub fn get_time(
        &mut self,
        output_buffer: &mut [u8; 8],
    ) -> Result<NaiveDateTime, ds323x::Error<arduino_hal::i2c::Error, ()>> {
        let date_time = self.rtc.datetime()?;
        Ok(date_time)
    }
}
