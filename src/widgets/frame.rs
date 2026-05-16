use crate::core::size::Size;
use crate::core::event::{Action, Event};
use crate::core::{color::Color, pos::Pos};
use crate::core::common::{ChooseCords};
use crate::core::widget::{UsedCord, Widget, WidgetBase};
use tiny_skia::{PixmapMut, Paint, Rect, Color as SkiaColor};

use crate::core::common::{LayoutEnum, LayoutStrat, Side, intersect_rects};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FrameStyle {
    FLAT,
    RAISED,
    SUNKEN,
    GROOVE,
    RIDGE,
}

pub struct Frame {
    pub base: WidgetBase,
    pub style: FrameStyle,
    pub children: Vec<Box<dyn Widget>>,
    pub usedleft: UsedCord,
    pub usedmiddle: UsedCord,
    pub usedright: UsedCord,
    pub totalused: UsedCord,
}

impl Frame {
    pub fn new(id:String) -> Self {
        Self {
            base: WidgetBase::new(id),
            style: FrameStyle::FLAT,
            children: Vec::new(),
            usedmiddle: UsedCord::default(),
            usedleft: UsedCord::default(),
            usedright: UsedCord::default(),
            totalused: UsedCord::default(),
        }
    }

    pub fn find_mut(&mut self, target_id: &str) -> Option<&mut dyn Widget> {
        if self.base.id == target_id {
            return Some(self);
        }
        for child in &mut self.children {
            if child.get_id() == target_id {
                return Some(&mut **child);
            }
            if let Some(frame) = child.as_any_mut().downcast_mut::<Frame>() {
                if let Some(found) = frame.find_mut(target_id) {
                    return Some(found);
                }
            }
        }
        None
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

    //colors and styles
    pub fn color(mut self, color: Color) -> Self {
        self.base.bgcolor = color;
        self
    }

    pub fn style(mut self, style: FrameStyle) -> Self {
        self.style = style;
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

    pub fn size(mut self, size: Size) -> Self {
        self.base.size = size;
        self
    }

    pub fn refresh_layout(&mut self) {
        self.usedmiddle = UsedCord::default();
        self.usedleft = UsedCord::default();
        self.usedright = UsedCord::default();
        let mut max_child_width = 0;
        let mut current_total_height = 0;

        let border_thickness = if self.style == FrameStyle::FLAT { 0 } else { 2 };
        let border_padding = border_thickness * 2;

        for child in &mut self.children {
            child.update_layout(); 
            let child_size = child.get_size();
            
            if child_size.width > max_child_width {
                max_child_width = child_size.width;
            }
            current_total_height += child_size.height;
        }

        let required_width = max_child_width + border_padding;
        let required_height = current_total_height + border_padding;

        if self.base.size.width < required_width {
            self.base.size.width = required_width;
        }
        if self.base.size.height < required_height {
            self.base.size.height = required_height;
        }

        let parent_width = self.base.size.width - border_padding;
        self.usedmiddle.used_y = border_thickness;

        for child in &mut self.children {
            let layoutstrat = child.get_layout_strat();
            let child_size = child.get_size();

            if layoutstrat.method == LayoutEnum::AUTO && layoutstrat.side == Side::MIDDLE {
                let middlepos = border_thickness + (parent_width - child_size.width) / 2;
                child.set_pos(Pos::new(middlepos, self.usedmiddle.used_y));
                self.usedmiddle.used_y += child_size.height;
            }
        }
    }

    pub fn add_widget<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget));
        self.update_layout();
    }
}

impl Widget for Frame {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn get_id(&self) -> &str { &self.base.id }
    fn draw(&self, pixmap: &mut PixmapMut, pos_off: Pos, clip: Rect) {
        let abs_x = (pos_off.x + self.base.pos.x) as f32;
        let abs_y = (pos_off.y + self.base.pos.y) as f32;
        let w = self.base.size.width as f32;
        let h = self.base.size.height as f32;

        let my_rect = Rect::from_xywh(
            abs_x, 
            abs_y, 
            self.base.size.width as f32, 
            self.base.size.height as f32
        ).unwrap();

        let inner_clip = match intersect_rects(clip, my_rect) {
            Some(r) => r,
            None => return,
        };

        //BGRA is needed here
        let mut paint = Paint::default();
        paint.set_color(SkiaColor::from_rgba8(self.base.bgcolor.b, self.base.bgcolor.g, self.base.bgcolor.r, self.base.bgcolor.a));

        let mut paintdark = Paint::default();
        let darkercolor = self.base.bgcolor.clone().darker(75);
        paintdark.set_color(SkiaColor::from_rgba8(darkercolor.b, darkercolor.g, darkercolor.r, darkercolor.a));

        let mut paintlight = Paint::default();
        let lightercolor = self.base.bgcolor.clone().lighter(75);
        paintlight.set_color(SkiaColor::from_rgba8(lightercolor.b, lightercolor.g, lightercolor.r, lightercolor.a));

        let border_thickness = if self.style == FrameStyle::FLAT { 0.0 } else { 2.0 };
        
        if let Some(visible_part) = intersect_rects(my_rect, clip) {
            pixmap.fill_rect(visible_part, &paint, tiny_skia::Transform::identity(), None);
            if border_thickness != 0.0 {
                match self.style {
                    FrameStyle::FLAT => {}
                    FrameStyle::RAISED => {
                        pixmap.fill_rect(Rect::from_xywh(abs_x, abs_y, w, 1.0).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x, abs_y, 1.0, h).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                        
                        pixmap.fill_rect(Rect::from_xywh(abs_x + 1.0, abs_y + 1.0, w - 2.0, 1.0).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + 1.0, abs_y + 1.0, 1.0, h - 2.0).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);

                        pixmap.fill_rect(Rect::from_xywh(abs_x, abs_y + h - 1.0, w, 1.0).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + w - 1.0, abs_y, 1.0, h).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                        
                        pixmap.fill_rect(Rect::from_xywh(abs_x + 1.0, abs_y + h - 2.0, w - 2.0, 1.0).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + w - 2.0, abs_y + 1.0, 1.0, h - 2.0).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                    }

                    FrameStyle::SUNKEN => {
                        pixmap.fill_rect(Rect::from_xywh(abs_x, abs_y, w, 1.0).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x, abs_y, 1.0, h).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + 1.0, abs_y + 1.0, w - 2.0, 1.0).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + 1.0, abs_y + 1.0, 1.0, h - 2.0).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);

                        pixmap.fill_rect(Rect::from_xywh(abs_x, abs_y + h - 1.0, w, 1.0).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + w - 1.0, abs_y, 1.0, h).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + 1.0, abs_y + h - 2.0, w - 2.0, 1.0).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + w - 2.0, abs_y + 1.0, 1.0, h - 2.0).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                    }

                    FrameStyle::GROOVE => {
                        pixmap.fill_rect(Rect::from_xywh(abs_x, abs_y, w, 1.0).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x, abs_y, 1.0, h).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x, abs_y + h - 1.0, w, 1.0).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + w - 1.0, abs_y, 1.0, h).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);

                        pixmap.fill_rect(Rect::from_xywh(abs_x + 1.0, abs_y + 1.0, w - 2.0, 1.0).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + 1.0, abs_y + 1.0, 1.0, h - 2.0).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + 1.0, abs_y + h - 2.0, w - 2.0, 1.0).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + w - 2.0, abs_y + 1.0, 1.0, h - 2.0).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                    }

                    FrameStyle::RIDGE => {
                        pixmap.fill_rect(Rect::from_xywh(abs_x, abs_y, w, 1.0).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x, abs_y, 1.0, h).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x, abs_y + h - 1.0, w, 1.0).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + w - 1.0, abs_y, 1.0, h).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);

                        pixmap.fill_rect(Rect::from_xywh(abs_x + 1.0, abs_y + 1.0, w - 2.0, 1.0).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + 1.0, abs_y + 1.0, 1.0, h - 2.0).unwrap(), &paintdark, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + 1.0, abs_y + h - 2.0, w - 2.0, 1.0).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                        pixmap.fill_rect(Rect::from_xywh(abs_x + w - 2.0, abs_y + 1.0, 1.0, h - 2.0).unwrap(), &paintlight, tiny_skia::Transform::identity(), None);
                    }
                }
            }
        }

        let inner_rect = Rect::from_xywh(
            abs_x + border_thickness,
            abs_y + border_thickness,
            w - (border_thickness * 2.0),
            h - (border_thickness * 2.0)
        ).unwrap();

        // Скрещиваем наш внутренний прямоугольник с глобальным клипом
        let child_clip = match intersect_rects(inner_clip, inner_rect) {
            Some(r) => r,
            None => return, // Если внутреннее пространство полностью обрезано — детей не рендерим
        };

        for child in &self.children {
            child.draw(pixmap, Pos::new(abs_x as i32, abs_y as i32), child_clip);
        }
    }
    fn get_size(&self) -> Size {
        self.base.size
    }
    fn get_pos(&self) -> Pos {
        self.base.pos
    }
    fn get_layout_strat(&self) -> LayoutStrat {
        self.base.layoutstrat.clone()
    }
    fn set_size(&mut self, size_new: Size) {
        self.base.size.width = size_new.width; self.base.size.height = size_new.height;
    }
    fn set_pos(&mut self, pos_new: Pos) {
        self.base.pos.x = pos_new.x; self.base.pos.y = pos_new.y;
    }
    fn update_layout(&mut self) {
        for child in &mut self.children {
            child.update_layout();
        }
        self.refresh_layout();
    }
    fn handle_event(&mut self, event: &Event, pos_off: Pos, actions: &mut Vec<Action>) {
        let my_global_pos = self.get_global_pos(pos_off);
        for child in self.children.iter_mut().rev() {
            child.handle_event(event, my_global_pos, actions);
        }
    }
}