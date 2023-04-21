use arduino_hal::{
    hal::port::PC0,
    pac::ADC,
    port::{
        mode::{Analog, Floating, Input},
        Pin,
    },
    Adc,
};

/*
 * The keypad works by reading the voltage on the analog pin A0. The buttons
 * of the keypad are connected with the circuit using different resistors,
 * therefore when different buttons are pressed the analog input reads different
 * values of the voltage and thus we can determine which button was pressed.
 *
 * The ADC (Analog-to-digital) converter is needed because we need to convert
 * the analog signal that we get from the button A0 pin and turn it into a digital
 * value between 0-1024 so that we can then switch on the value and identify the
 * button pressed.
 */
pub struct Keypad {
    input_pin: Pin<Analog, PC0>, // the analog A0 input pin.
    adc: Adc,                    // Analog-to-digital converter.
}

impl Keypad {
    pub fn new(adc: ADC, a0: Pin<Input<Floating>, PC0>) -> Self {
        let mut adc = arduino_hal::Adc::new(adc, Default::default());
        let input_pin = a0.into_analog_input(&mut adc);
        Self { input_pin, adc }
    }

    fn read_voltage(&mut self) -> u16 {
        self.input_pin.analog_read(&mut self.adc)
    }

    pub fn get_input(&mut self) -> Option<KeypadInput> {
        let voltage: u16 = self.read_voltage();

        if voltage < RIGHT_THRESHOLD {
            Some(KeypadInput::Right)
        } else if voltage < UP_THRESHOLD {
            Some(KeypadInput::Up)
        } else if voltage < DOWN_THRESHOLD {
            Some(KeypadInput::Down)
        } else if voltage < LEFT_THRESHOLD {
            Some(KeypadInput::Left)
        } else if voltage < SELECT_THRESHOLD {
            Some(KeypadInput::Select)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeypadInput {
    Select,
    Up,
    Down,
    Left,
    Right,
}

impl KeypadInput {
    pub fn to_string(&self) -> &'static str {
        match self {
            KeypadInput::Select => "Select",
            KeypadInput::Up => "Up",
            KeypadInput::Down => "Down",
            KeypadInput::Left => "Left",
            KeypadInput::Right => "Right",
        }
    }
}

const RIGHT_THRESHOLD: u16 = 60;
const UP_THRESHOLD: u16 = 200;
const DOWN_THRESHOLD: u16 = 400;
const LEFT_THRESHOLD: u16 = 600;
const SELECT_THRESHOLD: u16 = 800;
