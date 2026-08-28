use crate::core::{event::DrawCommand, kernel::GuiWindow, render::{cpurender::CPURenderConfig, rendercommands::Renderer, wgpurender::WGPURenderConfig}};
use std::sync::Arc;
use crate::core::{color::Color, shapes::Rect, size::Size};

pub enum RenderBackend {
    CPU(CPURenderConfig),
    WGPU(WGPURenderConfig),
    //GL11(GL11RenderConfig),
}

impl Renderer for RenderBackend {
    fn init_window(&mut self, window: Arc<dyn GuiWindow>, size: Size) {
        match self {
            RenderBackend::CPU(r) => r.init_window(window, size),
            RenderBackend::WGPU(r) => r.init_window(window, size),
            //RenderBackend::GL11(r) => r.init_window(window),
        }
    }

    fn is_partial_render(&self) -> bool {
        match self {
            RenderBackend::CPU(r) => r.is_partial_render(),
            RenderBackend::WGPU(r) => r.is_partial_render(),
            //RenderBackend::GL11(r) => r.init_window(window),
        }
    }

    fn begin(&mut self) {
        match self {
            RenderBackend::CPU(r) => r.begin(),
            RenderBackend::WGPU(r) => r.begin(),
            //RenderBackend::GL11(r) => r.begin(),
        }
    }

    fn flush(&mut self) {
        match self {
            RenderBackend::CPU(r) => r.flush(),
            RenderBackend::WGPU(r) => r.flush(),
            //RenderBackend::GL11(r) => r.flush(),
        }
    }

    fn resize(&mut self, size: Size) {
        match self {
            RenderBackend::CPU(r) => r.resize(size),
            RenderBackend::WGPU(r) => r.resize(size),
            //RenderBackend::GL11(r) => r.flush(),
        }
    }

    fn rendercl(&mut self, cl: &Vec<DrawCommand>) {
        match self {
            RenderBackend::CPU(r) => r.rendercl(cl),
            RenderBackend::WGPU(r) => r.rendercl(cl),
            //RenderBackend::GL11(r) => r.drawrect(rect, color, clip),
        }
    }

    fn drawrect(&mut self, rect: Rect, color: Color, clip: Rect) {
        match self {
            RenderBackend::CPU(r) => r.drawrect(rect, color, clip),
            RenderBackend::WGPU(r) => r.drawrect(rect, color, clip),
            //RenderBackend::GL11(r) => r.drawrect(rect, color, clip),
        }
    }

    fn drawtext(&mut self, rect: Rect, color: Color, textcolor: Color, text: Arc<String>, clip: Rect, font_size: i32, padding: f32) {
        match self {
            RenderBackend::CPU(r) => r.drawtext(rect, color, textcolor, text, clip, font_size, padding),
            RenderBackend::WGPU(r) => r.drawtext(rect, color, textcolor, text, clip, font_size, padding),
            //RenderBackend::GL11(r) => r.drawtext(rect, color, textcolor, text, clip, font_size, padding),
        }
    }
}