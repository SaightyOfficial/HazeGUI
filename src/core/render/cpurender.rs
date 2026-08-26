use std::{num::NonZeroU32, sync::Arc};
use crate::core::{color::Color, render::{font::FONT, font::get_glyph, rendercommands::Renderer}, shapes::Rect, size::Size, common::intersect_rects};
use tiny_skia::{Paint, Pixmap, Rect as SkiaRect};
use softbuffer::{Context, Surface};
use raw_window_handle::{WindowHandle, DisplayHandle, HasWindowHandle, HasDisplayHandle};

pub struct CPURenderConfig {
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
    fn init_window<W: HasWindowHandle + HasDisplayHandle>(&mut self, window: &W) {
        // 1. Достаем хэндлы из окна
        let handle_w = window.window_handle().unwrap();
        let handle_d = window.display_handle().unwrap();

        // 2. ПРИВОДИМ К 'static ЧЕРЕЗ RAW-УКАЗАТЕЛИ (безопасно, пока живо само окно)
        let display_static = unsafe { DisplayHandle::borrow_raw(handle_d.as_raw()) };
        let window_static = unsafe { WindowHandle::borrow_raw(handle_w.as_raw()) };

        // 3. Создаем контекст и поверхностный буфер
        let context = Context::new(display_static).expect("Failed to create context");
        let surface = Surface::new(&context, window_static).expect("Failed to create surface");

        self.context = Some(context);
        self.surface = Some(surface);
    }

    fn begin(&mut self, size: Size) {
        self.buffer = Pixmap::new(size.width as u32, size.height as u32);

        // Автоматически делаем resize поверхностного буфера окна при смене размера
        if let (Some(surface), Some(w), Some(h)) = (
            &mut self.surface,
            NonZeroU32::new(size.width as u32),
            NonZeroU32::new(size.height as u32),
        ) {
            let _ = surface.resize(w, h);
        }
    }
    fn flush(&mut self) {
        // Копируем пиксели из tiny-skia Pixmap прямо в softbuffer окно
        if let (Some(pixmap), Some(surface)) = (&mut self.buffer, &mut self.surface) {
            let mut window_buffer = surface.buffer_mut().unwrap();
            let skia_bytes = pixmap.data();

            for (i, chunk) in skia_bytes.chunks_exact(4).enumerate() {
                if i < window_buffer.len() {
                    window_buffer[i] = u32::from_ne_bytes([chunk[2], chunk[1], chunk[0], chunk[3]]);
                }
            }

            window_buffer.present().unwrap();
        }
    }

    fn drawrect(&mut self, rect: Rect, color: Color, clip: Rect) {
        if color.a == 0 {
            return;
        }

        // 1. Клипаем прямоугольник прямо перед рендером!
        let visible_rect = match intersect_rects(rect, clip) {
            Some(r) => r,
            None => return, // Полностью за пределами клипа — отбрасываем
        };

        let mut paint = Paint::default();
        paint.set_color_rgba8(color.r, color.g, color.b, color.a);

        // 2. Рисуем ТОЛЬКО урезанную видимую часть
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
        // 1. Сначала рисуем задний фон (если он не прозрачный)
        if color.a > 0 {
            self.drawrect(rect, color, clip);
        }

        // 2. Забираем мутабельный буфер кадров
        let pixmap = match &mut self.buffer {
            Some(p) => p,
            None => return,
        };

        let abs_x = rect.x as f32;
        let abs_y = rect.y as f32;

        // Font metrics for proper displaying
        let line_metrics = FONT.horizontal_line_metrics(font_size as f32);
        let lheight = line_metrics.map(|m| m.new_line_size).unwrap_or(font_size as f32);
        let ascent = line_metrics.map(|m| m.ascent).unwrap_or(font_size as f32 * 0.75);

        let mut y_cursor = abs_y + padding / 2.0;
        let mut x_cursor = abs_x + padding;

        let img_w = pixmap.width() as i32;
        let img_h = pixmap.height() as i32;
        let pixels = pixmap.pixels_mut();

        // Clips
        let clip_top = clip.top() as i32;
        let clip_bottom = clip.bottom() as i32;
        let clip_left = clip.left() as i32;
        let clip_right = clip.right() as i32;

        // Итерируемся по переданному Arc<String>
        for c in text.chars() {
            // Handle new line character
            if c == '\n' {
                y_cursor += lheight;
                x_cursor = abs_x + padding;
                continue;
            }

            // If line is too high then skip character vertically
            if y_cursor + lheight < clip_top as f32 {
                continue;
            }

            // If line is too low, stop rendering completely
            if y_cursor > clip_bottom as f32 {
                break;
            }

            // Getting char glyph
            let (metrics, bitmap) = get_glyph(c, font_size as f32);

            // If space then smart skip
            if c == ' ' {
                x_cursor += metrics.advance_width;
                continue;
            }

            // If letter is too left then skip
            if x_cursor + metrics.advance_width < clip_left as f32 {
                x_cursor += metrics.advance_width;
                continue;
            }

            // If letter is too right then skip current char and move x
            if x_cursor > clip_right as f32 {
                x_cursor += metrics.advance_width;
                continue;
            }

            let baseline = y_cursor + ascent;

            if metrics.width > 0 && metrics.height > 0 {
                // Getting letter coordinates
                let px_base = (x_cursor + metrics.xmin as f32).round() as i32;
                let py_base = (baseline - metrics.height as f32 - metrics.ymin as f32).round() as i32;

                // Calculating clip rects
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

                            // Text color with antialiasing (используем textcolor из аргументов)
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