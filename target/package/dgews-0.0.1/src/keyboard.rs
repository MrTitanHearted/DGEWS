use crate::prelude::*;

use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct Keyboard {
    keys: HashMap<usize, KeyState>,
    chars: HashMap<usize, bool>,
    autorepeat: bool,
    n_keys: usize,
}

#[allow(dead_code)]
impl Keyboard {
    pub(crate) fn new(autorepeat: bool) -> Self {
        Self::from_nkeys(autorepeat, 256)
    }

    pub(crate) fn from_nkeys(autorepeat: bool, n_keys: usize) -> Self {
        let mut keys = HashMap::new();
        let mut chars = HashMap::new();

        for i in 0..n_keys {
            keys.insert(i, KeyState::new(false, false, false));
            chars.insert(i, false);
        }

        Self {
            keys,
            chars,
            autorepeat,
            n_keys,
        }
    }

    pub(crate) fn replace(
        &mut self,
        key: usize,
        is_down: bool,
        is_released: bool,
        is_changed: bool,
    ) {
        *self.keys.get_mut(&key).unwrap() = KeyState::new(is_down, is_released, is_changed);
    }

    pub(crate) fn is_down(&self, keycode: usize) -> bool {
        return self.keys.get(&keycode).unwrap().is_down();
    }

    pub(crate) fn is_released(&self, keycode: usize) -> bool {
        return self.keys.get(&keycode).unwrap().is_released();
    }

    pub(crate) fn is_changed(&self, keycode: usize) -> bool {
        return self.keys.get(&keycode).unwrap().is_changed();
    }

    pub(crate) fn is_char(&self, char: usize) -> bool {
        return *self.chars.get(&char).unwrap();
    }

    pub(crate) fn set_is_down(&mut self, keycode: usize, value: bool) {
        self.keys.get_mut(&keycode).unwrap().set_down(value);
    }

    pub(crate) fn set_is_released(&mut self, keycode: usize, value: bool) {
        self.keys.get_mut(&keycode).unwrap().set_released(value);
    }

    pub(crate) fn set_is_changed(&mut self, keycode: usize, value: bool) {
        self.keys.get_mut(&keycode).unwrap().set_changed(value);
    }

    pub(crate) fn set_is_char(&mut self, char: usize, pressed: bool) {
        *self.chars.get_mut(&char).unwrap() = pressed;
    }

    pub(crate) fn autorepeat(&self) -> bool {
        return self.autorepeat;
    }

    pub(crate) fn set_autorepeat(&mut self, value: bool) {
        self.autorepeat = value;
    }

    pub(crate) fn enable_autorepeat(&mut self) {
        self.set_autorepeat(true);
    }

    pub(crate) fn disable_autorepeat(&mut self) {
        self.set_autorepeat(false);
    }

    pub(crate) fn clear(&mut self) {
        for i in 0..self.n_keys {
            self.set_is_changed(i, false);
            self.set_is_char(i, false);
        }
    }
}
