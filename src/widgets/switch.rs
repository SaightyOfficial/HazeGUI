use crate::core::common::{Axis, LayoutEnum, LayoutStrat, Side, SizeEnum, SizeStrat};
use crate::core::event::{Action, Event, MKey};
use crate::core::idpool::{get_id, regid};
use crate::core::size::Size;
use crate::core::widget::Widget;
use crate::core::{color::Color, pos::Pos};
use crate::hsid;
use crate::widgets::frame::{Frame, FrameStyle};
use crate::widgets::label::Label;
use tiny_skia::{PixmapMut, Rect};

///Switch struct, stores everything switch needs
pub struct Switch {
    pub id: u64,
    pub frame: Frame,
    pub check_color: Color,
    pub is_on: bool,
}

impl Switch {
    pub fn new(id: String) -> Self {
        let mut framesetter = Frame::new(format!("{}.frame", id.clone()));

        let mut textsetter = Label::new(format!("{}.label", id.clone()));
        textsetter.bgcolor(Color::TRANSPARENT);
        textsetter.side(Side::RIGHT);

        let mut checkcontsetter = Frame::new(format!("{}.checkcontframe", id.clone()));
        checkcontsetter.style(FrameStyle::SUNKEN);
        checkcontsetter.side(Side::LEFT);
        checkcontsetter.padding(3);

        let mut checksetter = Frame::new(format!("{}.checkframe", id.clone()));
        checksetter.min_size(Some(Some(10)), Some(Some(10)));
        //checksetter.size(Size::new(10, 10));
        
        checkcontsetter.add_widget(checksetter);

        framesetter.add_widget(textsetter);
        framesetter.add_widget(checkcontsetter);

        Self {
            id: regid(id.clone()),
            frame: framesetter,
            check_color: Color::BLACK,
            is_on: false,
        }
    }
    ///Changes switch text at runtime
    pub fn text(&mut self, new_text: String) {
        if let Some(id) = get_id(self.id.clone()) {
            let labelid = format!("{}.label", id.clone());

            if let Some(widget) = self.frame.find_mut(hsid!(&labelid)) {
                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                    label.text(new_text);
                }
            }
        }
    }

    //positions
    ///Manualy set widget position, widget will not participate in auto layout composing and just be where you said it to be
    pub fn pos(&mut self, posnew: Pos) {
        self.frame.base.pos = posnew;
        self.frame.base.layoutstrat.method = LayoutEnum::MANUAL;
        self.set_relayout_flag(true);
    }

    ///Changes widget side at runtime, there are only [`Side::LEFT`], [`Side::MIDDLE`] and [`Side::RIGHT`], Y axis position depends on order by which widgets are added in your code
    pub fn side(&mut self, side: Side) {
        self.frame.base.layoutstrat.side = side;
        self.set_relayout_flag(true);
    }

    //colors
    ///Sets switch color 
    pub fn color(&mut self, new_color: Color) {
        //self.text.base.bgcolor = new_color;
        self.frame.base.bgcolor = new_color;
        //println!("{:?}", self.frame.base.bgcolor.clone());
        self.set_dirty_flag(true);
    }

    ///Sets text color
    pub fn textcolor(&mut self, new_color: Color) {
        if let Some(id) = get_id(self.id.clone()) {
            let labelid = format!("{}.label", id.clone());

            if let Some(widget) = self.frame.find_mut(hsid!(&labelid)) {
                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                    label.color(new_color);
                }
            }
        }
    }

    ///Sets hover color
    pub fn checkcolor(&mut self, new_color: Color) {
        self.check_color = new_color;

        if self.is_on {
            if let Some(id) = get_id(self.id.clone()) {
                let wid = format!("{}.checkframe", id.clone());

                if let Some(widget) = self.frame.find_mut(hsid!(&wid)) {
                    if let Some(label) = widget.as_any_mut().downcast_mut::<Frame>() {
                        label.set_dirty_flag(true);
                    }
                }
            }
        }
    }

    pub fn checkcontcolor(&mut self, new_color: Color) {
        if let Some(id) = get_id(self.id.clone()) {
            let wid = format!("{}.checkcontframe", id.clone());

            if let Some(widget) = self.frame.find_mut(hsid!(&wid)) {
                if let Some(frame) = widget.as_any_mut().downcast_mut::<Frame>() {
                    frame.color(new_color);
                }
            }
        }
    }

    //sizes
    ///Changes at runtime by which axises widget will stretch and fill itself
    pub fn fill(&mut self, cords: Axis) {
        self.frame.base.sizestrat.fill = cords;
        if cords == Axis::NONE {
            self.frame.base.sizestrat.method = SizeEnum::AUTO;
        } else {
            self.frame.base.sizestrat.method = SizeEnum::FILL;
        }
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

    pub fn font_size(&mut self, size: f32) {
        if let Some(id) = get_id(self.id.clone()) {
            let wid = format!("{}.label", id.clone());

            if let Some(widget) = self.frame.find_mut(hsid!(&wid)) {
                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                    label.font_size(size);
                }
            }
        }
    }

    ///Changes max widget size at runtime
    /// 
    ///Example arguments:
    ///None - Dont tourch tha axis
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

    ///Changes minimal widget size at runtime
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
}

impl Widget for Switch {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn get_id(&self) -> u64 {
        self.id
    }
    fn draw(&self,pixmap: &mut PixmapMut,pos_off: Pos,clip: Rect,_preferred_color: Option<Color>) {
        self.frame.draw(pixmap, pos_off, clip, None);
    }

    fn update_layout(&mut self, forced: bool) {
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
        //Button event handling
        match event {
            //On mouse click we are checking is this button inside, then if it is changins some values and making the button pressed and pushing action
            Event::MouseClick { pos, key } => {
                if let Some(id) = get_id(self.id.clone()) {
                    let checkframeid = format!("{}.checkframe", id.clone());
                    
                    if self.is_point_inside(*pos, pos_off) && *key == MKey::Left  {
                        self.is_on = !self.is_on;

                        if let Some(widget) = self.frame.find_mut(hsid!(&checkframeid)) {
                            if let Some(frame) = widget.as_any_mut().downcast_mut::<Frame>() {
                                if self.is_on {
                                    frame.color(self.check_color);
                                } else {
                                    frame.color(Color::TRANSPARENT);
                                }
                                self.set_dirty_flag(true);
                            }
                        }

                        actions.push(Action::SwitchChanged(self.id, self.is_on));
                    }
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
