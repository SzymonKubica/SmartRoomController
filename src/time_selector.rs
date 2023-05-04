use ds323x::{Datelike, NaiveDate, NaiveDateTime};

struct TimeSelector {
    year: u32,
    month: Month,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    current_mode: SelectorMode,
    current_field: SelectorField,
}

impl TimeSelector {
    pub fn new() -> Self {
        Self {
            year: 2023,
            month: Month::January,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            current_mode: SelectorMode::DateSelection,
            current_field: SelectorField::Year,
        }
    }

    pub fn next_field(&mut self) {}

    pub fn increment_field(&mut self, selected_field: SelectorField) {
        match selected_field {
            SelectorField::Year => self.year += 1,
            SelectorField::Month => self.increment_month(),
            SelectorField::Day => self.increment_day(),
            SelectorField::Hour => self.hour = (self.hour + 1) % 24,
            SelectorField::Minute => self.minute = (self.minute + 1) % 60,
            SelectorField::Second => self.second = (self.second + 1) % 60,
        }
    }

    pub fn decrement_field(&mut self, selected_field: SelectorField) {
        match selected_field {
            SelectorField::Year => self.year -= 1,
            SelectorField::Month => self.decrement_month(),
            SelectorField::Day => self.decrement_day(),
            SelectorField::Hour => self.hour = (self.hour - 1) % 24,
            SelectorField::Minute => self.minute = (self.minute - 1) % 60,
            SelectorField::Second => self.second = (self.second - 1) % 60,
        }
    }

    fn increment_day(&mut self) {
        self.day = (self.day + 1) % self.month.get_number_of_days();
    }

    fn decrement_day(&mut self) {
        self.day = (self.day - 1) % self.month.get_number_of_days();
    }

    fn increment_month(&mut self) {
        self.month = Month::from_index((self.month.get_index() + 1) % 12)
    }

    fn decrement_month(&mut self) {
        self.month = Month::from_index((self.month.get_index() - 1) % 12)
    }
}

enum SelectorMode {
    DateSelection,
    TimeSelection,
}

enum SelectorField {
    Date(DateField),
    Time(TimeField),
}

enum DateField {
    Year,
    Month,
    Day,
}

enum TimeField {
    Hour,
    Minute,
    Second,
}

impl SelectorField {
    pub fn next_field(&self, mode: SelectorMode) -> SelectorField {
        match mode {
            SelectorMode::DateSelection => todo!(),
            SelectorMode::TimeSelection => todo!(),
        }
    }
}

enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn from_index(index: u32) -> Self {
        match index {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("Invalid month index: {}", index),
        }
    }
    pub fn get_index(&self) -> u32 {
        match self {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }

    pub fn get_number_of_days(&self) -> u32 {
        match self {
            Month::January => 31,
            Month::February => 28,
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }
}
