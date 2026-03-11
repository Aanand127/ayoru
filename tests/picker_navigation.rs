use ani::cli::picker::{Key, Picker, PickerEvent};

#[test]
fn j_and_down_move_selection_forward() {
    let mut picker = Picker::new(3);
    assert_eq!(picker.selected(), 0);

    picker.handle(Key::J);
    assert_eq!(picker.selected(), 1);

    picker.handle(Key::Down);
    assert_eq!(picker.selected(), 2);
}

#[test]
fn q_and_esc_cancel_picker() {
    let mut picker = Picker::new(2);
    assert_eq!(picker.handle(Key::Q), PickerEvent::Cancelled);

    let mut picker2 = Picker::new(2);
    assert_eq!(picker2.handle(Key::Esc), PickerEvent::Cancelled);
}

#[test]
fn enter_selects_current_index() {
    let mut picker = Picker::new(2);
    picker.handle(Key::Down);
    assert_eq!(picker.handle(Key::Enter), PickerEvent::Selected(1));
}
