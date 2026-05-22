use crate::core::pos::Pos;

/// Represents input events that can be handled by widgets
pub enum Event {
    MouseClick { pos: Pos },
    MouseRelease { pos: Pos },
    MouseMove { pos: Pos },
    KeyPress { key: char },
}

/// Actions requested by widgets to be processed by HazeGUI
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// No action required, used as a placeholder for nothing
    None,
    /// Button was clicked, contains ID
    ButtonClicked(u64),
    /// Button was released, contains ID
    ButtonReleased(u64),
    /// Something was hovered, contains ID
    Hovered(u64),
    /// Something was unhovered, contains ID
    Unhovered(u64),
    /// Should be sent if something wants to redraw itself,
    /// contains self rect (can and in most cases should be got by [`crate::Widget::get_self_rect`]),
    /// if contains [`None`] whole window will be redrawn
    RedrawRequest(Option<tiny_skia::Rect>),
    /// Should be sent if something wants to relayout, relayouts and automaticaly redraws whole window and widget tree
    UpdateLayoutRequest,
}
