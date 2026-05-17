use crate::core::event::{Action, Event};
use crate::core::size::Size;
use crate::core::{color::Color, pos::Pos};
use crate::core::widget::Widget;
use crate::widgets::frame::{Frame, FrameStyle};
use crate::widgets::label::Label;
use crate::core::common::{ChooseCords, LayoutEnum, LayoutStrat, Side, SizeStrat};
use tiny_skia::{PixmapMut, Rect};

pub struct Button {
    pub id: String,
    pub frame: Frame,
    pub text: Label,
    pub hover_color: Option<Color>,
    is_hovered:bool,
    is_pressed:bool,
}

impl Button {
    pub fn new(id: String) -> Self {
        Self {
            id: id.clone(),
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
    pub fn fill_x(mut self) -> Self {
        self.frame.base.sizestrat.fill = ChooseCords::X;
        self
    }

    pub fn fill_y(mut self) -> Self {
        self.frame.base.sizestrat.fill = ChooseCords::Y;
        self
    }

    pub fn fill_both(mut self) -> Self {
        self.frame.base.sizestrat.fill = ChooseCords::BOTH;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.frame.base.size = size;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.text.font_size = size;
        self
    }
}

impl Widget for Button {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn get_id(&self) -> &str { &self.id }
    fn draw(&self, pixmap: &mut PixmapMut, pos_off: Pos, clip: Rect, _preferred_color: Option<Color>) {
        let display_color = if self.is_hovered {
            let hover_color = self.hover_color
                .unwrap_or_else(|| self.frame.base.bgcolor.lighter(25));
            hover_color
        } else {
            self.frame.base.bgcolor
        };

        //println!("{:?}", display_color);

        self.frame.draw(pixmap, pos_off, clip, Some(display_color));
    }

    fn update_layout(&mut self, forced: bool) {
        self.frame.children.clear();
        
        self.text.update_size(); 
        //println!("wid{} hei{}", self.frame.base.size.width, self.frame.base.size.height);
        
        let text_size = self.text.get_size();
        self.frame.set_size(text_size); 
        
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
        self.frame.base.is_dirty = flag;
        if flag == false {
            self.text.base.is_dirty = false;
            if let Some(widget) = self.frame.find_mut(&self.text.base.id) {
                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                    widget.set_dirty_flag(false);
                }
            }
        }
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
                    actions.push(Action::ButtonClicked(self.id.clone()));
                    self.frame.set_style(FrameStyle::SUNKEN);
                }
            }
            Event::MouseRelease { pos } => {
                if self.is_point_inside(*pos, pos_off) {
                    self.is_pressed = false;
                    actions.push(Action::ButtonReleased(self.id.clone()));
                    self.frame.set_style(FrameStyle::RAISED);
                }
            }
            Event::MouseMove { pos } => {
                let now_hovered = self.is_point_inside(*pos, pos_off);
                if now_hovered && !self.is_hovered {
                    self.is_hovered = true;
                    actions.push(Action::Hovered(self.id.clone()));
                    self.set_dirty_flag(true);
                } else if !now_hovered && self.is_hovered {
                    self.is_hovered = false;
                    self.frame.set_style(FrameStyle::RAISED);
                    self.is_pressed = false;
                    actions.push(Action::Unhovered(self.id.clone()));
                    self.set_dirty_flag(true);
                }
            }
        }
        if self.is_dirty() {
            if let Some(dirty_rect) = self.get_self_rect(pos_off) {
                actions.push(Action::RedrawRequest(Some(dirty_rect)));
            }
            self.set_dirty_flag(false);
        }
    }
}