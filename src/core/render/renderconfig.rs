use crate::core::render::{cpurender::CPURenderConfig, rendercommands::Renderer};
use std::sync::Arc;
use raw_window_handle::{HasWindowHandle, HasDisplayHandle};
use crate::core::{color::Color, shapes::Rect, size::Size};

pub enum RenderBackend {
    CPU(CPURenderConfig),
    //WGPU(WGPURenderConfig),
    //GL11(GL11RenderConfig), // Задел на будущее!
}

impl Renderer for RenderBackend {
    fn init_window<W: HasWindowHandle + HasDisplayHandle>(&mut self, window: &W) {
        match self {
            RenderBackend::CPU(r) => r.init_window(window),
            //RenderBackend::WGPU(r) => r.init_window(window),
            //RenderBackend::GL11(r) => r.init_window(window),
        }
    }

    fn begin(&mut self, size: Size) {
        match self {
            RenderBackend::CPU(r) => r.begin(size),
            //RenderBackend::WGPU(r) => r.begin(size),
            //RenderBackend::GL11(r) => r.begin(size),
        }
    }

    fn flush(&mut self) {
        match self {
            RenderBackend::CPU(r) => r.flush(),
            //RenderBackend::WGPU(r) => r.flush(),
            //RenderBackend::GL11(r) => r.flush(),
        }
    }

    fn drawrect(&mut self, rect: Rect, color: Color, clip: Rect) {
        match self {
            RenderBackend::CPU(r) => r.drawrect(rect, color, clip),
            //RenderBackend::WGPU(r) => r.drawrect(rect, color, clip),
            //RenderBackend::GL11(r) => r.drawrect(rect, color, clip),
        }
    }

    fn drawtext(&mut self, rect: Rect, color: Color, textcolor: Color, text: Arc<String>, clip: Rect, font_size: i32, padding: f32) {
        match self {
            RenderBackend::CPU(r) => r.drawtext(rect, color, textcolor, text, clip, font_size, padding),
            //RenderBackend::WGPU(r) => r.drawtext(rect, color, textcolor, text, clip, font_size, padding),
            //RenderBackend::GL11(r) => r.drawtext(rect, color, textcolor, text, clip, font_size, padding),
        }
    }
}