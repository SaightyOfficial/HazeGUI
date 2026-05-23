use crate::core::common::{Axis, LayoutEnum, LayoutStrat, Side, SizeEnum, SizeStrat};
use crate::core::event::{Action, Event};
use crate::core::idpool::regid;
use crate::core::size::Size;
use crate::core::widget::Widget;
use crate::core::{color::Color, pos::Pos};
use crate::widgets::frame::{Frame, FrameStyle};
use crate::widgets::label::Label;
use tiny_skia::{PixmapMut, Rect};

pub struct Button {
    pub id: u64,
    pub frame: Frame,
    pub text: Label,
    pub hover_color: Option<Color>,
    is_hovered: bool,
    is_pressed: bool,
}

impl Button {
    pub fn new(id: String) -> Self {
        Self {
            id: regid(id.clone()),
            frame: Frame::new(format!("{}.frame", id.clone())).style(FrameStyle::RAISED),
            text: Label::new(format!("{}.label", id.clone())).bgcolor(Color::TRANSPARENT),
            hover_color: None,
            is_hovered: false,
            is_pressed: false,
        }
    }
    pub fn text(mut self, new_text: &str) -> Self {
        self.text.new_text(new_text.into());
        self.update_layout(false);
        self
    }

    pub fn new_text(&mut self, new_text: String) {
        self.text.new_text(new_text);
        self.text.update_size();
    }

    //positions
    pub fn pos(mut self, posnew: Pos) -> Self {
        self.frame.base.pos = posnew;
        self.frame.base.layoutstrat.method = LayoutEnum::MANUAL;
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.frame.base.layoutstrat.side = side;
        self
    }

    //colors
    pub fn color(mut self, new_color: Color) -> Self {
        //self.text.base.bgcolor = new_color;
        self.frame.base.bgcolor = new_color;
        //println!("{:?}", self.frame.base.bgcolor.clone());
        self
    }

    pub fn textcolor(mut self, new_color: Color) -> Self {
        self.text.textcolor = new_color;
        self
    }

    pub fn hovercolor(mut self, new_color: Color) -> Self {
        self.hover_color = Some(new_color);
        self
    }

    //sizes
    ///By which axises widget will stretch and fill itself
    pub fn fill(mut self, cords: Axis) -> Self {
        self.frame.base.sizestrat.fill = cords;
        if cords == Axis::NONE {
            self.frame.base.sizestrat.method = SizeEnum::AUTO;
        } else {
            self.frame.base.sizestrat.method = SizeEnum::FILL;
        }
        self
    }

    ///Changes at runtime by which axises widget will stretch and fill itself
    pub fn set_fill(&mut self, cords: Axis){
        self.frame.base.sizestrat.fill = cords;
        if cords == Axis::NONE {
            self.frame.base.sizestrat.method = SizeEnum::AUTO;
        } else {
            self.frame.base.sizestrat.method = SizeEnum::FILL;
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.frame.base.size = size;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.text.font_size = size;
        self
    }

    pub fn max_size(mut self, w: Option<i32>, h: Option<i32>) -> Self {
        self.frame.base.sizestrat.max_width = w;
        self.frame.base.sizestrat.max_height = h;
        self
    }

    pub fn set_max_size(&mut self, w: Option<Option<i32>>, h: Option<Option<i32>>) {
        if let Some(width) = w {
            self.frame.base.sizestrat.max_width = width;
        }
        if let Some(height) = h {
            self.frame.base.sizestrat.max_height = height;
        }
    }
}

impl Widget for Button {
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
        let display_color = if self.is_hovered {
            self.hover_color
                .unwrap_or_else(|| self.frame.base.bgcolor.lighter(25))
        } else {
            self.frame.base.bgcolor
        };

        //println!("{:?}", display_color);

        self.frame.draw(pixmap, pos_off, clip, Some(display_color));
    }

    fn update_layout(&mut self, forced: bool) {
        self.frame.children.clear();

        self.text.update_size();

        if self.frame.get_size_strat().method == SizeEnum::AUTO {
            let text_size = self.text.get_size();
            self.frame.base.size = text_size;
        }

        self.text.base.layoutstrat.side = Side::MIDDLE;

        self.frame.add_widget(self.text.clone());

        self.frame.update_layout(forced);
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
        self.text.is_dirty() || self.frame.is_dirty()
    }
    fn set_dirty_flag(&mut self, flag: bool) {
        self.frame.set_dirty_flag(flag);
        self.text.base.is_dirty = flag;
    }
    fn needs_relayout(&self) -> bool {
        self.frame.base.needs_relayout || self.text.base.needs_relayout
    }
    fn set_relayout_flag(&mut self, flag: bool) {
        self.frame.set_relayout_flag(flag);
        self.text.base.needs_relayout = flag;
    }
    fn handle_event(&mut self, event: &Event, pos_off: Pos, actions: &mut Vec<Action>) {
        match event {
            Event::MouseClick { pos } => {
                if self.is_point_inside(*pos, pos_off) {
                    self.is_pressed = true;
                    actions.push(Action::ButtonClicked(self.id));
                    self.frame.set_style(FrameStyle::SUNKEN);
                }
            }
            Event::MouseRelease { pos } => {
                if self.is_point_inside(*pos, pos_off) {
                    self.is_pressed = false;
                    actions.push(Action::ButtonReleased(self.id));
                    self.frame.set_style(FrameStyle::RAISED);
                }
            }
            Event::MouseMove { pos } => {
                let now_hovered = self.is_point_inside(*pos, pos_off);
                if now_hovered && !self.is_hovered {
                    self.is_hovered = true;
                    actions.push(Action::Hovered(self.id));
                    self.set_dirty_flag(true);
                } else if !now_hovered && self.is_hovered {
                    self.is_hovered = false;
                    self.frame.set_style(FrameStyle::RAISED);
                    self.is_pressed = false;
                    actions.push(Action::Unhovered(self.id));
                    self.set_dirty_flag(true);
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
