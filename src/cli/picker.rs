#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    J,
    K,
    Enter,
    Esc,
    Q,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickerEvent {
    Continue,
    Selected(usize),
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct Picker {
    selected: usize,
    len: usize,
}

impl Picker {
    pub fn new(len: usize) -> Self {
        Self { selected: 0, len }
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn handle(&mut self, key: Key) -> PickerEvent {
        if self.len == 0 {
            return PickerEvent::Cancelled;
        }

        match key {
            Key::Up | Key::K => {
                self.selected = self.selected.saturating_sub(1);
                PickerEvent::Continue
            }
            Key::Down | Key::J => {
                self.selected = (self.selected + 1).min(self.len - 1);
                PickerEvent::Continue
            }
            Key::Enter => PickerEvent::Selected(self.selected),
            Key::Esc | Key::Q => PickerEvent::Cancelled,
        }
    }
}
