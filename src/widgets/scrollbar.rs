use crate::core::common::{Axis, LayoutEnum, LayoutStrat, Side, SizeEnum, SizeStrat};
use crate::core::event::{Action, Event};
use crate::core::idpool::regid;
use crate::core::size::Size;
use crate::core::widget::Widget;
use crate::core::{color::Color, pos::Pos};
use crate::widgets::frame::{Frame, FrameStyle};
use tiny_skia::{PixmapMut, Rect};

///ScrollBar struct, stores everything scrollbar needs
pub struct ScrollBar {
    pub id: u64,
    pub frame: Frame,
    pub scroll_value: f32,
    pub widgetaxis: Axis,
    is_hovered: bool,
    is_dragged: bool,
    dragstartmouse: f32, // Универсальное имя вместо dragstartmousey
}

impl ScrollBar {
    pub fn new(id: String) -> Self {
        let mut framesetter = Frame::new(format!("{}.frame", id.clone()));
        framesetter.style(FrameStyle::SUNKEN);
        framesetter.min_size(Some(Some(15)), Some(Some(15)));

        let mut thumbsetter = Frame::new(format!("{}.thumb", id));
        thumbsetter.style(FrameStyle::RAISED);

        framesetter.add_widget(thumbsetter);
        Self {
            id: regid(id.clone()),
            frame: framesetter,
            scroll_value: 0.0,
            widgetaxis: Axis::Y,
            is_hovered: false,
            is_dragged: false,
            dragstartmouse: 0.0,
        }
    }

    //positions
    pub fn pos(&mut self, posnew: Pos) {
        self.frame.base.pos = posnew;
        self.frame.base.layoutstrat.method = LayoutEnum::MANUAL;
        self.set_relayout_flag(true);
    }

    pub fn side(&mut self, side: Side) {
        self.frame.base.layoutstrat.side = side;
        self.frame.base.layoutstrat.method = LayoutEnum::AUTO;
        self.set_relayout_flag(true);
    }

    //colors
    pub fn set_color(&mut self, new_color: Color) {
        self.frame.base.bgcolor = new_color;
    }

    pub fn thumbcolor(&mut self, new_color: Color) {
        if let Some(widget) = self.frame.children.get_mut(0) {
            if let Some(thumb) = widget.as_any_mut().downcast_mut::<Frame>() {
                thumb.base.bgcolor = new_color;
            }
        }
    }

    //sizes
    pub fn fill(&mut self, cords: Axis) {
        self.frame.base.sizestrat.fill = cords;
        if cords == Axis::NONE {
            self.frame.base.sizestrat.method = SizeEnum::AUTO;
        } else {
            self.frame.base.sizestrat.method = SizeEnum::FILL;
        }
    }

    pub fn axis(&mut self, axis: Axis) { 
        // Просто сохраняем ось, никакой самодеятельности с fill() дочернего фрейма!
        self.widgetaxis = match axis {
            Axis::X => Axis::X,
            _ => Axis::Y,
        };
        self.set_relayout_flag(true);
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

    pub fn max_size(&mut self, w: Option<Option<i32>>, h: Option<Option<i32>>) {
        if let Some(width) = w {
            self.frame.base.sizestrat.max_width = width;
        }
        if let Some(height) = h {
            self.frame.base.sizestrat.max_height = height;
        }
    }

    pub fn min_size(&mut self, w: Option<Option<i32>>, h: Option<Option<i32>>) {
        if let Some(width) = w {
            self.frame.base.sizestrat.min_width = width;
        }
        if let Some(height) = h {
            self.frame.base.sizestrat.min_height = height;
        }
    }
}

impl Widget for ScrollBar {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn get_id(&self) -> u64 {
        self.id
    }
    fn draw(&self, pixmap: &mut PixmapMut, pos_off: Pos, clip: Rect, _preferred_color: Option<Color>) {
        self.frame.draw(pixmap, pos_off, clip, None);
    }

    fn update_layout(&mut self, forced: bool) {
        // 1. Сначала даем базовому фрейму обновить свои размеры, если они AUTO/FILL
        self.frame.update_layout(forced);

        let bordersize = match self.frame.style {
            FrameStyle::FLAT => 0,
            _ => 2,
        };

        let size = self.frame.get_size();
        let aval_w = (size.width - bordersize * 2) as f32;
        let aval_h = (size.height - bordersize * 2) as f32;

        if let Some(widget) = self.frame.children.get_mut(0) {
            if let Some(thumb) = widget.as_any_mut().downcast_mut::<Frame>() {
                // Сбрасываем любые автоматические стратегии размеров ползунка в MANUAL, 
                // иначе лайаут родителя сожрет наши ручные вычисления
                thumb.base.sizestrat.method = SizeEnum::MANUAL;
                thumb.base.layoutstrat.method = LayoutEnum::MANUAL;

                if self.widgetaxis == Axis::X {
                    // Математика для ГОРИЗОНТАЛЬНОГО ползунка
                    let thumb_width = (aval_w * 0.20).max(10.0);
                    thumb.base.size = Size::new(thumb_width as i32, aval_h as i32);
                    
                    let thumb_x = (self.scroll_value * (aval_w - thumb_width)).round() as i32 + bordersize;
                    thumb.base.pos = Pos::new(thumb_x, bordersize);
                } else {
                    // Математика для ВЕРТИКАЛЬНОГО ползунка
                    let thumb_height = (aval_h * 0.20).max(10.0);
                    thumb.base.size = Size::new(aval_w as i32, thumb_height as i32);
                    
                    let thumb_y = (self.scroll_value * (aval_h - thumb_height)).round() as i32 + bordersize;
                    thumb.base.pos = Pos::new(bordersize, thumb_y);
                }

                // 2. И только после того, как вбили координаты руками, форсированно обновляем сам ползунок
                thumb.update_layout(true);
            }
        }
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
        if let Some(widget) = self.frame.children.get(0) {
            if let Some(thumb) = widget.as_any().downcast_ref::<Frame>() {
                return thumb.is_dirty() || self.frame.is_dirty();
            }
        }
        self.frame.is_dirty()
    }
    fn set_dirty_flag(&mut self, flag: bool) {
        self.frame.set_dirty_flag(flag);
    }
    fn needs_relayout(&self) -> bool {
        if let Some(widget) = self.frame.children.get(0) {
            if let Some(thumb) = widget.as_any().downcast_ref::<Frame>() {
                return thumb.needs_relayout() || self.frame.needs_relayout();
            }
        }
        self.frame.needs_relayout()
    }
    fn set_relayout_flag(&mut self, flag: bool) {
        self.frame.set_relayout_flag(flag);
    }

    fn handle_event(&mut self, event: &Event, pos_off: Pos, actions: &mut Vec<Action>) {
        let track_abs_pos = self.get_pos() + pos_off;
        let size = self.frame.get_size();

        let bordersize = match self.frame.style {
            FrameStyle::FLAT => 0.0,
            _ => 2.0,
        };

        // Общие ивенты для обеих осей, чтобы не дублировать код
        match event {
            Event::MouseRelease { pos: _ } => {
                if self.is_dragged {
                    self.is_dragged = false;
                }
                return;
            }
            Event::MouseMove { pos } => {
                let now_hovered = self.is_point_inside(*pos, pos_off);
                if now_hovered && !self.is_hovered {
                    self.is_hovered = true;
                } else if !now_hovered && self.is_hovered {
                    self.is_hovered = false;
                }
                // Если мышь просто двигается, но драга нет — выходим, 
                // иначе управление передается ниже в обработку драга по осям.
                if !self.is_dragged {
                    return;
                }
            }
            _ => {}
        }

        // А вот теперь считаем логику клика и драга в зависимости от оси
        match self.widgetaxis {
            Axis::Y => {
                let aval_frame_h = if self.frame.style == FrameStyle::FLAT { size.height as f32 } else { size.height as f32 - 4.0 };
                let thumb_height = (aval_frame_h * 0.20).max(10.0);
                let max_travel = aval_frame_h - thumb_height;

                let current_thumb_y_rel = (self.scroll_value * max_travel).round() as i32 + bordersize as i32;
                let thumb_abs_y = track_abs_pos.y + current_thumb_y_rel;
                let thumb_abs_x = track_abs_pos.x + bordersize as i32;

                match event {
                    Event::MouseClick { pos } => {
                        let inside_thumb_x = pos.x >= thumb_abs_x && pos.x <= thumb_abs_x + size.width;
                        let inside_thumb_y = pos.y >= thumb_abs_y && pos.y <= thumb_abs_y + thumb_height as i32;

                        if inside_thumb_x && inside_thumb_y {
                            self.is_dragged = true;
                            self.dragstartmouse = pos.y as f32;
                            self.set_dirty_flag(true);
                        } else if self.is_point_inside(*pos, pos_off) && max_travel > 0.0 {
                            let click_y_rel = (pos.y - track_abs_pos.y) as f32 - bordersize;
                            let new_thumb_y = (click_y_rel - thumb_height / 2.0).clamp(0.0, max_travel);
                            
                            self.scroll_value = new_thumb_y / max_travel;
                            actions.push(Action::ScrollChanged(self.id, self.scroll_value));
                            
                            self.is_dragged = true;
                            self.dragstartmouse = track_abs_pos.y as f32 + bordersize + new_thumb_y + (thumb_height / 2.0);
                            
                            self.update_layout(false);
                        }
                    }
                    Event::MouseMove { pos } => {
                        if max_travel > 0.0 {
                            let delta_y = pos.y as f32 - self.dragstartmouse;
                            if delta_y.abs() > 0.0 {
                                let current_thumb_y = self.scroll_value * max_travel;
                                let new_thumb_y = (current_thumb_y + delta_y).clamp(0.0, max_travel);
                                let new_scroll = new_thumb_y / max_travel;
                                
                                if (new_scroll - self.scroll_value).abs() > 0.0001 {
                                    self.scroll_value = new_scroll;
                                    self.dragstartmouse = pos.y as f32;
                                    
                                    actions.push(Action::ScrollChanged(self.id, self.scroll_value));
                                    self.update_layout(false);
                                    self.set_dirty_flag(true);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Axis::X => {
                let aval_frame_w = if self.frame.style == FrameStyle::FLAT { size.width as f32 } else { size.width as f32 - 4.0 };
                let thumb_width = (aval_frame_w * 0.20).max(10.0);
                let max_travel = aval_frame_w - thumb_width;

                let current_thumb_x_rel = (self.scroll_value * max_travel).round() as i32 + bordersize as i32;
                let thumb_abs_x = track_abs_pos.x + current_thumb_x_rel;
                let thumb_abs_y = track_abs_pos.y + bordersize as i32;

                match event {
                    Event::MouseClick { pos } => {
                        let inside_thumb_x = pos.x >= thumb_abs_x && pos.x <= thumb_abs_x + thumb_width as i32;
                        let inside_thumb_y = pos.y >= thumb_abs_y && pos.y <= thumb_abs_y + size.height;

                        if inside_thumb_x && inside_thumb_y {
                            self.is_dragged = true;
                            self.dragstartmouse = pos.x as f32;
                            self.set_dirty_flag(true);
                        } else if self.is_point_inside(*pos, pos_off) && max_travel > 0.0 {
                            let click_x_rel = (pos.x - track_abs_pos.x) as f32 - bordersize;
                            let new_thumb_x = (click_x_rel - thumb_width / 2.0).clamp(0.0, max_travel);
                            
                            self.scroll_value = new_thumb_x / max_travel;
                            actions.push(Action::ScrollChanged(self.id, self.scroll_value));
                            
                            self.is_dragged = true;
                            self.dragstartmouse = track_abs_pos.x as f32 + bordersize + new_thumb_x + (thumb_width / 2.0);
                            
                            self.update_layout(false);
                        }
                    }
                    Event::MouseMove { pos } => {
                        if max_travel > 0.0 {
                            let delta_x = pos.x as f32 - self.dragstartmouse;
                            if delta_x.abs() > 0.0 {
                                let current_thumb_x = self.scroll_value * max_travel;
                                let new_thumb_x = (current_thumb_x + delta_x).clamp(0.0, max_travel);
                                let new_scroll = new_thumb_x / max_travel;
                                
                                if (new_scroll - self.scroll_value).abs() > 0.0001 {
                                    self.scroll_value = new_scroll;
                                    self.dragstartmouse = pos.x as f32;
                                    
                                    actions.push(Action::ScrollChanged(self.id, self.scroll_value));
                                    self.update_layout(false);
                                    self.set_dirty_flag(true);
                                }
                            }
                        }
                    }
                    _ => {}
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