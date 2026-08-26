use std::sync::Arc;
use raw_window_handle::{HasWindowHandle, HasDisplayHandle};
use crate::core::{color::Color, shapes::Rect, size::Size};

pub trait Renderer {
    fn init_window<W: HasWindowHandle + HasDisplayHandle>(&mut self, window: &W);

    fn begin(&mut self, size: Size);
    fn flush(&mut self);

    fn drawrect(&mut self, rect: Rect, color: Color, clip: Rect);
    fn drawtext(&mut self, rect: Rect, color: Color, textcolor: Color, text: Arc<String>, clip: Rect, font_size: i32, padding: f32);
}