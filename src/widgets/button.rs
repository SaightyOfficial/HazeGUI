use crate::core::event::{Action, Event};
use crate::core::size::Size;
use crate::core::{color::Color, pos::Pos};
use crate::core::widget::Widget;
use crate::widgets::frame::{Frame, FrameStyle};
use crate::widgets::label::Label;
use crate::core::common::{LayoutStrat, SizeStrat};
use tiny_skia::{PixmapMut, Rect};

pub struct Button {
    pub id: String,
    pub frame: Frame,
    pub text: Label,
    is_hovered:bool,
    is_pressed:bool,
}

impl Button {
    pub fn new(id: String) -> Self {
        Self {
            id: id.clone(),
            frame: Frame::new(format!("{}.frame", id.clone())).style(FrameStyle::RAISED),
            text: Label::new(format!("{}.label", id.clone())).bgcolor(Color::TRANSPARENT),
            is_hovered: false,
            is_pressed: false,
        }
    }
    pub fn text(mut self, new_text: &str) -> Self {
        self.text.text = new_text.to_string();
        self.update_layout();
        self
    }

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
}

impl Widget for Button {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn get_id(&self) -> &str { &self.id }
    fn draw(&self, pixmap: &mut PixmapMut, pos_off: Pos, clip: Rect) {
        //let display_color = if self.is_hovered {
        //    self.frame.base.bgcolor.clone().lighter(30)
        //} else {
        //    self.frame.base.bgcolor.clone()
        //};

        //println!("{:?}", self.frame.base.bgcolor.clone());

        self.frame.draw(pixmap, pos_off, clip);
    }

    fn update_layout(&mut self) {
        self.frame.children.clear();
        
        self.text.update_size(); 
        //println!("wid{} hei{}", self.frame.base.size.width.clone(), self.frame.base.size.height.clone());
        
        let text_size = self.text.get_size();
        self.frame.set_size(text_size); 

        
        self.frame.add_widget(self.text.clone());
        self.frame.update_layout();
    }

    fn set_size(&mut self, size: Size) { 
        self.frame.base.size = size; 
        self.update_layout(); 
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
    /*
    fn handle_event(&mut self, event: &Event, pos_off: Pos) -> bool {
        match event {
            Event::MouseClick { pos } => {
                if self.is_point_inside(*pos, pos_off) {
                    self.on_click(); // Вызываем наш метод с lock()
                    return true;
                }
            }
            Event::MouseMove { pos } => {
                let currently_inside = self.is_point_inside(*pos, pos_off);
                
                // Если мышка только что зашла в зону кнопки
                if currently_inside && !self.is_hovered {
                    self.is_hovered = true;
                    self.on_hover(); // Вызываем коллбэк ховера один раз
                    return true;
                } 
                // Если мышка вышла из зоны кнопки
                else if !currently_inside && self.is_hovered {
                    self.is_hovered = false;
                }

                // Если мы просто внутри — возвращаем true, чтобы клик под кнопкой не сработал
                if currently_inside { return true; }
            }
        }
        false
    }*/
    fn handle_event(&mut self, event: &Event, pos_off: Pos, actions: &mut Vec<Action>) {
        match event {
            Event::MouseClick { pos } => {
                if self.is_point_inside(*pos, pos_off) {
                    self.frame.style = FrameStyle::SUNKEN;
                    self.is_pressed = true;
                    actions.push(Action::ButtonClicked(self.id.clone()));
                }
            }
            Event::MouseRelease { pos } => {
                if self.is_point_inside(*pos, pos_off) {
                    self.frame.style = FrameStyle::RAISED;
                    self.is_pressed = false;
                    actions.push(Action::ButtonReleased(self.id.clone()));
                }
            }
            Event::MouseMove { pos } => {
                let now_hovered = self.is_point_inside(*pos, pos_off);
                if now_hovered && !self.is_hovered {
                    self.is_hovered = true;
                    actions.push(Action::Hovered(self.id.clone()));
                } else if !now_hovered && self.is_hovered {
                    self.is_hovered = false;
                    self.frame.style = FrameStyle::RAISED;
                    self.is_pressed = false;
                    actions.push(Action::Unhovered(self.id.clone()));
                }
            }
        }
    }
}