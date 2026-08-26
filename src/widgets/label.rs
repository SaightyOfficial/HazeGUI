use crate::core::common::{Axis, SizeEnum, SizeStrat, LayoutEnum, LayoutStrat, Side, intersect_rects};
use crate::core::event::{Action, DrawCommand, Event};
use crate::core::idpool::regid;
use crate::core::size::Size;
use crate::core::shapes::Rect;
use crate::core::widget::{Widget, WidgetBase};
use crate::core::{color::Color, pos::Pos};
use std::sync::Arc;
use crate::core::render::font::FONT;

///Label struct where all label data is stored
#[derive(Clone)]
pub struct Label {
    pub base: WidgetBase,
    pub text: Arc<String>,
    pub font_size: i32,
    pub textcolor: Color,
    padding: f32,
}

impl Label {
    pub fn new(id: String) -> Self {
        let mut label = Self {
            base: WidgetBase::new(regid(id)),
            text: Arc::from("".to_string()),
            font_size: 14,
            textcolor: Color::BLACK,
            padding: 2.0,
        };
        label.base.bgcolor = Color::TRANSPARENT;
        label.update_size();
        label
    }

    ///Recaulcalting and updating label size
    pub fn update_size(&mut self) {
        let old_size = self.base.size;
        let mut max_width: f32 = 0.0;
        let mut height = 0.0;
        let mut width = 0.0; //Line width

        //Getting char catrics
        let line_metrics = FONT.horizontal_line_metrics(self.font_size as f32);

        //Line height
        let lheight = line_metrics
            .map(|m| m.new_line_size)
            .unwrap_or(self.font_size as f32);

        height += lheight;

        for ch in self.text.chars() {
            if ch == '\n' {
                max_width = max_width.max(width);
                height += lheight;
                width = 0.0;
            } else {
                let metrics = FONT.metrics(ch, self.font_size as f32);
                width += metrics.advance_width;
            }
        }

        max_width = max_width.max(width);

        //Setting new size, and if size has changed, then request relayout
        self.base.size = Size::new((max_width + self.padding * 2.0) as i32, (height + self.padding) as i32);
        if old_size.width != self.base.size.width || old_size.height != self.base.size.height {
            self.set_relayout_flag(true);
        }
    }

    /// Получение cheap-clone Arc
    pub fn get_text(&self) -> Arc<String> {
        self.text.clone()
    }

    /// Неизменяемая ссылка на String
    pub fn get_text_ref(&self) -> &str {
        &self.text
    }

    /// Мутируемая ссылка на String с защитой Arc::make_mut
    /// Если Arc уникален, мутирует на месте без аллокаций.
    pub fn get_text_mutref(&mut self) -> &mut String {
        self.set_dirty_flag(true);
        // Важно: update_size вызовешь после изменений, если нужно
        Arc::make_mut(&mut self.text)
    }

    /// Быстрая установка нового текста (принимает String, &str или Arc<String>)
    pub fn set_text(&mut self, new_text: impl Into<Arc<String>>) {
        self.text = new_text.into();
        self.update_size();
        self.set_dirty_flag(true);
    }

    //positions
    pub fn pos(&mut self, posnew: Pos) {
        self.base.pos = posnew;
        self.base.layoutstrat.method = LayoutEnum::MANUAL;
        self.set_relayout_flag(true);
    }

    ///Sets greedness of widget
    ///Greedy widgets go onto other sides, widget from right line can go onto central and left lines if its size is big enough
    pub fn greedy(&mut self, greed: bool) {
        self.base.layoutstrat.is_greedy = greed;
        self.set_relayout_flag(true);
    }

    ///Sets spaceness of widget
    ///Spacer widgets take space in other lines
    pub fn spacer(&mut self, spacer: bool) {
        self.base.layoutstrat.is_spacer = spacer;
        self.set_relayout_flag(true);
    }

    ///Changes widget side at runtime, there are only [`Side::LEFT`], [`Side::MIDDLE`] and [`Side::RIGHT`], Y axis position depends on order by which widgets are added in your code
    pub fn side(&mut self, side: Side) {
        self.base.layoutstrat.side = side;
        self.set_relayout_flag(true);
    }

    //colors
    ///Setting font size for label at runtime
    pub fn font_size(&mut self, size: i32) {
        self.font_size = size;
        self.update_size();
        self.set_relayout_flag(true);
        self.set_dirty_flag(true);
    }

    ///Setting text color at runtime
    pub fn color(&mut self, color: Color) {
        self.textcolor = color;
        self.set_dirty_flag(true);
    }

    ///Setting background color at runtime
    pub fn bgcolor(&mut self, color: Color) {
        self.base.bgcolor = color;
        self.set_dirty_flag(true);
    }

    //sizes
    ///Changes at runtime by which axises widget will stretch and fill itself
    pub fn fill(&mut self, cords: Axis){
        self.base.sizestrat.fill = cords;
        if cords == Axis::NONE {
            self.base.sizestrat.method = SizeEnum::AUTO;
        } else {
            self.base.sizestrat.method = SizeEnum::FILL;
        }
    }

    ///Sets widget size, widget will not dynamicaly change size
    ///NOTE: Be careful while using it because if widget is too big it will be not fully visible
    pub fn size(&mut self, size: Size) {
        self.base.size = size;
        self.base.sizestrat.method = SizeEnum::MANUAL;
        self.set_relayout_flag(true);
    }

    pub fn auto_size(&mut self) {
        self.base.sizestrat.method = SizeEnum::AUTO;
        self.set_relayout_flag(true);
    }

    pub fn get_cursor_pos(&self, byte_offset: usize) -> (Pos, i32) {
        let line_metrics = FONT.horizontal_line_metrics(self.font_size as f32);
        let lheight = line_metrics.map(|m| m.new_line_size).unwrap_or(self.font_size as f32);

        let mut x = self.padding;
        let mut y = self.padding / 2.0;

        for (idx, c) in self.text.char_indices() {
            if idx >= byte_offset {
                break;
            }

            if c == '\n' {
                x = self.padding;
                y += lheight;
            } else {
                let metrics = FONT.metrics(c, self.font_size as f32);
                x += metrics.advance_width;
            }
        }

        let pos = Pos {
            x: x.round() as i32,
            y: y.round() as i32,
        };
        let height = lheight.round() as i32;

        (pos, height)
    }

    pub fn get_char_index_at_x(&self, target_x: f32) -> usize {
        let mut current_x = self.padding;

        for (idx, c) in self.text.char_indices() {
            if c == '\n' {
                // Если текст однострочный в тексбоксе, то при переходе строки можно завершать
                break;
            }

            let metrics = FONT.metrics(c, self.font_size as f32);
            let char_width = metrics.advance_width;

            // Половина ширины символа — чтобы клик ближе к правому краю буквы ставил каретку ПОСЛЕ неё
            let half_char = char_width / 2.0;

            if target_x < current_x + half_char {
                return idx;
            }

            current_x += char_width;
        }

        // Если кликнули правее самого последнего символа — ставим каретку в самый конец строки
        self.text.len()
    }
}

impl Widget for Label {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn get_id(&self) -> u64 {
        self.base.id
    }
    fn draw(
        &self,
        drawcommands: &mut Vec<DrawCommand>,
        pos_off: Pos,
        clip: Rect,
        _preferred_color: Option<Color>,
    ) {
        //Getting absolute coordinates
        let abs_x = pos_off.x + self.base.pos.x;
        let abs_y = pos_off.y + self.base.pos.y;

        //Backgournd and text render
        if let Some(bg_rect) = Rect::from_xywh(abs_x, abs_y, self.base.size.width, self.base.size.height) {
            if let Some(visible_bg) = intersect_rects(bg_rect, clip) {
                drawcommands.push(DrawCommand::Text(visible_bg, self.base.bgcolor, self.textcolor, self.text.clone(), visible_bg, self.font_size, self.padding));
            }
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
        self.base.size.width = size_new.width;
        self.base.size.height = size_new.height;
    }
    fn set_pos(&mut self, pos_new: Pos) {
        self.base.pos.x = pos_new.x;
        self.base.pos.y = pos_new.y;
    }
    fn is_dirty(&self) -> bool {
        self.base.is_dirty
    }
    fn set_dirty_flag(&mut self, flag: bool) {
        self.base.is_dirty = flag;
    }
    fn needs_relayout(&self) -> bool {
        self.base.needs_relayout
    }
    fn set_relayout_flag(&mut self, flag: bool) {
        self.base.needs_relayout = flag;
    }
    fn update_layout(&mut self, _forced: bool) {}
    fn handle_event(&mut self, _event: &Event, _pos_off: Pos, _actions: &mut Vec<Action>) {
        if self.needs_relayout() {
            self.set_relayout_flag(false);
        }
    }
    fn get_dirty_rect(&mut self, pos_off: Pos, requests: &mut Vec<Action>) {
        //If self is dirty, then push redraw request with self rect inside
        if self.is_dirty() {
            if let Some(dirty_rect) = self.get_self_rect(pos_off) {
                requests.push(Action::RedrawRequest(Some(dirty_rect)));
            }
            self.set_dirty_flag(false); //Clearing flag for self
        }
    }
}
