use crate::prelude::*;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Mouse {
    pub(crate) x: i16,
    pub(crate) y: i16,
    pub(crate) last_x: i16,
    pub(crate) last_y: i16,
    pub(crate) l_button: KeyState,
    pub(crate) r_button: KeyState,
    pub(crate) m_button: KeyState,
    pub(crate) x1_button: KeyState,
    pub(crate) x2_button: KeyState,
}

#[allow(dead_code)]
impl Mouse {
    pub(crate) fn new() -> Self {
        return Self {
            x: 0,
            y: 0,
            last_x: 0,
            last_y: 0,
            l_button: KeyState::new(false, false, false),
            r_button: KeyState::new(false, false, false),
            m_button: KeyState::new(false, false, false),
            x1_button: KeyState::new(false, false, false),
            x2_button: KeyState::new(false, false, false),
        };
    }

    pub(crate) fn clear_keystates(&mut self) {
        self.l_button.set_changed(false);
        self.r_button.set_changed(false);
        self.m_button.set_changed(false);
        self.x1_button.set_changed(false);
        self.x2_button.set_changed(false);
    }

    pub(crate) fn update_pos(&mut self, x: i16, y: i16) {
        self.last_x = self.x;
        self.last_y = self.y;
        self.x = x;
        self.y = y;
    }

    pub(crate) fn l_button(&self) -> KeyState {
        return self.l_button;
    }

    pub(crate) fn r_button(&self) -> KeyState {
        return self.r_button;
    }

    pub(crate) fn m_button(&self) -> KeyState {
        return self.m_button;
    }

    pub(crate) fn x1_button(&self) -> KeyState {
        return self.x1_button;
    }

    pub(crate) fn x2_button(&self) -> KeyState {
        return self.x2_button;
    }

    pub(crate) fn set_l_button_changed(&mut self, value: bool) {
        self.l_button.set_changed(value);
    }

    pub(crate) fn set_r_button_changed(&mut self, value: bool) {
        self.r_button.set_changed(value);
    }

    pub(crate) fn set_m_button_changed(&mut self, value: bool) {
        self.m_button.set_changed(value);
    }

    pub(crate) fn set_x1_button_changed(&mut self, value: bool) {
        self.x1_button.set_changed(value);
    }

    pub(crate) fn set_x2_button_changed(&mut self, value: bool) {
        self.x2_button.set_changed(value);
    }

    pub(crate) fn set_l_button_released(&mut self, value: bool) {
        self.l_button.set_released(value);
    }

    pub(crate) fn set_r_button_released(&mut self, value: bool) {
        self.r_button.set_released(value);
    }

    pub(crate) fn set_m_button_released(&mut self, value: bool) {
        self.m_button.set_released(value);
    }

    pub(crate) fn set_x1_button_released(&mut self, value: bool) {
        self.x1_button.set_released(value);
    }

    pub(crate) fn set_x2_button_released(&mut self, value: bool) {
        self.x2_button.set_released(value);
    }

    pub(crate) fn set_l_button_down(&mut self, value: bool) {
        self.l_button.set_down(value);
    }

    pub(crate) fn set_r_button_down(&mut self, value: bool) {
        self.r_button.set_down(value);
    }

    pub(crate) fn set_m_button_down(&mut self, value: bool) {
        self.m_button.set_down(value);
    }

    pub(crate) fn set_x1_button_down(&mut self, value: bool) {
        self.x1_button.set_down(value);
    }

    pub(crate) fn set_x2_button_down(&mut self, value: bool) {
        self.x2_button.set_down(value);
    }

    pub(crate) fn x_offset(&self) -> i16 {
        return self.x - self.last_x;
    }

    pub(crate) fn y_offset(&self) -> i16 {
        return self.y - self.last_y;
    }

    pub(crate) fn x(&self) -> i16 {
        return self.x;
    }

    pub(crate) fn y(&self) -> i16 {
        return self.y;
    }

    pub(crate) fn xy(&self) -> (i16, i16) {
        return (self.x, self.y);
    }

    pub(crate) fn last_x(&self) -> i16 {
        return self.last_x;
    }

    pub(crate) fn last_y(&self) -> i16 {
        return self.last_y;
    }

    pub(crate) fn last_xy(&self) -> (i16, i16) {
        return (self.last_x, self.last_y);
    }

    pub(crate) fn xy_offset(&self) -> (i16, i16) {
        return (self.x_offset(), self.y_offset());
    }
}
