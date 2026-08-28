use std::sync::Arc;
use crate::core::{color::Color, event::DrawCommand, kernel::GuiWindow, shapes::Rect, size::Size};

pub trait Renderer {
    fn init_window(&mut self, window: Arc<dyn GuiWindow>, size: Size);
    fn is_partial_render(&self) -> bool;

    fn begin(&mut self);
    fn flush(&mut self);
    fn resize(&mut self, size: Size);

    fn drawrect(&mut self, rect: Rect, color: Color, clip: Rect);
    fn drawtext(&mut self, rect: Rect, color: Color, textcolor: Color, text: Arc<String>, clip: Rect, font_size: i32, padding: f32);
    fn rendercl(&mut self, cl: &Vec<DrawCommand>);
}