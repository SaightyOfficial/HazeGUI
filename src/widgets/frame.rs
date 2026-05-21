use crate::core::common::{ChooseCords, SizeEnum, SizeStrat};
use crate::core::event::{Action, Event};
use crate::core::size::Size;
use crate::core::widget::{UsedCord, Widget, WidgetBase};
use crate::core::{color::Color, pos::Pos};
use tiny_skia::{Color as SkiaColor, Paint, PixmapMut, Rect};

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
    pub usedtotal: UsedCord,
    pub lightchangeamount: u8,
    pub vecpushedactions: Vec<Action>,
}

impl Frame {
    pub fn new(id: String) -> Self {
        Self {
            base: WidgetBase::new(id),
            style: FrameStyle::FLAT,
            children: Vec::new(),
            usedmiddle: UsedCord::default(),
            usedleft: UsedCord::default(),
            usedright: UsedCord::default(),
            usedtotal: UsedCord::default(),
            lightchangeamount: 60,
            vecpushedactions: Vec::new(),
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
            if let Some(frame) = child.as_any_mut().downcast_mut::<Frame>()
                && let Some(found) = frame.find_mut(target_id)
            {
                return Some(found);
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

    pub fn set_color(&mut self, color: Color) {
        self.base.bgcolor = color;
        self.set_dirty_flag(true);
    }

    pub fn lightchangeamount(mut self, amount: u8) -> Self {
        self.lightchangeamount = amount;
        self
    }
    pub fn set_lightchangeamount(&mut self, amount: u8) {
        self.lightchangeamount = amount;
        self.set_dirty_flag(true);
    }

    //sizes
    pub fn fill_x(mut self) -> Self {
        self.base.sizestrat.fill = ChooseCords::X;
        self.base.sizestrat.method = SizeEnum::FILL;
        self
    }

    pub fn fill_y(mut self) -> Self {
        self.base.sizestrat.fill = ChooseCords::Y;
        self.base.sizestrat.method = SizeEnum::FILL;
        self
    }

    pub fn fill_both(mut self) -> Self {
        self.base.sizestrat.fill = ChooseCords::BOTH;
        self.base.sizestrat.method = SizeEnum::FILL;
        self
    }

    pub fn fill_none(mut self) -> Self {
        self.base.sizestrat.fill = ChooseCords::NONE;
        self.base.sizestrat.method = SizeEnum::AUTO;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.base.size = size;
        self.base.sizestrat.method = SizeEnum::MANUAL;
        self
    }

    pub fn max_size(mut self, w: Option<i32>, h: Option<i32>) -> Self {
        self.base.sizestrat.max_width = w;
        self.base.sizestrat.max_height = h;
        self
    }

    pub fn set_max_size(&mut self, w: Option<Option<i32>>, h: Option<Option<i32>>) {
        if let Some(width) = w {
            self.base.sizestrat.max_width = width;
        }
        if let Some(height) = h {
            self.base.sizestrat.max_height = height;
        }
    }

    pub fn refresh_layout(&mut self) {
        self.usedleft = UsedCord::default();
        self.usedmiddle = UsedCord::default();
        self.usedright = UsedCord::default();
        self.usedtotal = UsedCord::default();

        let mut max_left_width = 0;
        let mut max_middle_width = 0;
        let mut max_right_width = 0;

        let border_thickness = if self.style == FrameStyle::FLAT { 0 } else { 2 };
        let border_padding = border_thickness * 2;

        for child in &mut self.children {
            child.update_layout(true);
            let child_size = child.get_size();
            let layoutstrat = child.get_layout_strat();
            let sizestrat = child.get_size_strat();

            let is_fill_y = sizestrat.method == SizeEnum::FILL
                && (sizestrat.fill == ChooseCords::Y || sizestrat.fill == ChooseCords::BOTH);

            if layoutstrat.method == LayoutEnum::AUTO {
                match layoutstrat.side {
                    Side::LEFT => {
                        if child_size.width > max_left_width {
                            max_left_width = child_size.width;
                        }
                        if is_fill_y {
                            self.usedleft.fill_widgets += 1;
                        } else {
                            self.usedleft.used_y += child_size.height;
                        }
                    }
                    Side::MIDDLE => {
                        if child_size.width > max_middle_width {
                            max_middle_width = child_size.width;
                        }
                        if is_fill_y {
                            self.usedmiddle.fill_widgets += 1;
                        } else {
                            self.usedmiddle.used_y += child_size.height;
                        }
                    }
                    Side::RIGHT => {
                        if child_size.width > max_right_width {
                            max_right_width = child_size.width;
                        }
                        if is_fill_y {
                            self.usedright.fill_widgets += 1;
                        } else {
                            self.usedright.used_y += child_size.height;
                        }
                    }
                }
            }
        }

        let required_width = max_left_width + max_middle_width + max_right_width + border_padding;
        let max_corridor_height = self
            .usedleft
            .used_y
            .max(self.usedmiddle.used_y)
            .max(self.usedright.used_y);
        let required_height = max_corridor_height + border_padding;

        if self.get_size_strat().method == SizeEnum::AUTO {
            self.base.size.width = required_width;
            self.base.size.height = required_height;
        }

        let parent_width = self.base.size.width - border_padding;
        let parent_height = self.base.size.height - border_padding;

        let fill_left_h = if self.usedleft.fill_widgets > 0 && parent_height > self.usedleft.used_y
        {
            (parent_height - self.usedleft.used_y) / self.usedleft.fill_widgets
        } else {
            0
        };

        let fill_middle_h =
            if self.usedmiddle.fill_widgets > 0 && parent_height > self.usedmiddle.used_y {
                (parent_height - self.usedmiddle.used_y) / self.usedmiddle.fill_widgets
            } else {
                0
            };

        let fill_right_h =
            if self.usedright.fill_widgets > 0 && parent_height > self.usedright.used_y {
                (parent_height - self.usedright.used_y) / self.usedright.fill_widgets
            } else {
                0
            };

        self.usedleft.used_y = border_thickness;
        self.usedmiddle.used_y = border_thickness;
        self.usedright.used_y = border_thickness;

        let middle_allowed_width = if parent_width > (max_left_width + max_right_width) {
            parent_width - max_left_width - max_right_width
        } else {
            0
        };

        for child in &mut self.children {
            let layoutstrat = child.get_layout_strat();
            let sizestrat = child.get_size_strat();
            let mut child_size = child.get_size();

            let is_fill_x = sizestrat.method == SizeEnum::FILL
                && (sizestrat.fill == ChooseCords::X || sizestrat.fill == ChooseCords::BOTH);
            let is_fill_y = sizestrat.method == SizeEnum::FILL
                && (sizestrat.fill == ChooseCords::Y || sizestrat.fill == ChooseCords::BOTH);

            if layoutstrat.method == LayoutEnum::AUTO {
                if is_fill_y {
                    match layoutstrat.side {
                        Side::LEFT => child_size.height = fill_left_h,
                        Side::MIDDLE => child_size.height = fill_middle_h,
                        Side::RIGHT => child_size.height = fill_right_h,
                    }
                }

                if is_fill_x {
                    match layoutstrat.side {
                        Side::LEFT => child_size.width = max_left_width,
                        Side::MIDDLE => child_size.width = middle_allowed_width,
                        Side::RIGHT => child_size.width = max_right_width,
                    }
                }

                if is_fill_x || is_fill_y {
                    let sizestrat = child.get_size_strat();

                    if let Some(max_w) = sizestrat.max_width
                        && child_size.width > max_w
                    {
                        child_size.width = max_w;
                    }

                    if let Some(max_h) = sizestrat.max_height
                        && child_size.height > max_h
                    {
                        child_size.height = max_h;
                    }

                    child.set_size(child_size);
                }

                match layoutstrat.side {
                    Side::LEFT => {
                        let x_pos = border_thickness;
                        child.set_pos(Pos::new(x_pos, self.usedleft.used_y));
                        self.usedleft.used_y += child_size.height;
                    }
                    Side::MIDDLE => {
                        let center_zone_start = border_thickness + max_left_width;
                        let x_pos =
                            center_zone_start + (middle_allowed_width - child_size.width) / 2;

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

    pub fn set_style(&mut self, style: FrameStyle) {
        self.style = style;
        if self.style != FrameStyle::FLAT && style != FrameStyle::FLAT {
            self.update_layout(false);
        }
        self.set_dirty_flag(true);
    }
    pub fn add_widget<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget));
        self.update_layout(false);
        self.set_dirty_flag(true);
    }

    pub fn remove_widget(&mut self, target_id: &str) -> bool {
        self.set_dirty_flag(true);
        let old_len = self.children.len();

        self.children.retain(|child| child.get_id() != target_id);

        if self.children.len() < old_len {
            self.update_layout(false);
            return true;
        }

        false
    }

    pub fn push_action(&mut self, action: Action) {
        self.vecpushedactions.push(action);
    }
}

impl Widget for Frame {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn get_id(&self) -> &str {
        &self.base.id
    }
    fn draw(
        &self,
        pixmap: &mut PixmapMut,
        pos_off: Pos,
        clip: Rect,
        preferred_color: Option<Color>,
    ) {
        let abs_x = (pos_off.x + self.base.pos.x) as f32;
        let abs_y = (pos_off.y + self.base.pos.y) as f32;
        let w = self.base.size.width as f32;
        let h = self.base.size.height as f32;

        let my_rect = Rect::from_xywh(abs_x, abs_y, w, h).unwrap();

        let inner_clip = match intersect_rects(clip, my_rect) {
            Some(r) => r,
            None => return,
        };

        //BGRA colors needed
        let mut paint = Paint::default();
        let mut paintdark = Paint::default();
        let mut paintlight = Paint::default();

        if let Some(preferred_color_done) = preferred_color {
            paint.set_color(SkiaColor::from_rgba8(
                preferred_color_done.b,
                preferred_color_done.g,
                preferred_color_done.r,
                preferred_color_done.a,
            ));
            let darkercolor = preferred_color_done.darker(self.lightchangeamount);
            paintdark.set_color(SkiaColor::from_rgba8(
                darkercolor.b,
                darkercolor.g,
                darkercolor.r,
                darkercolor.a,
            ));
            let lightercolor = preferred_color_done.lighter(self.lightchangeamount);
            paintlight.set_color(SkiaColor::from_rgba8(
                lightercolor.b,
                lightercolor.g,
                lightercolor.r,
                lightercolor.a,
            ));
        } else {
            paint.set_color(SkiaColor::from_rgba8(
                self.base.bgcolor.b,
                self.base.bgcolor.g,
                self.base.bgcolor.r,
                self.base.bgcolor.a,
            ));
            let darkercolor = self.base.bgcolor.darker(self.lightchangeamount);
            paintdark.set_color(SkiaColor::from_rgba8(
                darkercolor.b,
                darkercolor.g,
                darkercolor.r,
                darkercolor.a,
            ));
            let lightercolor = self.base.bgcolor.lighter(self.lightchangeamount);
            paintlight.set_color(SkiaColor::from_rgba8(
                lightercolor.b,
                lightercolor.g,
                lightercolor.r,
                lightercolor.a,
            ));
        }

        pixmap.fill_rect(inner_clip, &paint, tiny_skia::Transform::identity(), None);

        let border_thickness = if self.style == FrameStyle::FLAT {
            0.0
        } else {
            2.0
        };

        if border_thickness != 0.0 {
            //Safe line rendering that is not going outide of clip
            let draw_line = |pixmap: &mut PixmapMut,
                             x: f32,
                             y: f32,
                             width: f32,
                             height: f32,
                             paint_style: &Paint| {
                if let Some(line_rect) = Rect::from_xywh(x, y, width, height)
                    && let Some(visible_line) = intersect_rects(line_rect, clip)
                {
                    pixmap.fill_rect(
                        visible_line,
                        paint_style,
                        tiny_skia::Transform::identity(),
                        None,
                    );
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
                    draw_line(
                        pixmap,
                        abs_x + 1.0,
                        abs_y + h - 2.0,
                        w - 2.0,
                        1.0,
                        &paintdark,
                    );
                    draw_line(
                        pixmap,
                        abs_x + w - 2.0,
                        abs_y + 1.0,
                        1.0,
                        h - 2.0,
                        &paintdark,
                    );
                }
                FrameStyle::SUNKEN => {
                    draw_line(pixmap, abs_x, abs_y, w, 1.0, &paintdark);
                    draw_line(pixmap, abs_x, abs_y, 1.0, h, &paintdark);
                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, w - 2.0, 1.0, &paintdark);
                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, 1.0, h - 2.0, &paintdark);

                    draw_line(pixmap, abs_x, abs_y + h - 1.0, w, 1.0, &paintlight);
                    draw_line(pixmap, abs_x + w - 1.0, abs_y, 1.0, h, &paintlight);
                    draw_line(
                        pixmap,
                        abs_x + 1.0,
                        abs_y + h - 2.0,
                        w - 2.0,
                        1.0,
                        &paintlight,
                    );
                    draw_line(
                        pixmap,
                        abs_x + w - 2.0,
                        abs_y + 1.0,
                        1.0,
                        h - 2.0,
                        &paintlight,
                    );
                }
                FrameStyle::GROOVE => {
                    draw_line(pixmap, abs_x, abs_y, w, 1.0, &paintdark);
                    draw_line(pixmap, abs_x, abs_y, 1.0, h, &paintdark);
                    draw_line(pixmap, abs_x, abs_y + h - 1.0, w, 1.0, &paintlight);
                    draw_line(pixmap, abs_x + w - 1.0, abs_y, 1.0, h, &paintlight);

                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, w - 2.0, 1.0, &paintlight);
                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, 1.0, h - 2.0, &paintlight);
                    draw_line(
                        pixmap,
                        abs_x + 1.0,
                        abs_y + h - 2.0,
                        w - 2.0,
                        1.0,
                        &paintdark,
                    );
                    draw_line(
                        pixmap,
                        abs_x + w - 2.0,
                        abs_y + 1.0,
                        1.0,
                        h - 2.0,
                        &paintdark,
                    );
                }
                FrameStyle::RIDGE => {
                    draw_line(pixmap, abs_x, abs_y, w, 1.0, &paintlight);
                    draw_line(pixmap, abs_x, abs_y, 1.0, h, &paintlight);
                    draw_line(pixmap, abs_x, abs_y + h - 1.0, w, 1.0, &paintdark);
                    draw_line(pixmap, abs_x + w - 1.0, abs_y, 1.0, h, &paintdark);

                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, w - 2.0, 1.0, &paintdark);
                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, 1.0, h - 2.0, &paintdark);
                    draw_line(
                        pixmap,
                        abs_x + 1.0,
                        abs_y + h - 2.0,
                        w - 2.0,
                        1.0,
                        &paintlight,
                    );
                    draw_line(
                        pixmap,
                        abs_x + w - 2.0,
                        abs_y + 1.0,
                        1.0,
                        h - 2.0,
                        &paintlight,
                    );
                }
            }
        }

        let inner_rect = Rect::from_xywh(
            abs_x + border_thickness,
            abs_y + border_thickness,
            w - (border_thickness * 2.0),
            h - (border_thickness * 2.0),
        )
        .unwrap();

        let child_clip = match intersect_rects(inner_clip, inner_rect) {
            Some(r) => r,
            None => return,
        };

        for child in &self.children {
            child.draw(
                pixmap,
                Pos::new(abs_x as i32, abs_y as i32),
                child_clip,
                None,
            );
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
        if self.base.size.width != size_new.width || self.base.size.height != size_new.height {
            self.base.size.width = size_new.width;
            self.base.size.height = size_new.height;
            //self.refresh_layout();
        }
    }
    fn set_pos(&mut self, pos_new: Pos) {
        self.base.pos.x = pos_new.x;
        self.base.pos.y = pos_new.y;
    }
    fn is_dirty(&self) -> bool {
        self.base.is_dirty || self.children.iter().any(|c| c.is_dirty())
    }
    fn set_dirty_flag(&mut self, flag: bool) {
        self.base.is_dirty = flag;
        if !flag {
            for widget in &mut self.children {
                widget.set_dirty_flag(flag);
            }
        }
    }
    fn update_layout(&mut self, forced: bool) {
        for child in &mut self.children {
            child.update_layout(false);
        }
        if !forced && !self.needs_relayout() {
            return;
        }
        self.refresh_layout();
        self.set_relayout_flag(false);
    }
    fn needs_relayout(&self) -> bool {
        let mut needed = false;
        if self.base.needs_relayout {
            return true;
        }
        for widget in &self.children {
            needed = needed || widget.needs_relayout();
        }
        needed
    }
    fn set_relayout_flag(&mut self, flag: bool) {
        self.base.needs_relayout = flag;
        if !flag {
            for widget in &mut self.children {
                widget.set_relayout_flag(flag);
            }
        }
        //println!("Set flag to {} by \"{}\"", flag, self.get_id());
    }
    fn handle_event(&mut self, event: &Event, pos_off: Pos, actions: &mut Vec<Action>) {
        let my_global_pos = self.get_global_pos(pos_off);
        for action in &self.vecpushedactions {
            actions.push(action.clone());
        }
        self.vecpushedactions.clear();
        for child in self.children.iter_mut().rev() {
            child.handle_event(event, my_global_pos, actions);
        }
    }
    fn get_dirty_rect(&mut self, pos_off: Pos, actions: &mut Vec<Action>) {
        if self.base.is_dirty {
            if let Some(dirty_rect) = self.get_self_rect(pos_off) {
                actions.push(Action::RedrawRequest(Some(dirty_rect)));
            }
            self.base.is_dirty = false;
        }

        let my_global_pos = self.get_global_pos(pos_off);
        for child in &mut self.children {
            child.get_dirty_rect(my_global_pos, actions);
        }
    }
}
