use crate::core::common::{Axis, LayoutEnum, LayoutStrat, Side, SizeEnum, SizeStrat, intersect_rects};
use crate::core::event::Action::ScrollChanged;
use crate::core::event::{Action, DrawCommand, Event};
use crate::core::idpool::{regid, get_id};
use crate::core::size::Size;
use crate::core::shapes::Rect;
use crate::core::widget::Widget;
use crate::core::{color::Color, pos::Pos};
use crate::hsid;
use crate::widgets::frame::{Frame, FrameStyle};
use crate::widgets::scrollbar::ScrollBar;

///Buton struct, stores everything button needs
pub struct ScrollFrame {
    pub id: u64,
    pub frame: Frame,
    is_bottom_to_top: bool,
    scrollaxis: Axis,
}

impl ScrollFrame {
    pub fn new(id: String, scrolling: Axis, is_bottom_to_top: bool,) -> Self {
        let mut framesetter = Frame::new(format!("{}.frame", id.clone()));
        framesetter.style(FrameStyle::SUNKEN);

        let mut subcontainerframesetter = Frame::new(format!("{}.subcontainer", id.clone()));
        subcontainerframesetter.color(Color::TRANSPARENT);
        subcontainerframesetter.fill(Axis::BOTH);

        let mut containerframesetter = Frame::new(format!("{}.container", id.clone()));
        containerframesetter.pos(Pos::new(0, 0)); //For manual layout method that is needed

        let mut vthumbsetter = ScrollBar::new(format!("{}.v_scbar", id.clone()), is_bottom_to_top);
        vthumbsetter.side(Side::RIGHT);
        vthumbsetter.fill(Axis::Y);
        vthumbsetter.axis(Axis::Y);

        let mut hthumbsetter = ScrollBar::new(format!("{}.h_scbar", id.clone()),is_bottom_to_top);
        hthumbsetter.fill(Axis::X);
        hthumbsetter.axis(Axis::X);
        
        subcontainerframesetter.add_widget(containerframesetter);
        framesetter.add_widget(subcontainerframesetter);
        if scrolling == Axis::Y || scrolling == Axis::BOTH {
            framesetter.add_widget(vthumbsetter);
        }
        if scrolling == Axis::X || scrolling == Axis::BOTH {
            framesetter.add_widget(hthumbsetter);
        }
        Self {
            id: regid(id.clone()),
            frame: framesetter,
            is_bottom_to_top: is_bottom_to_top,
            scrollaxis: scrolling,
        }
    }

    //positions
    ///Manualy set widget position, widget will not participate in auto layout composing and just be where you said it to be
    pub fn pos(&mut self, posnew: Pos) {
        self.frame.base.pos = posnew;
        self.frame.base.layoutstrat.method = LayoutEnum::MANUAL;
        self.set_relayout_flag(true);
    }

    ///Sets widget side, there are only [`Side::LEFT`], [`Side::MIDDLE`] and [`Side::RIGHT`], Y axis position depends on order by which widgets are added in your code
    pub fn side(&mut self, side: Side) {
        self.frame.base.layoutstrat.side = side;
        self.frame.base.layoutstrat.method = LayoutEnum::AUTO;
        self.set_relayout_flag(true);
    }

    //colors
    pub fn color(&mut self, new_color: Color) {
        if let Some(id) = get_id(self.id) {
            let containerid = format!("{}.container", id);
            if let Some(widget) = self.frame.find_mut(hsid!(&containerid)) {
                if let Some(scbar) = widget.as_any_mut().downcast_mut::<Frame>() {
                    scbar.color(new_color);
                }
            }
            self.frame.base.bgcolor = new_color;
        }
    }

    pub fn scrollbar_color(&mut self, new_color: Color) {
        if let Some(id) = get_id(self.id) {
            let hscbarid = format!("{}.h_scbar", id.clone());
            let vscbarid = format!("{}.v_scbar", id.clone());

            if let Some(widget) = self.frame.find_mut(hsid!(&hscbarid)) {
                if let Some(scbar) = widget.as_any_mut().downcast_mut::<ScrollBar>() {
                    scbar.set_color(new_color);
                }
            }
            if let Some(widget) = self.frame.find_mut(hsid!(&vscbarid)) {
                if let Some(scbar) = widget.as_any_mut().downcast_mut::<ScrollBar>() {
                    scbar.set_color(new_color);
                }
            }
        }
    }

    pub fn scrollbar_thumb_color(&mut self, new_color: Color) {
        if let Some(id) = get_id(self.id) {
            let hscbarid = format!("{}.h_scbar", id.clone());
            let vscbarid = format!("{}.v_scbar", id.clone());

            if let Some(widget) = self.frame.find_mut(hsid!(&hscbarid)) {
                if let Some(scbar) = widget.as_any_mut().downcast_mut::<ScrollBar>() {
                    scbar.thumbcolor(new_color);
                }
            }
            if let Some(widget) = self.frame.find_mut(hsid!(&vscbarid)) {
                if let Some(scbar) = widget.as_any_mut().downcast_mut::<ScrollBar>() {
                    scbar.thumbcolor(new_color);
                }
            }
        }
    }

    //sizes
    ///Sets by which axises widget will stretch and fill itself
    pub fn fill(&mut self, cords: Axis){
        self.frame.base.sizestrat.fill = cords;
        if cords == Axis::NONE {
            self.frame.base.sizestrat.method = SizeEnum::AUTO;
        } else {
            self.frame.base.sizestrat.method = SizeEnum::FILL;
        }
    }

    ///Sets greedness of widget
    ///Greedy widgets go onto other sides, widget from right line can go onto central and left lines if its size is big enough
    pub fn greedy(&mut self, greed: bool) {
        self.frame.base.layoutstrat.is_greedy = greed;
        self.set_relayout_flag(true);
    }

    ///Sets spaceness of widget
    ///Spacer widgets take space in other lines
    pub fn spacer(&mut self, spacer: bool) {
        self.frame.base.layoutstrat.is_spacer = spacer;
        self.set_relayout_flag(true);
    }

    ///Sets widget size, widget will not dynamicaly change size
    ///NOTE: Be careful while using it because if widget is too big or too small it or its child widgets will not be fully visible
    pub fn size(&mut self, size: Size) {
        self.frame.base.size = size;
        self.frame.base.sizestrat.method = SizeEnum::MANUAL;
        self.set_relayout_flag(true);
    }

    pub fn auto_size(&mut self) {
        self.frame.base.sizestrat.method = SizeEnum::AUTO;
        self.set_relayout_flag(true);
    }

    pub fn padding(&mut self, padding: i32) {
        if let Some(id) = get_id(self.id.clone()) {
            let containerid = format!("{}.container", id);
            if let Some(widget) = self.frame.find_mut(hsid!(&containerid)) {
                if let Some(frame) = widget.as_any_mut().downcast_mut::<Frame>() {
                    frame.padding(padding);
                    self.set_relayout_flag(true);
                }
            }
        }
    }

    pub fn style(&mut self, new_style: FrameStyle) {
        self.frame.style = new_style;
        self.set_relayout_flag(true);
    }

    ///Changes max widget size at runtime
    /// 
    ///Example arguments:
    ///None - Dont change that axis
    ///Some(None) - Removes limit
    ///Some(Some(i32)) - Sets limit
    pub fn max_size(&mut self, w: Option<Option<i32>>, h: Option<Option<i32>>) {
        if let Some(width) = w {
            self.frame.base.sizestrat.max_width = width;
        }
        if let Some(height) = h {
            self.frame.base.sizestrat.max_height = height;
        }
    }

    ///Changes minimal widget size
    /// 
    ///Example arguments:
    ///None - Dont change that axis
    ///Some(None) - Removes limit
    ///Some(Some(i32)) - Sets limit
    pub fn min_size(&mut self, w: Option<Option<i32>>, h: Option<Option<i32>>) {
        if let Some(width) = w {
            self.frame.base.sizestrat.min_width = width;
        }
        if let Some(height) = h {
            self.frame.base.sizestrat.min_height = height;
        }
    }

    ///Adds presetted widget inside self
    pub fn add_widget<W: Widget + 'static>(&mut self, new_widget: W) {
        if let Some(id) = get_id(self.id.clone()) {
            let containerid = format!("{}.container", id);

            if let Some(widget) = self.frame.find_mut(hsid!(&containerid)) {
                if let Some(frame) = widget.as_any_mut().downcast_mut::<Frame>() {
                    frame.add_widget(new_widget);
                    self.set_relayout_flag(true);
                }
            }
        }
    }

    ///Finds and removes widget by its id, NOT RECURSIVE
    pub fn remove_widget(&mut self, target_id: u64) -> bool {
        if let Some(id) = get_id(self.id.clone()) {
            let containerid = format!("{}.container", id.clone());

            if let Some(widget) = self.frame.find_mut(hsid!(&containerid)) {
                if let Some(frame) = widget.as_any_mut().downcast_mut::<Frame>() {
                    let a = frame.remove_widget(target_id);
                    frame.set_relayout_flag(true);
                    return a;
                }
            }   
        }
        false
    }

    fn recalculate_content_pos(&mut self, v_scval: Option<f32>, h_scval: Option<f32>) {
        if let Some(id) = get_id(self.id) {
            let containerid = format!("{}.container", id);
            let subcontainerid = format!("{}.subcontainer", id);

            let mut bordersize = 0;
            if let Some(widget) = self.frame.find(hsid!(&subcontainerid)) {
                if let Some(subcontentframe) = widget.as_any().downcast_ref::<Frame>() {
                    bordersize = match subcontentframe.style {
                        FrameStyle::FLAT => 0,
                        _ => 2,
                    };
                }
            }

            let selfsize = self.frame.get_size();

            if let Some(widget) = self.frame.find_mut(hsid!(&containerid)) {
                if let Some(contentframe) = widget.as_any_mut().downcast_mut::<Frame>() {
                    let contentsize = contentframe.get_size();
                    let mut newpos = contentframe.base.pos;

                    if let Some(scval) = v_scval {
                        if !self.is_bottom_to_top {
                            newpos.y = ((contentsize.height as f32 * scval) * -1.0).round() as i32 + bordersize;
                        } else {
                            newpos.y = (selfsize.height - contentsize.height) - ((contentsize.height as f32 * scval) * -1.0).round() as i32 + bordersize;
                        }
                    }

                    if let Some(scval) = h_scval {
                        newpos.x = ((contentsize.width as f32 * scval) * -1.0).round() as i32 + bordersize;
                    }

                    contentframe.set_pos(newpos);
                }
            }
        }
    }
}

impl Widget for ScrollFrame {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn get_id(&self) -> u64 {
        self.id
    }
    ///Recursively finds widget by its id and returns mutable reference to it, if you need read-only reference then use [`Scroll::find`]
    fn find_mut(&mut self, target_id: u64) -> Option<&mut dyn Widget> {
        if self.id == target_id {
            return Some(self);
        }
        self.frame.find_mut(target_id)
    }
    ///Recursively finds widget by its id and returns mutable reference to it, if you need read-only reference then use [`ScrollFrame::find_mut`]
    fn find(&self, target_id: u64) -> Option<&dyn Widget> {
        if self.id == target_id {
            return Some(self);
        }
        self.frame.find(target_id)
    }
    fn draw(&self, drawcommans: &mut Vec<DrawCommand>, pos_off: Pos, clip: Rect, _preferred_color: Option<Color>) {
        self.frame.draw(drawcommans, pos_off, clip, None);
    }

    fn update_layout(&mut self, forced: bool) {
        self.frame.update_layout(forced);

        if let Some(id) = get_id(self.id.clone()) {
            let containerid = format!("{}.container", id);
            let subcontainerid = format!("{}.subcontainer", id);

            let mut sub_size = Size::new(0, 0);
            if let Some(widget) = self.frame.find(hsid!(&subcontainerid)) {
                sub_size = widget.get_size();
            }

            if let Some(widget) = self.frame.find_mut(hsid!(&containerid)) {
                if let Some(container) = widget.as_any_mut().downcast_mut::<Frame>() {

                    if self.scrollaxis == Axis::Y {
                        container.base.size.width = sub_size.width;
                        container.base.sizestrat.min_width = Some(sub_size.width);
                    }

                    if self.scrollaxis == Axis::X {
                        container.base.size.height = sub_size.height;
                        container.base.sizestrat.min_height = Some(sub_size.height);
                    }

                    container.update_layout(true);
                }
            }

            let hscbarid = format!("{}.h_scbar", id.clone());
            let vscbarid = format!("{}.v_scbar", id.clone());

            let mut v_val = None;
            let mut h_val = None;

            if let Some(widget) = self.frame.find(hsid!(&vscbarid)) {
                if let Some(vscbar) = widget.as_any().downcast_ref::<ScrollBar>() {
                    v_val = Some(vscbar.scroll_value);
                }
            }

            if let Some(widget) = self.frame.find(hsid!(&hscbarid)) {
                if let Some(hscbar) = widget.as_any().downcast_ref::<ScrollBar>() {
                    h_val = Some(hscbar.scroll_value);
                }
            }

            self.recalculate_content_pos(v_val, h_val);
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
        self.frame.is_dirty()
    }
    fn set_dirty_flag(&mut self, flag: bool) {
        self.frame.set_dirty_flag(flag);
    }
    fn needs_relayout(&self) -> bool {
        self.frame.needs_relayout()
    }
    fn set_relayout_flag(&mut self, flag: bool) {
        self.frame.set_relayout_flag(flag);
    }
    fn handle_event(&mut self, event: &Event, pos_off: Pos, actions: &mut Vec<Action>) {
        let mut local_actions: Vec<Action> = Vec::new();

        self.frame.handle_event(event, pos_off, &mut local_actions);

        for action in local_actions {
            if let Some(id) = get_id(self.id.clone()) {
                let hscbarid = format!("{}.h_scbar", id.clone());
                let vscbarid = format!("{}.v_scbar", id.clone());

                if let ScrollChanged(scid, scval) = action {
                    if scid == hsid!(&vscbarid) {
                        self.recalculate_content_pos(Some(scval), None);
                        self.set_dirty_flag(true);
                    } else if scid == hsid!(&hscbarid) {
                        self.recalculate_content_pos(None, Some(scval));
                        self.set_dirty_flag(true);
                    }
                }
            }
            
            actions.push(action);
        }
    }
    /*
    fn get_dirty_rect(&mut self, pos_off: Pos, requests: &mut Vec<Action>) {
        if self.is_dirty() {
            if let Some(dirty_rect) = self.get_self_rect(pos_off) {
                requests.push(Action::RedrawRequest(Some(dirty_rect)));
            }
            self.set_dirty_flag(false);
        }
    }*/
    fn get_dirty_rect(&mut self, pos_off: Pos, requests: &mut Vec<Action>) {
        //let abs_pos = Pos::new(pos_off.x + self.get_pos().x, pos_off.y + self.get_pos().y);
        
        if self.frame.base.is_dirty {
            if let Some(dirty_rect) = self.get_self_rect(pos_off) {
                requests.push(Action::RedrawRequest(Some(dirty_rect)));
            }
            self.set_dirty_flag(false); 
            return;
        }

        if self.frame.is_dirty() {
            let mut child_requests = Vec::new();
            
            self.frame.get_dirty_rect(pos_off, &mut child_requests);

            if let Some(my_rect) = self.get_self_rect(pos_off) {
                for action in child_requests {
                    if let Action::RedrawRequest(Some(child_rect)) = action {
                        if let Some(clipped_rect) = intersect_rects(my_rect, child_rect) {
                            requests.push(Action::RedrawRequest(Some(clipped_rect)));
                        }
                    } else {
                        requests.push(action);
                    }
                }
            }
        }
    }
}