use crate::core::pos::Pos;

pub enum Event {
    MouseClick { pos: Pos },
    MouseMove { pos: Pos },
    // Сюда потом добавишь нажатия клавиш и т.д.
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    None,
    ButtonClicked(String),
    ButtonReleased(String),
    Hovered(String),
    Unhovered(String),
    RedrawRequest,
    UpdateLayoutRequest,
}