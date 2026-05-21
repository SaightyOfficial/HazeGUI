use std::any::Any;

use crate::core::color::Color;
use crate::core::common::LayoutStrat;
use crate::core::common::SizeStrat;
use crate::core::event::Action;
use crate::core::event::Event;
use crate::core::pos::Pos;
use crate::core::size::Size;
use tiny_skia::{PixmapMut, Rect};

///Struct used to store used coordinates, used in frame's composing engine
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct UsedCord {
    pub used_y: i32,
    pub used_x: i32,
    pub fill_widgets: i32,
}

///Struct used to store info that every widget should have
#[derive(Clone)]
pub struct WidgetBase {
    pub id: String,
    pub pos: Pos,
    pub size: Size,
    pub bgcolor: Color,
    pub layoutstrat: LayoutStrat,
    pub sizestrat: SizeStrat,
    pub needs_relayout: bool,
    pub is_dirty: bool,
}

impl WidgetBase {
    pub fn new(id_new: String) -> Self {
        Self {
            id: id_new,
            pos: Pos::new(0, 0),
            size: Size::new(0, 0),
            bgcolor: Color::LIGHT_GRAY,
            layoutstrat: LayoutStrat::default(),
            sizestrat: SizeStrat::default(),
            needs_relayout: true,
            is_dirty: true,
        }
    }
}

pub trait Widget: Any {
    fn get_id(&self) -> &str; //Getting widgets id
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn draw(
        &self,
        pixmap: &mut PixmapMut,
        pos_off: Pos,
        clip: Rect,
        preferred_color: Option<Color>,
    );
    fn is_point_inside(&self, global_point: Pos, parent_off: Pos) -> bool {
        let abs_x = parent_off.x + self.get_pos().x;
        let abs_y = parent_off.y + self.get_pos().y;
        let size = self.get_size();

        global_point.x >= abs_x
            && global_point.x <= abs_x + size.width
            && global_point.y >= abs_y
            && global_point.y <= abs_y + size.height
    }
    fn on_click(&mut self) {}
    fn on_hover(&mut self) {}
    fn update_layout(&mut self, forced: bool);
    fn set_size(&mut self, size: Size);
    fn set_pos(&mut self, pos: Pos);
    fn get_size(&self) -> Size;
    fn get_self_rect(&self, pos_off: Pos) -> Option<tiny_skia::Rect> {
        let abs_x = (pos_off.x + self.get_pos().x) as f32;
        let abs_y = (pos_off.y + self.get_pos().y) as f32;
        let size = self.get_size();
        if let Some(dirty_rect) =
            tiny_skia::Rect::from_xywh(abs_x, abs_y, size.width as f32, size.height as f32)
        {
            return Some(dirty_rect);
        }
        None
    }
    fn is_dirty(&self) -> bool;
    fn set_dirty_flag(&mut self, flag: bool);
    fn get_pos(&self) -> Pos;
    fn get_global_pos(&self, parent_off: Pos) -> Pos {
        Pos::new(
            parent_off.x + self.get_pos().x,
            parent_off.y + self.get_pos().y,
        )
    }
    fn get_layout_strat(&self) -> LayoutStrat;
    fn get_size_strat(&self) -> SizeStrat;
    fn needs_relayout(&self) -> bool;
    fn set_relayout_flag(&mut self, flag: bool);
    fn get_dirty_rect(&mut self, pos_off: Pos, actions: &mut Vec<Action>);
    fn handle_event(&mut self, event: &Event, pos_off: Pos, actions: &mut Vec<Action>);
}
