use crate::core::common::{Axis, LayoutEnum, LayoutStrat, Side, SizeEnum, SizeStrat, intersect_rects};
use crate::core::event::{Action, Event, KKey, MKey};
use crate::core::idpool::{regid, get_id};
use crate::hsid;
use crate::core::size::Size;
use crate::core::widget::Widget;
use crate::core::{color::Color, pos::Pos};
use crate::widgets::frame::{Frame, FrameStyle};
use crate::widgets::label::Label;
use tiny_skia::{Color as SkiaColor, Paint, PixmapMut, Rect};

///Buton struct, stores everything textbox needs
pub struct Textbox {
    pub id: u64,
    pub frame: Frame,
    pub hover_color: Option<Color>,
    is_focused: bool,
    pub cursor_pos: usize,
}

impl Textbox {
    pub fn new(id: String) -> Self {
        let mut framesetter = Frame::new(format!("{}.frame", id.clone()));
        framesetter.style(FrameStyle::SUNKEN);

        let mut textsetter = Label::new(format!("{}.label", id.clone()));
        textsetter.bgcolor(Color::TRANSPARENT);
        textsetter.side(Side::LEFT);

        framesetter.add_widget(textsetter);

        Self {
            id: regid(id.clone()),
            frame: framesetter,
            hover_color: None,
            is_focused: false,
            cursor_pos: 0,
        }
    }

    pub fn text(&mut self, new_text: String) {
        if let Some(id) = get_id(self.id.clone()) {
            let labelid = format!("{}.label", id.clone());

            if let Some(widget) = self.frame.find_mut(hsid!(&labelid)) {
                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                    label.text(new_text);
                }
            }
        }
    }

    //positions
    ///Manualy set widget position, widget will not participate in auto layout composing and just be where you said it to be
    pub fn pos(&mut self, posnew: Pos) {
        self.frame.base.pos = posnew;
        self.frame.base.layoutstrat.method = LayoutEnum::MANUAL;
        self.set_relayout_flag(true);
    }

    ///Changes widget side at runtime, there are only [`Side::LEFT`], [`Side::MIDDLE`] and [`Side::RIGHT`], Y axis position depends on order by which widgets are added in your code
    pub fn side(&mut self, side: Side) {
        self.frame.base.layoutstrat.side = side;
        self.set_relayout_flag(true);
    }

    //colors
    ///Sets button color 
    pub fn color(&mut self, new_color: Color) {
        //self.text.base.bgcolor = new_color;
        self.frame.base.bgcolor = new_color;
        //println!("{:?}", self.frame.base.bgcolor.clone());
        self.set_dirty_flag(true);
    }

    ///Sets text color
    pub fn textcolor(&mut self, new_color: Color) {
        if let Some(id) = get_id(self.id.clone()) {
            let labelid = format!("{}.label", id.clone());

            if let Some(widget) = self.frame.find_mut(hsid!(&labelid)) {
                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                    label.color(new_color);
                }
            }
        }
    }

    ///Sets hover color
    pub fn hovercolor(&mut self, new_color: Color) {
        self.hover_color = Some(new_color);
        self.set_dirty_flag(true);
    }

    //sizes
    ///Changes at runtime by which axises widget will stretch and fill itself
    pub fn fill(&mut self, cords: Axis) {
        self.frame.base.sizestrat.fill = cords;
        if cords == Axis::NONE {
            self.frame.base.sizestrat.method = SizeEnum::AUTO;
        } else {
            self.frame.base.sizestrat.method = SizeEnum::FILL;
        }
    }

    pub fn size(&mut self, size: Size) {
        self.frame.base.size = size;
        self.frame.base.sizestrat.method = SizeEnum::MANUAL;
        self.set_relayout_flag(true);
    }

    pub fn auto_size(&mut self) {
        self.frame.base.sizestrat.method = SizeEnum::AUTO;
        self.set_relayout_flag(true);
    }

    pub fn font_size(&mut self, size: f32) {
        if let Some(id) = get_id(self.id.clone()) {
            let labelid = format!("{}.label", id.clone());

            if let Some(widget) = self.frame.find_mut(hsid!(&labelid)) {
                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                    label.font_size(size);
                }
            }
        }
    }

    ///Changes max widget size at runtime
    /// 
    ///Example arguments:
    ///None - Dont tourch tha axis
    ///Some(None) - Removes limit
    ///Some(Some(i32)) - Sets limit
    pub fn max_size(&mut self, w: Option<Option<i32>>, h: Option<Option<i32>>) {
        if let Some(width) = w {
            self.frame.base.sizestrat.max_width = width;
        }
        if let Some(height) = h {
            self.frame.base.sizestrat.max_height = height;
        }
    }

    ///Changes minimal widget size at runtime
    /// 
    ///Example arguments:
    ///None - Dont change that axis
    ///Some(None) - Removes limit
    ///Some(Some(i32)) - Sets limit
    pub fn min_size(&mut self, w: Option<Option<i32>>, h: Option<Option<i32>>) {
        if let Some(width) = w {
            self.frame.base.sizestrat.min_width = width;
        }
        if let Some(height) = h {
            self.frame.base.sizestrat.min_height = height;
        }
    }
}

impl Widget for Textbox {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn get_id(&self) -> u64 {
        self.id
    }
    fn draw(
        &self,
        pixmap: &mut PixmapMut,
        pos_off: Pos,
        clip: Rect,
        _preferred_color: Option<Color>,
    ) {
        self.frame.draw(pixmap, pos_off, clip, None);

        if !self.is_focused {
            return;
        }

        if let Some(id) = get_id(self.id) {
            let labelid = format!("{}.label", id);

            if let Some(widget) = self.frame.find(hsid!(&labelid)) {
                if let Some(label) = widget.as_any().downcast_ref::<Label>() {
                    let (rel_pos, cursor_h) = label.get_cursor_pos(self.cursor_pos);

                    let abs_x = (pos_off.x + self.frame.base.pos.x + label.base.pos.x + rel_pos.x) as f32;
                    let abs_y = (pos_off.y + self.frame.base.pos.y + label.base.pos.y + rel_pos.y) as f32;
                    let cursor_w = 1.0;

                    if let Some(cursor_rect) = Rect::from_xywh(abs_x, abs_y, cursor_w, cursor_h as f32) {
                        if let Some(visible_cursor) = intersect_rects(cursor_rect, clip) {
                            let mut paint = Paint::default();
                            paint.set_color(SkiaColor::from_rgba8(
                                label.textcolor.b,
                                label.textcolor.g,
                                label.textcolor.r,
                                255,
                            ));

                            pixmap.fill_rect(
                                visible_cursor,
                                &paint,
                                tiny_skia::Transform::identity(),
                                None,
                            );
                        }
                    }
                }
            }
        }
    }

    fn update_layout(&mut self, forced: bool) {
        self.frame.update_layout(forced);//Layout update
    }

    fn set_size(&mut self, size: Size) {
        self.frame.base.size = size;
        self.update_layout(false);
    }
    fn set_pos(&mut self, pos: Pos) {
        self.frame.base.pos = pos;
    }
    fn get_size(&self) -> Size {
        self.frame.get_size()
    }
    fn get_pos(&self) -> Pos {
        self.frame.get_pos()
    }
    fn get_layout_strat(&self) -> LayoutStrat {
        self.frame.get_layout_strat()
    }
    fn get_size_strat(&self) -> SizeStrat {
        self.frame.get_size_strat()
    }
    fn is_dirty(&self) -> bool {
        self.frame.is_dirty()
    }
    fn set_dirty_flag(&mut self, flag: bool) {
        self.frame.set_dirty_flag(flag);
    }
    fn needs_relayout(&self) -> bool {
        self.frame.base.needs_relayout
    }
    fn set_relayout_flag(&mut self, flag: bool) {
        self.frame.set_relayout_flag(flag);
    }
    fn handle_event(&mut self, event: &Event, pos_off: Pos, actions: &mut Vec<Action>) {
        match event {
            Event::MouseClick { pos, key } => {
                if self.is_point_inside(*pos, pos_off) && *key == MKey::Left {
                    //if !self.is_focused {
                        self.is_focused = true;
                        actions.push(Action::TextboxFocus(self.id));
                    //} else {
                        if let Some(id) = get_id(self.id) {
                            let labelid = format!("{}.label", id);
                            let x_pos = self.frame.base.pos.x;

                            if let Some(widget) = self.frame.find_mut(hsid!(&labelid)) {
                                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                                    // 1. Считаем локальный X относительно начала текста в Label
                                    let label_abs_x = pos_off.x + x_pos + label.base.pos.x;
                                    let local_x = (pos.x - label_abs_x).max(0) as f32;

                                    // 2. Спрашиваем у Label индекс символа по этому X
                                    // (метод get_char_index_at_x тебе нужно будет добавить в Label)
                                    self.cursor_pos = label.get_char_index_at_x(local_x);
                                }
                            }
                        }
                    //}
                    self.set_dirty_flag(true);
                } else {
                    self.is_focused = false;

                    /*
                    if let Some(id) = get_id(self.id) {
                        let labelid = format!("{}.label", id);

                        if let Some(widget) = self.frame.find(hsid!(&labelid)) {
                            if let Some(label) = widget.as_any().downcast_ref::<Label>() {
                                self.cursor_pos = label.get_text_ref().len();
                            }
                        }
                    }*/

                    actions.push(Action::TextboxUnfocus(self.id));
                    self.set_dirty_flag(true);
                }
            }
            Event::KeyPress { ch, key } => {
                if self.is_focused {
                    if let Some(id) = get_id(self.id.clone()) {
                        let labelid = format!("{}.label", id.clone());

                        if let Some(widget) = self.frame.find_mut(hsid!(&labelid)) {
                            if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                                let text = label.get_text_mutref();

                                self.cursor_pos = self.cursor_pos.min(text.len());

                                if *ch != '\0' && *ch != '\u{8}' && *ch != '\u{7f}' {
                                    text.insert(self.cursor_pos, *ch);
                                    self.cursor_pos += ch.len_utf8();

                                    label.update_size();
                                    actions.push(Action::TextboxTextChange(self.id));
                                    self.set_dirty_flag(true);
                                } else {
                                    match key {
                                        KKey::Backspace => {
                                            if self.cursor_pos > 0 {
                                                if let Some((prev_idx, _)) = text[..self.cursor_pos].char_indices().last() {
                                                    text.remove(prev_idx);
                                                    self.cursor_pos = prev_idx;

                                                    label.update_size();
                                                    actions.push(Action::TextboxTextChange(self.id));
                                                    self.set_dirty_flag(true);
                                                }
                                            }
                                        }
                                        KKey::Delete => {
                                            if self.cursor_pos < text.len() {
                                                text.remove(self.cursor_pos);

                                                label.update_size();
                                                actions.push(Action::TextboxTextChange(self.id));
                                                self.set_dirty_flag(true);
                                            }
                                        }
                                        KKey::ArrowLeft => {
                                            if self.cursor_pos > 0 {
                                                if let Some((prev_idx, _)) = text[..self.cursor_pos].char_indices().last() {
                                                    self.cursor_pos = prev_idx;
                                                    self.set_dirty_flag(true);
                                                }
                                            }
                                        }
                                        KKey::ArrowRight => {
                                            if self.cursor_pos < text.len() {
                                                if let Some((_rel_idx, c)) = text[self.cursor_pos..].char_indices().next() {
                                                    self.cursor_pos += c.len_utf8();
                                                    self.set_dirty_flag(true);
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
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
