use crate::core::common::{ChooseCords, SizeEnum, SizeStrat};
use crate::core::event::{Action, Event};
use crate::core::idpool::regid;
use crate::core::size::Size;
use crate::core::widget::{Widget, WidgetBase};
use crate::core::{color::Color, pos::Pos};
use tiny_skia::{Color as SkiaColor, Paint, PixmapMut, Rect};

use fontdue::{Font, FontSettings};

use crate::core::common::{LayoutEnum, LayoutStrat, Side, intersect_rects};

static FONT_DATA: &[u8] = include_bytes!("../../fonts/JetBrainsMono-Regular.ttf");

lazy_static::lazy_static! {
    pub static ref JETBRAINS_FONT: Font = {
        Font::from_bytes(FONT_DATA, FontSettings::default()).expect("Ошибка загрузки шрифта")
    };
}

#[derive(Clone)]
pub struct Label {
    pub base: WidgetBase,
    pub text: Vec<String>,
    pub font_size: f32,
    pub textcolor: Color,
}

impl Label {
    pub fn new(id: String) -> Self {
        let mut label = Self {
            base: WidgetBase::new(regid(id)),
            text: Vec::new(),
            font_size: 14.0,
            textcolor: Color::BLACK,
        };
        label.base.bgcolor = Color::TRANSPARENT;
        label.update_size();
        label
    }

    pub fn update_size(&mut self) {
        let old_size = self.base.size;
        let mut max_width: f32 = 0.0;
        let mut height = 0.0;

        let line_metrics = JETBRAINS_FONT.horizontal_line_metrics(self.font_size);

        let lheight = line_metrics
            .map(|m| m.new_line_size)
            .unwrap_or(self.font_size);

        for line in &self.text {
            let mut lwidth = 0.0;
            for c in line.chars() {
                let metrics = JETBRAINS_FONT.metrics(c, self.font_size);
                lwidth += metrics.advance_width;
            }
            max_width = max_width.max(lwidth);
            height += lheight;
        }

        self.base.size = Size::new((max_width + 4.0) as i32, (height + 2.0) as i32);
        if old_size.width != self.base.size.width || old_size.height != self.base.size.height {
            self.set_relayout_flag(true);
        }
    }

    pub fn text(mut self, new_text: String) -> Self {
        self.text = new_text.split("\n").map(String::from).collect();
        self.update_size();
        self
    }

    pub fn get_text(&self) -> String {
        self.text.join("\n")
    }

    pub fn new_text(&mut self, new_text: String) {
        self.text = new_text.split("\n").map(String::from).collect();
        self.update_size();
        self.set_dirty_flag(true);
    }

    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size;
        self.update_size();
        self.set_relayout_flag(true);
        self.set_dirty_flag(true);
    }

    pub fn set_color(&mut self, color: Color) {
        self.textcolor = color;
        self.set_dirty_flag(true);
    }

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
    pub fn fill_x(mut self) -> Self {
        self.base.sizestrat.fill = ChooseCords::X;
        self
    }

    pub fn fill_y(mut self) -> Self {
        self.base.sizestrat.fill = ChooseCords::Y;
        self
    }

    pub fn fill_both(mut self) -> Self {
        self.base.sizestrat.fill = ChooseCords::BOTH;
        self
    }

    pub fn fill_none(mut self) -> Self {
        self.base.sizestrat.fill = ChooseCords::NONE;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.base.size = size;
        self.base.sizestrat.method = SizeEnum::MANUAL;
        self
    }

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
        let abs_x = (pos_off.x + self.base.pos.x) as f32;
        let abs_y = (pos_off.y + self.base.pos.y) as f32;

        /*
        println!(
            "LABEL DRAW: id={}, text={:?}, abs_pos=({}, {}), size=({}, {}), clip=({}, {}, {}, {})",
            self.base.id,
            self.text,
            abs_x,
            abs_y,
            self.base.size.width,
            self.base.size.height,
            clip.left(),
            clip.top(),
            clip.right(),
            clip.bottom()
        );*/

        if let Some(bg_rect) = Rect::from_xywh(
            abs_x,
            abs_y,
            self.base.size.width as f32,
            self.base.size.height as f32,
        ) {
            let mut bg_paint = Paint::default();
            bg_paint.set_color(SkiaColor::from_rgba8(
                self.base.bgcolor.b,
                self.base.bgcolor.g,
                self.base.bgcolor.r,
                self.base.bgcolor.a,
            ));
            if let Some(visible_bg) = intersect_rects(bg_rect, clip) {
                pixmap.fill_rect(
                    visible_bg,
                    &bg_paint,
                    tiny_skia::Transform::identity(),
                    None,
                );
            }
        }

        let line_metrics = JETBRAINS_FONT.horizontal_line_metrics(self.font_size);
        let lheight = line_metrics
            .map(|m| m.new_line_size)
            .unwrap_or(self.font_size);
        let ascent = line_metrics
            .map(|m| m.ascent)
            .unwrap_or(self.font_size * 0.75);

        let mut y_cursor = abs_y + 1.0;

        for line in &self.text {
            let mut x_cursor = abs_x + 2.0;
            let baseline = y_cursor + ascent;

            let img_w = pixmap.width() as i32;
            let img_h = pixmap.height() as i32;

            let pixels = pixmap.pixels_mut();

            for c in line.chars() {
                if c == ' ' {
                    let metrics = JETBRAINS_FONT.metrics(' ', self.font_size);
                    x_cursor += metrics.advance_width;
                    continue;
                }

                let (metrics, bitmap) = JETBRAINS_FONT.rasterize(c, self.font_size);

                if metrics.width > 0 && metrics.height > 0 {
                    let px_base = (x_cursor + metrics.xmin as f32).round() as i32;
                    let py_base =
                        (baseline - metrics.height as f32 - metrics.ymin as f32).round() as i32;

                    for row in 0..metrics.height {
                        let screen_y = py_base + row as i32;

                        if screen_y < clip.top() as i32
                            || screen_y >= clip.bottom() as i32
                            || screen_y < 0
                            || screen_y >= img_h
                        {
                            continue;
                        }

                        for col in 0..metrics.width {
                            let screen_x = px_base + col as i32;

                            if screen_x < clip.left() as i32
                                || screen_x >= clip.right() as i32
                                || screen_x < 0
                                || screen_x >= img_w
                            {
                                continue;
                            }

                            let alpha = bitmap[row * metrics.width + col] as u32;
                            if alpha == 0 {
                                continue;
                            }

                            let pixel_idx = (screen_y * img_w + screen_x) as usize;

                            let fg_color = tiny_skia::ColorU8::from_rgba(
                                self.textcolor.r,
                                self.textcolor.g,
                                self.textcolor.b,
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

                                let out_a = fg_a + (bg_a * (255 - fg_a) / 255);
                                let out_r = fg_r + (bg_r * (255 - fg_a) / 255);
                                let out_g = fg_g + (bg_g * (255 - fg_a) / 255);
                                let out_b = fg_b + (bg_b * (255 - fg_a) / 255);

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
                x_cursor += metrics.advance_width;
            }
            y_cursor += lheight;
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
    /*
    fn handle_event(&mut self, event: &Event, pos_off: Pos, ) -> Action {
        if let Event::MouseClick { pos } = event {
            if self.is_point_inside(*pos, pos_off) {
                return Action::None;
            }
        }
        Action::None
    }*/
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
        if self.is_dirty() {
            if let Some(dirty_rect) = self.get_self_rect(pos_off) {
                requests.push(Action::RedrawRequest(Some(dirty_rect)));
            }
            self.set_dirty_flag(false);
        }
    }
}
