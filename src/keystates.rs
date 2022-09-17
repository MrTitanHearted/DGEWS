#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct KeyState {
    is_down: bool,
    is_released: bool,
    is_changed: bool,
}

#[allow(dead_code)]
impl KeyState {
    pub(crate) fn new(is_down: bool, is_released: bool, is_changed: bool) -> Self {
        Self {
            is_down,
            is_released,
            is_changed,
        }
    }

    pub(crate) fn is_down(&self) -> bool {
        return self.is_down;
    }

    pub(crate) fn is_released(&self) -> bool {
        return self.is_released;
    }

    pub(crate) fn is_changed(&self) -> bool {
        return self.is_changed;
    }

    pub(crate) fn set_down(&mut self, value: bool) {
        self.is_down = value;
    }

    pub(crate) fn set_released(&mut self, value: bool) {
        self.is_released = value;
    }

    pub(crate) fn set_changed(&mut self, value: bool) {
        self.is_changed = value;
    }
}
