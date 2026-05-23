use crate::core::common::{Axis, SizeEnum, SizeStrat, LayoutEnum, LayoutStrat, Side, intersect_rects};
use crate::core::event::{Action, Event};
use crate::core::idpool::regid;
use crate::core::size::Size;
use crate::core::widget::{Widget, WidgetBase};
use crate::core::{color::Color, pos::Pos};
use tiny_skia::{Color as SkiaColor, Paint, PixmapMut, Rect};
use std::collections::HashMap;
use std::sync::Mutex;

use fontdue::{Font, FontSettings};

static FONT_DATA: &[u8] = include_bytes!("../../fonts/JetBrainsMono-Regular.ttf");

//Glyph cache and font data
lazy_static::lazy_static! {
    pub static ref GLYPH_CACHE: Mutex<HashMap<(char, u32), (fontdue::Metrics, Vec<u8>)>> = Mutex::new(HashMap::new());
    pub static ref FONT: Font = {
        Font::from_bytes(FONT_DATA, FontSettings::default()).expect("Font load error")
    };
}

///Rasterizes character and saves it to glyph cache
pub fn get_glyph(c: char, font_size: f32) -> (fontdue::Metrics, Vec<u8>) {
    let size_key = (font_size * 100.0) as u32;
    let mut cache = GLYPH_CACHE.lock().expect("Poisoned glyph cache");

    if let Some(glyph) = cache.get(&(c, size_key)) {
        return glyph.clone();
    }

    //Rasterising font and saving it
    let (metrics, bitmap) = FONT.rasterize(c, font_size);
    cache.insert((c, size_key), (metrics.clone(), bitmap.clone()));
    
    (metrics, bitmap)
}

///Label struct where all label data is stored
#[derive(Clone)]
pub struct Label {
    pub base: WidgetBase,
    pub text: Vec<String>,
    pub font_size: f32,
    pub textcolor: Color,
    padding: f32,
}

impl Label {
    pub fn new(id: String) -> Self {
        let mut label = Self {
            base: WidgetBase::new(regid(id)),
            text: Vec::new(),
            font_size: 14.0,
            textcolor: Color::BLACK,
            padding: 2.0,
        };
        label.base.bgcolor = Color::TRANSPARENT;
        label.update_size();
        label
    }

    ///Recaulcalting and updating label size
    pub fn update_size(&mut self) {
        let old_size = self.base.size;
        let mut max_width: f32 = 0.0;
        let mut height = 0.0;

        //Getting char catrics
        let line_metrics = FONT.horizontal_line_metrics(self.font_size);

        //Line height
        let lheight = line_metrics
            .map(|m| m.new_line_size)
            .unwrap_or(self.font_size);

        for line in &self.text {
            let mut lwidth = 0.0; //Line width
            for c in line.chars() {
                let metrics = FONT.metrics(c, self.font_size);
                lwidth += metrics.advance_width;
            }
            //Getting maximum size
            max_width = max_width.max(lwidth);
            height += lheight;
        }

        //Setting new size, and if size has changed, then request relayout
        self.base.size = Size::new((max_width + self.padding * 2.0) as i32, (height + self.padding) as i32);
        if old_size.width != self.base.size.width || old_size.height != self.base.size.height {
            self.set_relayout_flag(true);
        }
    }

    ///Setting text for label
    pub fn text(mut self, new_text: String) -> Self {
        self.text = new_text.split("\n").map(String::from).collect();
        self.update_size();
        self
    }

    ///Getting text from label
    pub fn get_text(&self) -> String {
        self.text.join("\n")
    }

    ///Setting text for label at runtime
    pub fn new_text(&mut self, new_text: String) {
        self.text = new_text.split("\n").map(String::from).collect();
        self.update_size();
        self.set_dirty_flag(true);
    }

    ///Setting font size for label at runtime
    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size;
        self.update_size();
        self.set_relayout_flag(true);
        self.set_dirty_flag(true);
    }

    ///Setting text color at runtime
    pub fn set_color(&mut self, color: Color) {
        self.textcolor = color;
        self.set_dirty_flag(true);
    }

    ///Setting background color at runtime
    pub fn set_bgcolor(&mut self, color: Color) {
        self.base.bgcolor = color;
        self.set_dirty_flag(true);
    }

    //positions
    pub fn pos(mut self, posnew: Pos) -> Self {
        self.base.pos = posnew;
        self.base.layoutstrat.method = LayoutEnum::MANUAL;
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.base.layoutstrat.side = side;
        self
    }

    //colors
    pub fn color(mut self, color: Color) -> Self {
        self.textcolor = color;
        self
    }

    pub fn bgcolor(mut self, color: Color) -> Self {
        self.base.bgcolor = color;
        self
    }

    //sizes
    ///By which axises widget will stretch and fill itself
    pub fn fill(mut self, cords: Axis) -> Self {
        self.base.sizestrat.fill = cords;
        if cords == Axis::NONE {
            self.base.sizestrat.method = SizeEnum::AUTO;
        } else {
            self.base.sizestrat.method = SizeEnum::FILL;
        }
        self
    }

    ///Changes at runtime by which axises widget will stretch and fill itself
    pub fn set_fill(&mut self, cords: Axis){
        self.base.sizestrat.fill = cords;
        if cords == Axis::NONE {
            self.base.sizestrat.method = SizeEnum::AUTO;
        } else {
            self.base.sizestrat.method = SizeEnum::FILL;
        }
    }

    ///Sets widget size, widget will not dynamicaly change size
    ///NOTE: Be careful while using it because if widget is too big it will be not fully visible
    pub fn size(mut self, size: Size) -> Self {
        self.base.size = size;
        self.base.sizestrat.method = SizeEnum::MANUAL;
        self
    }

    ///Sets font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self.update_size();
        self.set_relayout_flag(true);
        self
    }
}

impl Widget for Label {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn get_id(&self) -> u64 {
        self.base.id
    }
    fn draw(
        &self,
        pixmap: &mut PixmapMut,
        pos_off: Pos,
        clip: Rect,
        _preferred_color: Option<Color>,
    ) {
        //Getting absolute coordinates
        let abs_x = (pos_off.x + self.base.pos.x) as f32;
        let abs_y = (pos_off.y + self.base.pos.y) as f32;

        //Backgournd render, BGRA needed
        if let Some(bg_rect) = Rect::from_xywh(abs_x, abs_y, self.base.size.width as f32, self.base.size.height as f32) {
            let mut bg_paint = Paint::default();
            bg_paint.set_color(SkiaColor::from_rgba8(self.base.bgcolor.b, self.base.bgcolor.g, self.base.bgcolor.r, self.base.bgcolor.a));
            if let Some(visible_bg) = intersect_rects(bg_rect, clip) {
                pixmap.fill_rect(visible_bg, &bg_paint, tiny_skia::Transform::identity(), None);
            }
        }

        //Font metrics for proper displaying
        let line_metrics = FONT.horizontal_line_metrics(self.font_size);
        let lheight = line_metrics.map(|m| m.new_line_size).unwrap_or(self.font_size);
        let ascent = line_metrics.map(|m| m.ascent).unwrap_or(self.font_size * 0.75);

        let mut y_cursor = abs_y + self.padding/2.0;

        let img_w = pixmap.width() as i32;
        let img_h = pixmap.height() as i32;
        let pixels = pixmap.pixels_mut();

        //Clips
        let clip_top = clip.top() as i32;
        let clip_bottom = clip.bottom() as i32;
        let clip_left = clip.left() as i32;
        let clip_right = clip.right() as i32;

        for line in &self.text {
            //If line is too high then skip it
            if y_cursor + lheight < clip.top() as f32 {
                y_cursor += lheight;
                continue;
            }

            //If line is too low
            if y_cursor > clip_bottom as f32 {
                break;
            }

            //Basics
            let mut x_cursor = abs_x + self.padding;
            let baseline = y_cursor + ascent;

            for c in line.chars() {
                //Getting char glyph
                let (metrics, bitmap) = get_glyph(c, self.font_size);
                //If space then smart skip
                if c == ' ' {
                    x_cursor += metrics.advance_width;
                    continue;
                }

                //If letter is too left then skip
                if x_cursor + metrics.advance_width < clip.left() as f32 {
                    x_cursor += metrics.advance_width;
                    continue;
                }
                //If letter is too right then skip
                if x_cursor > clip_right as f32 {
                    break;
                }

                if metrics.width > 0 && metrics.height > 0 {
                    //Getting letter coordinates
                    let px_base = (x_cursor + metrics.xmin as f32).round() as i32;
                    let py_base = (baseline - metrics.height as f32 - metrics.ymin as f32).round() as i32;

                    //Calculating clip rects
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
                                if alpha == 0 { continue; } //Skipping empty pixels

                                let screen_x = px_base + col;
                                let pixel_idx = row_offset + screen_x as usize;

                                //Text color with antialiasing
                                let fg_color = tiny_skia::ColorU8::from_rgba(
                                    self.textcolor.r,
                                    self.textcolor.g,
                                    self.textcolor.b,
                                    alpha as u8,
                                ).premultiply();

                                //Color math
                                if alpha == 255 {
                                    pixels[pixel_idx] = fg_color;
                                } else {
                                    //Calculating alpha blending by using linear interpolation
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
                                        out_r as u8, out_g as u8, out_b as u8, out_a as u8
                                    ) {
                                        pixels[pixel_idx] = blended;
                                    }
                                }
                            }
                        }
                    }
                }
                x_cursor += metrics.advance_width;//Next character
            }
            y_cursor += lheight;//Next line
        }
    }
    fn get_size(&self) -> Size {
        self.base.size
    }
    fn get_pos(&self) -> Pos {
        self.base.pos
    }
    fn get_layout_strat(&self) -> LayoutStrat {
        self.base.layoutstrat
    }
    fn get_size_strat(&self) -> SizeStrat {
        self.base.sizestrat
    }
    fn set_size(&mut self, size_new: Size) {
        self.base.size.width = size_new.width;
        self.base.size.height = size_new.height;
    }
    fn set_pos(&mut self, pos_new: Pos) {
        self.base.pos.x = pos_new.x;
        self.base.pos.y = pos_new.y;
    }
    fn is_dirty(&self) -> bool {
        self.base.is_dirty
    }
    fn set_dirty_flag(&mut self, flag: bool) {
        self.base.is_dirty = flag;
    }
    fn needs_relayout(&self) -> bool {
        self.base.needs_relayout
    }
    fn set_relayout_flag(&mut self, flag: bool) {
        self.base.needs_relayout = flag;
    }
    fn update_layout(&mut self, _forced: bool) {}
    fn handle_event(&mut self, _event: &Event, _pos_off: Pos, _actions: &mut Vec<Action>) {
        if self.needs_relayout() {
            self.set_relayout_flag(false);
        }
    }
    fn get_dirty_rect(&mut self, pos_off: Pos, requests: &mut Vec<Action>) {
        //If self is dirty, then push redraw request with self rect inside
        if self.is_dirty() {
            if let Some(dirty_rect) = self.get_self_rect(pos_off) {
                requests.push(Action::RedrawRequest(Some(dirty_rect)));
            }
            self.set_dirty_flag(false); //Clearing flag for self
        }
    }
}
