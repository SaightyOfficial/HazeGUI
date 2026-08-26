use std::sync::Arc;

use crate::core::{color::Color, pos::Pos, shapes::Rect};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MKey {
    Right,
    Left,
    Middle,
    Back,
    Forward,
    None
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KKey {
    Backspace,
    Enter,
    Space,
    PrintScr,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Shift,
    Tab,
    Ctrl,
    Super,
    Alt,
    Escape,
    Fn,
    F1, F2, F3, F4, F5, F6,
    F7, F8, F9, F10, F11, F12,
    ContextMenu,
    CapsLock,
    None
}

/// Represents input events that can be handled by widgets
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    MouseClick { pos: Pos, key: MKey },
    MouseRelease { pos: Pos, key: MKey  },
    MouseMove { pos: Pos },
    KeyPress { ch: char, key: KKey },
    KeyRelease { ch: char, key: KKey },
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
    /// Change of scrollbar, contains ID and scroll percentage(from 0.0 to 1.0)
    ScrollChanged(u64, f32),
    /// Switch was clicked and changed its value,
    SwitchChanged(u64, bool),
    /// Textbox was focused
    TextboxFocus(u64),
    /// Textbox lost focus
    TextboxUnfocus(u64),
    /// Text inside textbox changed
    TextboxTextChange(u64),
    /// Should be sent if something wants to redraw itself,
    /// contains self rect (can and in most cases should be got by [`crate::core::widget::Widget::get_self_rect`]),
    /// if contains [`None`] whole window will be redrawn
    RedrawRequest(Option<Rect>),
    /// Should be sent if something wants to relayout, relayouts widget tree and automaticaly redraws whole window
    UpdateLayoutRequest,
    /// Custom action, has widget id and String with other data that you will need, can be sended only by manual push by using [`crate::frame::Frame::push_action`]
    CustomAction(u64,String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawCommand {
    ///rect, bgcolor, clip
    Rect(Rect, Color, Rect),
    ///rect, bgcolor, textcolor, text, clip, font_size, padding
    Text(Rect, Color, Color, Arc<String>, Rect, i32, f32),
    //DrawTexture(Rect, ),
}
