use crate::core::common::{Axis, LayoutEnum, LayoutStrat, Side, SizeEnum, SizeStrat};
use crate::core::event::{Action, DrawCommand, Event, MKey};
use crate::core::idpool::{regid, get_id};
use crate::hsid;
use crate::core::size::Size;
use crate::core::shapes::Rect;
use crate::core::widget::Widget;
use crate::core::{color::Color, pos::Pos};
use crate::widgets::frame::{Frame, FrameStyle};
use crate::widgets::label::Label;

///Buton struct, stores everything button needs
pub struct Button {
    pub id: u64,
    pub frame: Frame,
    pub hover_color: Option<Color>,
    is_hovered: bool,
    is_pressed: bool,
    is_styled: bool,
}

impl Button {
    pub fn new(id: String) -> Self {
        let mut framesetter = Frame::new(format!("{}.frame", id.clone()));
        framesetter.style(FrameStyle::RAISED);

        let mut textsetter = Label::new(format!("{}.label", id.clone()));
        textsetter.bgcolor(Color::TRANSPARENT);

        framesetter.add_widget(textsetter);

        Self {
            id: regid(id.clone()),
            frame: framesetter,
            hover_color: None,
            is_hovered: false,
            is_pressed: false,
            is_styled: true,
        }
    }

    ///Changes button text at runtime
    pub fn text(&mut self, new_text: String) {
        if let Some(id) = get_id(self.id.clone()) {
            let labelid = format!("{}.label", id.clone());

            if let Some(widget) = self.frame.find_mut(hsid!(&labelid)) {
                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                    label.set_text(new_text);
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
    ///Sets button color 
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
    pub fn hovercolor(&mut self, new_color: Color) {
        self.hover_color = Some(new_color);
        self.set_dirty_flag(true);
    }

    ///Will button slyling be applied?
    pub fn is_styled(&mut self, arg: bool) {
        self.is_styled = arg;
        self.frame.style = FrameStyle::FLAT;
        self.set_dirty_flag(true);
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

    ///Changes framestyle light/dark difference from base color at runtime
    pub fn light_change_amount(&mut self, amount: u8) {
        self.frame.lightchangeamount = amount;
        self.set_dirty_flag(true);
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

    pub fn font_size(&mut self, size: i32) {
        if let Some(id) = get_id(self.id.clone()) {
            let labelid = format!("{}.label", id.clone());

            if let Some(widget) = self.frame.find_mut(hsid!(&labelid)) {
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

impl Widget for Button {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn get_id(&self) -> u64 {
        self.id
    }
    fn draw(
        &self,
        drawcommans: &mut Vec<DrawCommand>,
        pos_off: Pos,
        clip: Rect,
        _preferred_color: Option<Color>,
    ) {
        //Getting display color depending on is button hovered
        let display_color = if self.is_hovered {
            self.hover_color
                .unwrap_or_else(|| self.frame.base.bgcolor.lighter(25))
        } else {
            self.frame.base.bgcolor
        };

        //println!("{:?}", display_color);

        self.frame.draw(drawcommans, pos_off, clip, Some(display_color));
    }

    fn update_layout(&mut self, forced: bool) {
        self.frame.update_layout(forced);//Layout update
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
                if self.is_point_inside(*pos, pos_off) && !self.is_pressed && *key == MKey::Left {
                    self.is_pressed = true;
                    actions.push(Action::ButtonClicked(self.id));
                    if self.is_styled {
                        self.frame.style(FrameStyle::SUNKEN);
                    }
                }
            }
            //Same thing here but we are pushing mouse release action
            Event::MouseRelease { pos, key } => {
                if self.is_point_inside(*pos, pos_off) && self.is_pressed && *key == MKey::Left  {
                    self.is_pressed = false;
                    actions.push(Action::ButtonReleased(self.id));
                    if self.is_styled {
                        self.frame.style(FrameStyle::RAISED);
                    }
                }
            }
            //Mouse hovering part
            Event::MouseMove { pos } => {
                let now_hovered = self.is_point_inside(*pos, pos_off);
                //If mouse is hovered then we are changing is_hovered, requesting redraw and pushing hovered action
                if now_hovered && !self.is_hovered {
                    self.is_hovered = true;
                    actions.push(Action::Hovered(self.id));
                    self.set_dirty_flag(true);
                //If mouse is unhovered then are setting style to raised, changing a lot of values and and pushing unhovered action
                } else if !now_hovered && self.is_hovered {
                    self.is_hovered = false;
                    if self.is_styled {
                        self.frame.style(FrameStyle::RAISED);
                    }
                    self.is_pressed = false;
                    actions.push(Action::Unhovered(self.id));
                    self.set_dirty_flag(true);
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
