use std::{num::NonZeroU32, sync::Arc};
use crate::core::{color::Color, common::intersect_rects, event::DrawCommand, kernel::GuiWindow, render::{font::{FONTBYTES, get_glyph}, rendercommands::Renderer}, shapes::Rect, size::Size};
use tiny_skia::{Paint, Pixmap, Rect as SkiaRect};
use softbuffer::{Context, Surface};
use raw_window_handle::{WindowHandle, DisplayHandle, HasWindowHandle, HasDisplayHandle};

pub struct CPURenderConfig {
    pub is_partial_render: bool,
    pub max_threads: usize,
    pub max_thread_area: i64,
    
    //pub fps: u32,
    pub buffer: Option<Pixmap>,
    pub context: Option<Context<DisplayHandle<'static>>>,
    pub surface: Option<Surface<DisplayHandle<'static>, WindowHandle<'static>>>,
}

impl Default for CPURenderConfig {
    fn default() -> Self {
        Self {
            is_partial_render: true,
            max_threads: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
            max_thread_area: 160_000,
            //fps: 60,

            buffer: None,
            context: None,
            surface: None,
        }
    }
}

impl Renderer for CPURenderConfig {
    fn is_partial_render(&self) -> bool {
        self.is_partial_render
    }
    fn init_window(&mut self, window: Arc<dyn GuiWindow>, size: Size) {
        let handle_w = window.window_handle().unwrap();
        let handle_d = window.display_handle().unwrap();

        let display_static = unsafe { DisplayHandle::borrow_raw(handle_d.as_raw()) };
        let window_static = unsafe { WindowHandle::borrow_raw(handle_w.as_raw()) };

        let context = Context::new(display_static).expect("Failed to create context");
        let surface = Surface::new(&context, window_static).expect("Failed to create surface");

        self.context = Some(context);
        self.surface = Some(surface);

        self.resize(size);
    }
    fn begin(&mut self) {}
    fn resize(&mut self, size: Size) {
        self.buffer = Pixmap::new(size.width as u32, size.height as u32);

        if let (Some(surface), Some(w), Some(h)) = (
            &mut self.surface,
            NonZeroU32::new(size.width as u32),
            NonZeroU32::new(size.height as u32),
        ) {
            let _ = surface.resize(w, h);
        }
    }
    fn flush(&mut self) {
        if let (Some(pixmap), Some(surface)) = (&mut self.buffer, &mut self.surface) {
            let mut window_buffer = surface.buffer_mut().unwrap();
            
            let skia_pixels: &[u32] = bytemuck::cast_slice(pixmap.data());

            for (dst, &src) in window_buffer.iter_mut().zip(skia_pixels.iter()) {
                let r = (src >> 0) & 0xFF;
                let g = (src >> 8) & 0xFF;
                let b = (src >> 16) & 0xFF;
                let a = (src >> 24) & 0xFF;

                *dst = (a << 24) | (r << 16) | (g << 8) | b;
            }

            window_buffer.present().unwrap();
        }
    }
    fn rendercl(&mut self, cl: &Vec<DrawCommand>) {
        for da in cl {
            match da {
                DrawCommand::Rect(rect, color, clip) => {
                    self.drawrect(*rect, *color, *clip);
                }
                DrawCommand::Text(rect, color, textcolor, text, cliprect, font_size, padding) => {
                    self.drawtext(*rect, *color, *textcolor, text.clone(), *cliprect, *font_size, *padding);
                }
            }
        }
    }
    fn drawrect(&mut self, rect: Rect, color: Color, clip: Rect) {
        if color.a == 0 {  return; }
        let visible_rect = match intersect_rects(rect, clip) {
            Some(r) => r,
            None => return,
        };

        let mut paint = Paint::default();
        paint.set_color_rgba8(color.r, color.g, color.b, color.a);

        if let Some(skia_rect) = SkiaRect::from_xywh(
            visible_rect.x as f32,
            visible_rect.y as f32,
            visible_rect.width as f32,
            visible_rect.height as f32,
        ) {
            if let Some(ebuffer) = &mut self.buffer {
                ebuffer.fill_rect(
                    skia_rect,
                    &paint,
                    tiny_skia::Transform::identity(),
                    None,
                );
            }
        }
    }
    fn drawtext(
        &mut self,
        rect: Rect,
        color: Color,
        textcolor: Color,
        text: Arc<String>,
        clip: Rect,
        font_size: i32,
        padding: f32,
    ) {
        if color.a > 0 {
            self.drawrect(rect, color, clip);
        }

        let pixmap = match &mut self.buffer {
            Some(p) => p,
            None => return,
        };

        let abs_x = rect.x as f32;
        let abs_y = rect.y as f32;

        let font_size_f32 = font_size as f32;
        let line_metrics = FONTBYTES.horizontal_line_metrics(font_size_f32);
        
        let lheight = line_metrics.map(|m| m.new_line_size).unwrap_or(font_size_f32);
        let ascent = line_metrics.map(|m| m.ascent).unwrap_or_else(|| {
            font_size_f32 * 0.8
        });

        let mut y_cursor = abs_y + padding / 2.0;
        let mut x_cursor = abs_x + padding;

        let img_w = pixmap.width() as i32;
        let img_h = pixmap.height() as i32;
        let pixels = pixmap.pixels_mut();

        let clip_top = clip.top() as i32;
        let clip_bottom = clip.bottom() as i32;
        let clip_left = clip.left() as i32;
        let clip_right = clip.right() as i32;

        for c in text.chars() {
            if c == '\n' {
                y_cursor += lheight;
                x_cursor = abs_x + padding;
                continue;
            }

            if y_cursor + lheight < clip_top as f32 {
                continue;
            }

            if y_cursor > clip_bottom as f32 {
                break;
            }

            let (metrics, bitmap) = get_glyph(c, font_size_f32);

            if c == ' ' {
                x_cursor += metrics.advance_width;
                continue;
            }

            if x_cursor + metrics.advance_width < clip_left as f32 {
                x_cursor += metrics.advance_width;
                continue;
            }

            if x_cursor > clip_right as f32 {
                x_cursor += metrics.advance_width;
                continue;
            }

            let baseline = y_cursor + ascent;

            if metrics.width > 0 && metrics.height > 0 {
                let px_base = (x_cursor + metrics.xmin as f32).round() as i32;
                let py_base = (baseline - metrics.ymin as f32 - metrics.height as f32).round() as i32;

                let start_row = 0.max(clip_top - py_base).max(0);
                let end_row = (metrics.height as i32).min(clip_bottom - py_base).min(img_h - py_base);

                let start_col = 0.max(clip_left - px_base).max(0);
                let end_col = (metrics.width as i32).min(clip_right - px_base).min(img_w - px_base);

                if start_row < end_row && start_col < end_col {
                    for row in start_row..end_row {
                        let screen_y = py_base + row;
                        let row_offset = (screen_y * img_w) as usize;
                        let bitmap_row_offset = (row as usize) * metrics.width;

                        for col in start_col..end_col {
                            let alpha = bitmap[bitmap_row_offset + (col as usize)] as u32;
                            if alpha == 0 {
                                continue;
                            }

                            let screen_x = px_base + col;
                            let pixel_idx = row_offset + screen_x as usize;

                            let fg_color = tiny_skia::ColorU8::from_rgba(
                                textcolor.r,
                                textcolor.g,
                                textcolor.b,
                                alpha as u8,
                            )
                            .premultiply();

                            if alpha == 255 {
                                pixels[pixel_idx] = fg_color;
                            } else {
                                let bg_color = pixels[pixel_idx];
                                let bg_a = bg_color.alpha() as u32;
                                let bg_r = bg_color.red() as u32;
                                let bg_g = bg_color.green() as u32;
                                let bg_b = bg_color.blue() as u32;

                                let fg_a = fg_color.alpha() as u32;
                                let fg_r = fg_color.red() as u32;
                                let fg_g = fg_color.green() as u32;
                                let fg_b = fg_color.blue() as u32;

                                let inv_a = 255 - fg_a;
                                let out_a = fg_a + (bg_a * inv_a) / 255;
                                let out_r = fg_r + (bg_r * inv_a) / 255;
                                let out_g = fg_g + (bg_g * inv_a) / 255;
                                let out_b = fg_b + (bg_b * inv_a) / 255;

                                if let Some(blended) = tiny_skia::PremultipliedColorU8::from_rgba(
                                    out_r as u8,
                                    out_g as u8,
                                    out_b as u8,
                                    out_a as u8,
                                ) {
                                    pixels[pixel_idx] = blended;
                                }
                            }
                        }
                    }
                }
            }
            x_cursor += metrics.advance_width;
        }
    }
}