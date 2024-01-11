extern crate egui;
use super::mousebutton::MouseButton;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Press,
    Release,
    Repeat,
}

pub type Key = egui::Key;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Close,
    Resize ((u32, u32)),
    Move ((i32, i32)),
    Focus,
    UnFocus,
    Button (MouseButton, Action),
    CursorMove ((f64, f64)),
    Scroll ((f64, f64)),
    Char (char),
    Key (Key, Action),
}