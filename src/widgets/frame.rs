use crate::core::common::{Axis, SizeEnum, SizeStrat};
use crate::core::event::{Action, Event};
use crate::core::idpool::regid;
use crate::core::size::Size;
use crate::core::widget::{UsedCord, Widget, WidgetBase};
use crate::core::{color::Color, pos::Pos};
use tiny_skia::{Color as SkiaColor, Paint, PixmapMut, Rect};

use crate::core::common::{LayoutEnum, LayoutStrat, Side, intersect_rects};

///Frame style enum, there will be more later
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FrameStyle {
    FLAT,
    RAISED,
    SUNKEN,
    GROOVE,
    RIDGE,
}

///Frame struct, stores frame's child widgets and other needed things
pub struct Frame {
    pub base: WidgetBase,
    pub style: FrameStyle,
    pub padding: i32,
    pub children: Vec<Box<dyn Widget>>,
    pub usedleft: UsedCord,
    pub usedmiddle: UsedCord,
    pub usedright: UsedCord,
    pub lightchangeamount: u8,
    pub vecpushedactions: Vec<Action>,
}

impl Frame {
    pub fn new(id: String) -> Self {
        Self {
            base: WidgetBase::new(regid(id)),
            style: FrameStyle::FLAT,
            padding: 0,
            children: Vec::new(),
            usedmiddle: UsedCord::default(),
            usedleft: UsedCord::default(),
            usedright: UsedCord::default(),
            lightchangeamount: 60,
            vecpushedactions: Vec::new(),
        }
    }

    ///Recursively finds widget by its id and returns mutable reference to it, if you need read-only reference then use [`Frame::find`]
    pub fn find_mut(&mut self, target_id: u64) -> Option<&mut dyn Widget> {
        //If self just return self
        if self.base.id == target_id {
            return Some(self);
        }
        //If not self then try to find in children vector
        for child in &mut self.children {
            //if found in self then return mut reference
            if child.get_id() == target_id {
                return Some(&mut **child);
            }
            //If not in self then find recursively
            if let Some(frame) = child.as_any_mut().downcast_mut::<Frame>()
                && let Some(found) = frame.find_mut(target_id)
            {
                return Some(found); //If found return some
            }
        }
        None //If not found return none
    }

    ///Recursively finds widget by its id and returns read-only reference to it, if you need mutable reference then use [`Frame::find_mut`]
    pub fn find(&self, target_id: u64) -> Option<&dyn Widget> {
        //If self just return self
        if self.base.id == target_id {
            return Some(self);
        }
        //If not self then try to find in children vector
        for child in &self.children {
            //if found in self then return read-only reference
            if child.get_id() == target_id {
                return Some(&**child);
            }
            if let Some(frame) = child.as_any().downcast_ref::<Frame>()
                && let Some(found) = frame.find(target_id)
            {
                return Some(found); //If found return some
            }
        }
        None //If not found return none
    }

    //positions
    ///Manualy set widget position, widget will not participate in auto layout composing and just be where you said it to be
    pub fn pos(mut self, posnew: Pos) -> Self {
        self.base.pos = posnew;
        self.base.layoutstrat.method = LayoutEnum::MANUAL;
        self
    }

    ///Setting widget side, there are only [`Side::LEFT`], [`Side::MIDDLE`] and [`Side::RIGHT`], Y axis position depends on order by which widgets are added in your code
    pub fn side(&mut self, side: Side) {
        self.base.layoutstrat.side = side;
        self.set_relayout_flag(true);
    }

    ///Changes widget side at runtime, there are only [`Side::LEFT`], [`Side::MIDDLE`] and [`Side::RIGHT`], Y axis position depends on order by which widgets are added in your code
    pub fn set_side(mut self, side: Side) -> Self {
        self.base.layoutstrat.side = side;
        self
    }

    ///Sets inner widget padding
    pub fn padding(mut self, pad: i32) -> Self {
        self.padding = pad;
        self.set_relayout_flag(true);
        self
    }

    ///Changes inner widget padding at runtime
    pub fn set_padding(&mut self, pad: i32) {
        self.padding = pad;
        self.set_relayout_flag(true);
    }

    //colors and styles
    ///Sets backgound color
    pub fn color(mut self, color: Color) -> Self {
        self.base.bgcolor = color;
        self
    }

    ///Sets frame style, they can be found in [`FrameStyle`]
    pub fn style(mut self, style: FrameStyle) -> Self {
        self.style = style;
        self
    }

    ///Changes frame style at runtime, styles can be found in [`FrameStyle`]
    pub fn set_style(&mut self, style: FrameStyle) {
        self.style = style;
        if self.style != FrameStyle::FLAT && style != FrameStyle::FLAT {
            self.update_layout(false);
        }
        self.set_dirty_flag(true);
    }

    ///Changes backgound color at runtime
    pub fn set_color(&mut self, color: Color) {
        self.base.bgcolor = color;
        self.set_dirty_flag(true);
    }

    ///Sets framestyle light/dark difference from base color
    pub fn lightchangeamount(mut self, amount: u8) -> Self {
        self.lightchangeamount = amount;
        self
    }

    ///Changes framestyle light/dark difference from base color at runtime
    pub fn set_lightchangeamount(&mut self, amount: u8) {
        self.lightchangeamount = amount;
        self.set_dirty_flag(true);
    }

    //sizes
    ///By which axises widget will stretch and fill itself
    pub fn fill(mut self, cords: Axis) -> Self {
        self.base.sizestrat.fill = cords;
        if cords == Axis::NONE {
            self.base.sizestrat.method = SizeEnum::AUTO;
        } else {
            self.base.sizestrat.method = SizeEnum::FILL;
        }
        self
    }

    ///Changes at runtime by which axises widget will stretch and fill itself
    pub fn set_fill(&mut self, cords: Axis){
        self.base.sizestrat.fill = cords;
        if cords == Axis::NONE {
            self.base.sizestrat.method = SizeEnum::AUTO;
        } else {
            self.base.sizestrat.method = SizeEnum::FILL;
        }
    }

    ///Sets widget size, widget will not dynamicaly change size
    ///NOTE: Be careful while using it because if widget is too big it will be not fully visible
    pub fn size(mut self, size: Size) -> Self {
        self.base.size = size;
        self.base.sizestrat.method = SizeEnum::MANUAL;
        self
    }

    ///Max size widget can be stretched to
    pub fn max_size(mut self, w: Option<i32>, h: Option<i32>) -> Self {
        self.base.sizestrat.max_width = w;
        self.base.sizestrat.max_height = h;
        self
    }

    ///Changes max size widget can be stretched to at runtime
    pub fn set_max_size(&mut self, w: Option<Option<i32>>, h: Option<Option<i32>>) {
        if let Some(width) = w {
            self.base.sizestrat.max_width = width;
        }
        if let Some(height) = h {
            self.base.sizestrat.max_height = height;
        }
    }

    //That function was hell to write... Idk how it works but it does YAAAAAAAAAAAY
    ///Refreshes children layout recursively
    pub fn refresh_layout(&mut self) {
        //Clearing used corrdinates
        self.usedleft = UsedCord::default();
        self.usedmiddle = UsedCord::default();
        self.usedright = UsedCord::default();

        //Setting maximum side width's
        let mut max_left_width = 0;
        let mut max_middle_width = 0;
        let mut max_right_width = 0;

        //Getting border thicknes and padding
        let border_thickness = if self.style == FrameStyle::FLAT { 0 + self.padding } else { 2 + self.padding };
        let border_padding = border_thickness * 2;

        //First loop: getting info we need
        for child in &mut self.children {
            //Forcingt update layout and then getting all info we need
            child.update_layout(true);
            let child_size = child.get_size();
            let layoutstrat = child.get_layout_strat();
            let sizestrat = child.get_size_strat();

            //Will be filled by Y axis?
            let is_fill_y = sizestrat.method == SizeEnum::FILL
                && (sizestrat.fill == Axis::Y || sizestrat.fill == Axis::BOTH);

            if layoutstrat.method == LayoutEnum::AUTO {
                //Checking for side and adding used cords
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

        //Width that we need to properly show everthing
        let required_width = max_left_width + max_middle_width + max_right_width + border_padding;
        //Max corridor height
        let max_corridor_height = self
            .usedleft
            .used_y
            .max(self.usedmiddle.used_y)
            .max(self.usedright.used_y);
        //Height that we need to properly show everthing
        let required_height = max_corridor_height + border_padding;

        //If self size strategy is auto, shrink itself to needed size 
        if self.get_size_strat().method == SizeEnum::AUTO {
            self.base.size.width = required_width;
            self.base.size.height = required_height;
        }

        //Getting self inner sizes where widgets will be placed
        let parent_width = self.base.size.width - border_padding;
        let parent_height = self.base.size.height - border_padding;

        //Getting left fill height
        let fill_left_h = if self.usedleft.fill_widgets > 0 && parent_height > self.usedleft.used_y
        {
            (parent_height - self.usedleft.used_y) / self.usedleft.fill_widgets
        } else {
            0
        };
        //Getting middle fill height
        let fill_middle_h =
            if self.usedmiddle.fill_widgets > 0 && parent_height > self.usedmiddle.used_y {
                (parent_height - self.usedmiddle.used_y) / self.usedmiddle.fill_widgets
            } else {
                0
            };
        //Getting right fill height
        let fill_right_h =
            if self.usedright.fill_widgets > 0 && parent_height > self.usedright.used_y {
                (parent_height - self.usedright.used_y) / self.usedright.fill_widgets
            } else {
                0
            };

        //Presetting used y cords to padding
        self.usedleft.used_y = border_thickness;
        self.usedmiddle.used_y = border_thickness;
        self.usedright.used_y = border_thickness;

        //Allowed middle width
        let middle_allowed_width = if parent_width > (max_left_width + max_right_width) {
            parent_width - max_left_width - max_right_width
        } else {
            0
        };

        //Second loop: placing
        for child in &mut self.children {
            //Getting info we need
            let layoutstrat = child.get_layout_strat();
            let sizestrat = child.get_size_strat();
            let mut child_size = child.get_size();

            //Getting fills
            let is_fill_x = sizestrat.method == SizeEnum::FILL
                && (sizestrat.fill == Axis::X || sizestrat.fill == Axis::BOTH);
            let is_fill_y = sizestrat.method == SizeEnum::FILL
                && (sizestrat.fill == Axis::Y || sizestrat.fill == Axis::BOTH);

            if layoutstrat.method == LayoutEnum::AUTO {
                //Filling by Y axis
                if is_fill_y {
                    match layoutstrat.side {
                        Side::LEFT => child_size.height = fill_left_h,
                        Side::MIDDLE => child_size.height = fill_middle_h,
                        Side::RIGHT => child_size.height = fill_right_h,
                    }
                }

                //Filling by X axis
                if is_fill_x {
                    match layoutstrat.side {
                        Side::LEFT => child_size.width = max_left_width,
                        Side::MIDDLE => child_size.width = middle_allowed_width,
                        Side::RIGHT => child_size.width = max_right_width,
                    }
                }

                //If needs any axis fill then set proper size
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

                //Final position setting by sides
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
    //End of hell

    ///Adds presetted widget inside self
    pub fn add_widget<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget));
        self.update_layout(false);
        self.set_dirty_flag(true);
    }

    ///Finds and removes widget by its id, NOT RECURSIVE
    pub fn remove_widget(&mut self, target_id: u64) -> bool {
        self.set_dirty_flag(true);
        let old_len = self.children.len();

        self.children.retain(|child| child.get_id() != target_id);

        if self.children.len() < old_len {
            self.update_layout(false);
            return true;
        }

        false
    }

    ///Manual action push, if you want to manualy send redraw/relayout requests, or custom action with your own signal
    ///All actions can be found in [`Action`]'s list
    pub fn push_action(&mut self, action: Action) {
        self.vecpushedactions.push(action);
    }
}

impl Widget for Frame {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn get_id(&self) -> u64 {
        self.base.id
    }
    //Another hellish function...
    fn draw(
        &self,
        pixmap: &mut PixmapMut,
        pos_off: Pos,
        clip: Rect,
        preferred_color: Option<Color>,
    ) {
        //Getting absolute positions and size
        let abs_x = (pos_off.x + self.base.pos.x) as f32;
        let abs_y = (pos_off.y + self.base.pos.y) as f32;
        let w = self.base.size.width as f32;
        let h = self.base.size.height as f32;

        let my_rect = match Rect::from_xywh(abs_x, abs_y, w, h) {
            Some(r) => r,
            None => return,
        };

        //Getting full frame cliprect
        let inner_clip = match intersect_rects(clip, my_rect) {
            Some(r) => r,
            None => return,
        };

        //Creating paints
        let mut paint = Paint::default();
        let mut paintdark = Paint::default();
        let mut paintlight = Paint::default();

        //Setting colors, BGRA is needed and is not a bug
        if let Some(preferred_color_done) = preferred_color {
            paint.set_color(SkiaColor::from_rgba8(preferred_color_done.b, preferred_color_done.g, preferred_color_done.r, preferred_color_done.a));

            let darkercolor = preferred_color_done.darker(self.lightchangeamount);
            paintdark.set_color(SkiaColor::from_rgba8(darkercolor.b, darkercolor.g, darkercolor.r, darkercolor.a, ));

            let lightercolor = preferred_color_done.lighter(self.lightchangeamount);
            paintlight.set_color(SkiaColor::from_rgba8(lightercolor.b, lightercolor.g, lightercolor.r, lightercolor.a));
        } else {
            paint.set_color(SkiaColor::from_rgba8(self.base.bgcolor.b, self.base.bgcolor.g, self.base.bgcolor.r, self.base.bgcolor.a));

            let darkercolor = self.base.bgcolor.darker(self.lightchangeamount);
            paintdark.set_color(SkiaColor::from_rgba8(
                darkercolor.b,
                darkercolor.g,
                darkercolor.r,
                darkercolor.a));

            let lightercolor = self.base.bgcolor.lighter(self.lightchangeamount);
            paintlight.set_color(SkiaColor::from_rgba8(lightercolor.b, lightercolor.g, lightercolor.r, lightercolor.a));
        }

        pixmap.fill_rect(inner_clip, &paint, tiny_skia::Transform::identity(), None);

        let border_thickness = if self.style == FrameStyle::FLAT { 0.0 + self.padding as f32 } else { 2.0  + self.padding as f32 };

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

            //The great framestyles render, idk what that is but it is working so dont touch that
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
                    draw_line(pixmap, abs_x + w - 2.0, abs_y + 1.0, 1.0, h - 2.0, &paintdark,);
                }
                FrameStyle::RIDGE => {
                    draw_line(pixmap, abs_x, abs_y, w, 1.0, &paintlight);
                    draw_line(pixmap, abs_x, abs_y, 1.0, h, &paintlight);
                    draw_line(pixmap, abs_x, abs_y + h - 1.0, w, 1.0, &paintdark);
                    draw_line(pixmap, abs_x + w - 1.0, abs_y, 1.0, h, &paintdark);

                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, w - 2.0, 1.0, &paintdark);
                    draw_line(pixmap, abs_x + 1.0, abs_y + 1.0, 1.0, h - 2.0, &paintdark);
                    draw_line(pixmap,abs_x + 1.0,abs_y + h - 2.0,w - 2.0,1.0, &paintlight);
                    draw_line(pixmap, abs_x + w - 2.0, abs_y + 1.0, 1.0, h - 2.0, &paintlight);
                }
            }
        }
        //Framestyles hell end

        //Getting inner rect with padding and etc
        let inner_rect = match Rect::from_xywh(
            abs_x + border_thickness,
            abs_y + border_thickness,
            w - (border_thickness * 2.0),
            h - (border_thickness * 2.0),
        ) {
            Some(r) => r,
            None => return,
        };

        //Child cliprect
        let child_clip = match intersect_rects(inner_clip, inner_rect) {
            Some(r) => r,
            None => return,
        };

        //Recursive draw of other widgets
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
        if !flag { //If flag is false then set dirty flag to false recursively
            for widget in &mut self.children {
                widget.set_dirty_flag(flag);
            }
        }
    }
    fn update_layout(&mut self, forced: bool) {
        if !forced && !self.needs_relayout() {
            return;
        }
        //Refreshing layout and clearing relayout flag
        self.refresh_layout();
        self.set_relayout_flag(false);
    }
    fn needs_relayout(&self) -> bool {
        //Recursive check for relayout flag
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
        if !flag {  //If flag is false then set dirty flag to false recursively
            for widget in &mut self.children {
                widget.set_relayout_flag(flag);
            }
        }
        //println!("Set flag to {} by \"{}\"", flag, self.get_id());
    }
    fn handle_event(&mut self, event: &Event, pos_off: Pos, actions: &mut Vec<Action>) {
        //Getting global pos and pushing all requested actions
        let my_global_pos = self.get_global_pos(pos_off);
        for action in &self.vecpushedactions {
            actions.push(action.clone());
        }
        self.vecpushedactions.clear();//Clearing them
        //Recursive event handling
        for child in self.children.iter_mut().rev() {
            child.handle_event(event, my_global_pos, actions);
        }
    }
    fn get_dirty_rect(&mut self, pos_off: Pos, requests: &mut Vec<Action>) {
        //If self is dirty, then push redraw request with self rect inside
        if self.base.is_dirty {
            if let Some(dirty_rect) = self.get_self_rect(pos_off) {
                requests.push(Action::RedrawRequest(Some(dirty_rect)));
            }
            self.base.is_dirty = false;//Clearing flag for self
            return;//Exiting because all child widgets will also redraw
        }

        //Getting global pos and recursive handling of getting dirty rects
        let my_global_pos = self.get_global_pos(pos_off);
        for child in &mut self.children {
            child.get_dirty_rect(my_global_pos, requests);
        }
    }
}
