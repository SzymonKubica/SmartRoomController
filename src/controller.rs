use crate::keypad::{Keypad, KeypadInput};


pub struct Controller {
    keypad: Keypad,
    sticky_selection: Option<KeypadInput>,
}

impl Controller {
    pub fn new(keypad: Keypad) -> Self {
        Self { keypad, sticky_selection: None }
    }

    pub fn read_instanteous_input(&mut self) -> Option<KeypadInput> {
        self.keypad.get_input()
    }

    pub fn read_persistent_input(&mut self) -> Option<KeypadInput> {
        let input = self.keypad.get_input();
        if let Some(value) = input {
            self.sticky_selection = Some(value);
        }
        self.sticky_selection.clone()
    }

    pub fn get_stored_input(&mut self) -> Option<KeypadInput> {
        self.sticky_selection.clone()
    }

    pub fn reset_persistent_input(&mut self) {
        self.sticky_selection = None;
    }
}
