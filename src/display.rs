use core::str::from_utf8;

use ag_lcd::LcdDisplay;
use arduino_hal::{
    clock::MHz16,
    hal::port::Dynamic,
    port::{mode::Output, Pin},
    Delay,
};
use ds323x::{NaiveDateTime, Timelike};
use numtoa::NumToA;

pub struct ShieldDisplay {
    lcd: LcdDisplay<Pin<Output, Dynamic>, Delay>,
}

impl ShieldDisplay {
    pub fn new(lcd: LcdDisplay<Pin<Output, Dynamic>, Delay>) -> Self {
        Self { lcd }
    }

    pub fn clear(&mut self) {
        self.lcd.clear();
    }

    pub fn print(&mut self, message: &str) {
        self.lcd.print(message);
    }

    pub fn print_first_line(&mut self, message: &str) {
        self.lcd.set_position(0, 0);
        self.lcd.print(message);
    }

    pub fn print_second_line(&mut self, message: &str) {
        self.lcd.set_position(0, 1);
        self.lcd.print(message);
    }

    pub fn print_time_centered(&mut self, date_time: NaiveDateTime) {
        self.lcd.print("----");
        self.print_time(date_time);
        self.lcd.print("----");
    }

    pub fn print_time(&mut self, date_time: NaiveDateTime) {
        let mut buf = [0u8; 8];
        Self::format_time(date_time, &mut buf);
        match from_utf8(&buf) {
            Ok(output_str) => self.lcd.print(output_str),
            Err(_) => self.lcd.print("UTF8 Error"),
        }
    }

    pub fn format_time(date_time: NaiveDateTime, out_buf: &mut [u8; 8]) {
        let mut buf_index = 0;
        Self::put_number(date_time.hour(), &mut buf_index, out_buf);
        Self::put_char_to_buf(':', &mut buf_index, out_buf);
        Self::put_number(date_time.minute(), &mut buf_index, out_buf);
        Self::put_char_to_buf(':', &mut buf_index, out_buf);
        Self::put_number(date_time.second(), &mut buf_index, out_buf);
    }

    fn put_number(num: u32, mut buf_index: &mut usize, out_buf: &mut [u8; 8]) {
        let mut temp_buf = [0u8; 16];
        let num_str = num.numtoa_str(10, &mut temp_buf);

        // If the hour/minute/second is a single-digit number we need to prepend a 0.
        // So that the length of the displayed hour is always the same.
        if num_str.len() == 1 {
            Self::put_char_to_buf('0', &mut buf_index, out_buf);
        };
        num_str.chars().for_each(|c| {
            Self::put_char_to_buf(c, &mut buf_index, out_buf);
        });
    }

    fn put_char_to_buf(c: char, buf_index: &mut usize, out_buf: &mut [u8; 8]) {
        c.encode_utf8(&mut out_buf[*buf_index..]);
        *buf_index += 1;
    }

}
