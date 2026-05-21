use crate::core::pos::Pos;

pub enum Event {
    MouseClick { pos: Pos },
    MouseRelease { pos: Pos },
    MouseMove { pos: Pos },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    None,
    ButtonClicked(String),
    ButtonReleased(String),
    Hovered(String),
    Unhovered(String),
    RedrawRequest(Option<tiny_skia::Rect>),
    UpdateLayoutRequest,
}
