use crate::core::size::Size;
use crate::core::event::{Action, Event};
use crate::core::{color::Color, pos::Pos};
use crate::core::common::{ChooseCords, SizeEnum, SizeStrat};
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
    pub lightchangeamount: u8,
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
            lightchangeamount: 60,
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

    pub fn set_lightchangeamount(mut self, amount:u8) -> Self {
        self.lightchangeamount = amount;
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
        self.base.sizestrat.method = SizeEnum::MANUAL;
        self
    }

    pub fn refresh_layout(&mut self) {
        self.usedleft.used_y = 0;
        self.usedmiddle.used_y = 0;
        self.usedright.used_y = 0;
        
        // Трекеры максимальной ширины для каждой стороны
        let mut max_left_width = 0;
        let mut max_middle_width = 0;
        let mut max_right_width = 0;

        let border_thickness = if self.style == FrameStyle::FLAT { 0 } else { 2 };
        let border_padding = border_thickness * 2;

        // --- ПРОХОД 1: Собираем статистику по всем трёх коридорам ---
        for child in &mut self.children {
            child.update_layout(true); 
            let child_size = child.get_size();
            let layoutstrat = child.get_layout_strat();
            
            if layoutstrat.method == LayoutEnum::AUTO {
                match layoutstrat.side {
                    Side::LEFT => {
                        if child_size.width > max_left_width { max_left_width = child_size.width; }
                        self.usedleft.used_y += child_size.height;
                    }
                    Side::MIDDLE => {
                        if child_size.width > max_middle_width { max_middle_width = child_size.width; }
                        self.usedmiddle.used_y += child_size.height;
                    }
                    Side::RIGHT => {
                        if child_size.width > max_right_width { max_right_width = child_size.width; }
                        self.usedright.used_y += child_size.height;
                    }
                }
            } else {
                // На случай MANUAL виджетов, чтобы они тоже не ломали общую логику
                if child_size.width > max_middle_width { max_middle_width = child_size.width; }
            }
        }

        // Финальная требуемая ширина — это сумма максимумов всех трёх сторон!
        let required_width = max_left_width + max_middle_width + max_right_width + border_padding;

        // Высота — по самому длинному коридору
        let max_corridor_height = self.usedleft.used_y
            .max(self.usedmiddle.used_y)
            .max(self.usedright.used_y);
        let required_height = max_corridor_height + border_padding;

        // Если у фрейма стоит AUTO-размер, выставляем честно посчитанные значения
        if self.get_size_strat().method == SizeEnum::AUTO {
            self.base.size.width = required_width;
            self.base.size.height = required_height;
        }

        // --- ПРОХОД 2: Расставляем виджеты по местам (код остаётся старым) ---
        let parent_width = self.base.size.width - border_padding;

        self.usedleft.used_y = border_thickness;
        self.usedmiddle.used_y = border_thickness;
        self.usedright.used_y = border_thickness;

        for child in &mut self.children {
            let layoutstrat = child.get_layout_strat();
            let child_size = child.get_size();

            if layoutstrat.method == LayoutEnum::AUTO {
                match layoutstrat.side {
                    Side::LEFT => {
                        let x_pos = border_thickness;
                        child.set_pos(Pos::new(x_pos, self.usedleft.used_y));
                        self.usedleft.used_y += child_size.height;
                    }
                    Side::MIDDLE => {
                        let x_pos = border_thickness + (parent_width - child_size.width) / 2;
                        child.set_pos(Pos::new(x_pos, self.usedmiddle.used_y));
                        self.usedmiddle.used_y += child_size.height;
                    }
                    Side::RIGHT => {
                        let x_pos = border_thickness + parent_width - child_size.width;
                        child.set_pos(Pos::new(x_pos, self.usedright.used_y));
                        self.usedright.used_y += child_size.height;
                    }
                }
            }
        }
    }

    pub fn add_widget<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget));
        self.update_layout(false);
    }

    pub fn remove_widget(&mut self, target_id: &str) -> bool {
        let old_len = self.children.len();

        self.children.retain(|child| child.get_id() != target_id);

        if self.children.len() < old_len {
            self.update_layout(false);
            return true;
        }

        false
    }
}

impl Widget for Frame {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn get_id(&self) -> &str { &self.base.id }
    fn draw(&self, pixmap: &mut PixmapMut, pos_off: Pos, clip: Rect, preferred_color: Option<Color>) {
        let abs_x = (pos_off.x + self.base.pos.x) as f32;
        let abs_y = (pos_off.y + self.base.pos.y) as f32;
        let w = self.base.size.width as f32;
        let h = self.base.size.height as f32;

        let my_rect = Rect::from_xywh(abs_x, abs_y, w, h).unwrap();

        let inner_clip = match intersect_rects(clip, my_rect) {
            Some(r) => r,
            None => return,
        };

        // Настраиваем цвета (BGRA)
        let mut paint = Paint::default();
        let mut paintdark = Paint::default();
        let mut paintlight = Paint::default();
        
        if let Some(preferred_color_done) = preferred_color {
            paint.set_color(SkiaColor::from_rgba8(preferred_color_done.b, preferred_color_done.g, preferred_color_done.r, preferred_color_done.a));
            let darkercolor = preferred_color_done.darker(self.lightchangeamount);
            paintdark.set_color(SkiaColor::from_rgba8(darkercolor.b, darkercolor.g, darkercolor.r, darkercolor.a));
            let lightercolor = preferred_color_done.lighter(self.lightchangeamount);
            paintlight.set_color(SkiaColor::from_rgba8(lightercolor.b, lightercolor.g, lightercolor.r, lightercolor.a));
        } else {
            paint.set_color(SkiaColor::from_rgba8(self.base.bgcolor.b, self.base.bgcolor.g, self.base.bgcolor.r, self.base.bgcolor.a));
            let darkercolor = self.base.bgcolor.darker(self.lightchangeamount);
            paintdark.set_color(SkiaColor::from_rgba8(darkercolor.b, darkercolor.g, darkercolor.r, darkercolor.a));
            let lightercolor = self.base.bgcolor.lighter(self.lightchangeamount);
            paintlight.set_color(SkiaColor::from_rgba8(lightercolor.b, lightercolor.g, lightercolor.r, lightercolor.a));
        }

        // Заливаем основной фон фрейма (он уже безопасно обрезан)
        pixmap.fill_rect(inner_clip, &paint, tiny_skia::Transform::identity(), None);

        let border_thickness = if self.style == FrameStyle::FLAT { 0.0 } else { 2.0 };
        
        if border_thickness != 0.0 {
            // ЛОКАЛЬНАЯ ХЕЛПЕР-ФУНКЦИЯ: безопасно рисует линию, не вылетая за глобальный clip
            let draw_line = |pixmap: &mut PixmapMut, x: f32, y: f32, width: f32, height: f32, paint_style: &Paint| {
                if let Some(line_rect) = Rect::from_xywh(x, y, width, height) {
                    if let Some(visible_line) = intersect_rects(line_rect, clip) {
                        pixmap.fill_rect(visible_line, paint_style, tiny_skia::Transform::identity(), None);
                    }
                }
            };

            match self.style {
                FrameStyle::FLAT => {}
                FrameStyle::RAISED => {
                    draw_line(pixmap, abs_x, abs_y, w, 1.0, &paintlight);
                    draw_line(pixmap, abs_x, abs_y, 1.0, h, &paintlight);
                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, w - 2.0, 1.0, &paintlight);
                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, 1.0, h - 2.0, &paintlight);

                    draw_line(pixmap, abs_x, abs_y + h - 1.0, w, 1.0, &paintdark);
                    draw_line(pixmap, abs_x + w - 1.0, abs_y, 1.0, h, &paintdark);
                    draw_line(pixmap, abs_x + 1.0, abs_y + h - 2.0, w - 2.0, 1.0, &paintdark);
                    draw_line(pixmap, abs_x + w - 2.0, abs_y + 1.0, 1.0, h - 2.0, &paintdark);
                }
                FrameStyle::SUNKEN => {
                    draw_line(pixmap, abs_x, abs_y, w, 1.0, &paintdark);
                    draw_line(pixmap, abs_x, abs_y, 1.0, h, &paintdark);
                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, w - 2.0, 1.0, &paintdark);
                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, 1.0, h - 2.0, &paintdark);

                    draw_line(pixmap, abs_x, abs_y + h - 1.0, w, 1.0, &paintlight);
                    draw_line(pixmap, abs_x + w - 1.0, abs_y, 1.0, h, &paintlight);
                    draw_line(pixmap, abs_x + 1.0, abs_y + h - 2.0, w - 2.0, 1.0, &paintlight);
                    draw_line(pixmap, abs_x + w - 2.0, abs_y + 1.0, 1.0, h - 2.0, &paintlight);
                }
                FrameStyle::GROOVE => {
                    draw_line(pixmap, abs_x, abs_y, w, 1.0, &paintdark);
                    draw_line(pixmap, abs_x, abs_y, 1.0, h, &paintdark);
                    draw_line(pixmap, abs_x, abs_y + h - 1.0, w, 1.0, &paintlight);
                    draw_line(pixmap, abs_x + w - 1.0, abs_y, 1.0, h, &paintlight);

                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, w - 2.0, 1.0, &paintlight);
                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, 1.0, h - 2.0, &paintlight);
                    draw_line(pixmap, abs_x + 1.0, abs_y + h - 2.0, w - 2.0, 1.0, &paintdark);
                    draw_line(pixmap, abs_x + w - 2.0, abs_y + 1.0, 1.0, h - 2.0, &paintdark);
                }
                FrameStyle::RIDGE => {
                    draw_line(pixmap, abs_x, abs_y, w, 1.0, &paintlight);
                    draw_line(pixmap, abs_x, abs_y, 1.0, h, &paintlight);
                    draw_line(pixmap, abs_x, abs_y + h - 1.0, w, 1.0, &paintdark);
                    draw_line(pixmap, abs_x + w - 1.0, abs_y, 1.0, h, &paintdark);

                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, w - 2.0, 1.0, &paintdark);
                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, 1.0, h - 2.0, &paintdark);
                    draw_line(pixmap, abs_x + 1.0, abs_y + h - 2.0, w - 2.0, 1.0, &paintlight);
                    draw_line(pixmap, abs_x + w - 2.0, abs_y + 1.0, 1.0, h - 2.0, &paintlight);
                }
            }
        }

        let inner_rect = Rect::from_xywh(
            abs_x + border_thickness,
            abs_y + border_thickness,
            w - (border_thickness * 2.0),
            h - (border_thickness * 2.0)
        ).unwrap();

        let child_clip = match intersect_rects(inner_clip, inner_rect) {
            Some(r) => r,
            None => return,
        };

        for child in &self.children {
            child.draw(pixmap, Pos::new(abs_x as i32, abs_y as i32), child_clip, None);
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
    fn get_size_strat(&self) -> SizeStrat {
        self.base.sizestrat.clone()
    }
    fn set_size(&mut self, size_new: Size) {
        self.base.size.width = size_new.width; self.base.size.height = size_new.height;
    }
    fn set_pos(&mut self, pos_new: Pos) {
        self.base.pos.x = pos_new.x; self.base.pos.y = pos_new.y;
    }
    fn update_layout(&mut self, forced: bool) {
        for child in &mut self.children {
            child.update_layout(false);
        }
        if !forced {
            if !self.needs_relayout() {
                return;
            }
        }
        self.refresh_layout();
        self.set_relayout_flag(false);
    }
    fn needs_relayout(&self) -> bool {
        let mut needed = false;
        if self.base.needs_relayout == true {
            return true;
        }
        for widget in &self.children {
            needed = needed || widget.needs_relayout();
        }
        needed
    }
    fn set_relayout_flag(&mut self, flag: bool) {
        self.base.needs_relayout = flag;
        if flag == false {
            for widget in &mut self.children {
                widget.set_relayout_flag(flag);
            }
        }
        //println!("Set frag to {} by \"{}\"", flag, self.get_id()); 
    }
    fn handle_event(&mut self, event: &Event, pos_off: Pos, actions: &mut Vec<Action>) {
        let my_global_pos = self.get_global_pos(pos_off);
        for child in self.children.iter_mut().rev() {
            child.handle_event(event, my_global_pos, actions);
        }
    }
}