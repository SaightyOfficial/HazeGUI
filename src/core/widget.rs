use std::any::Any;

use tiny_skia::{PixmapMut, Rect};
use crate::core::event::Action;
use crate::core::pos::Pos;
use crate::core::size::Size;
use crate::core::common::LayoutStrat;
use crate::core::common::SizeStrat;
use crate::core::color::Color;
use crate::core::event::Event;

#[derive(Clone)]
pub struct UsedCord {
    pub used_y: i32,
    pub used_x: i32,
}

impl Default for UsedCord {
    fn default() -> Self {
        Self{
            used_x: 0,
            used_y: 0,
        }
    }
}

#[derive(Clone)]
pub struct WidgetBase {
    pub id: String,
    pub pos: Pos,
    pub size: Size,
    pub bgcolor: Color,
    pub layoutstrat: LayoutStrat,
    pub sizestrat: SizeStrat,
}

impl WidgetBase {
    pub fn new(id_new: String) -> Self {
        Self{
            id: id_new,
            pos: Pos::new(0, 0),
            size: Size::new(0, 0),
            bgcolor: Color::LIGHT_GRAY,
            layoutstrat: LayoutStrat::default(),
            sizestrat: SizeStrat::default(),
        }
    }
}

pub trait Widget: Any {
    fn get_id(&self) -> &str;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn draw(&self, pixmap: &mut PixmapMut, pos_off: Pos, rect: Rect);
    fn is_point_inside(&self, global_point: Pos, parent_off: Pos) -> bool {
        let abs_x = parent_off.x + self.get_pos().x;
        let abs_y = parent_off.y + self.get_pos().y;
        let size = self.get_size();

        global_point.x >= abs_x && 
        global_point.x <= abs_x + size.width &&
        global_point.y >= abs_y && 
        global_point.y <= abs_y + size.height
    }
    fn on_click(&mut self) {}
    fn on_hover(&mut self) {}
    fn update_layout(&mut self) {}
    fn set_size(&mut self, size: Size);
    fn set_pos(&mut self, pos: Pos);
    fn get_size(&self) -> Size;
    fn get_pos(&self) -> Pos;
    fn get_global_pos(&self, parent_off: Pos) -> Pos {
        Pos::new(
            parent_off.x + self.get_pos().x,
            parent_off.y + self.get_pos().y
        )
    }
    fn get_layout_strat(&self) -> LayoutStrat;
    fn handle_event(&mut self, event: &Event, pos_off: Pos, actions: &mut Vec<Action>);
}