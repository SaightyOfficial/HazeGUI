use crate::core::size::Size;
use crate::core::event::{Action, Event};
use crate::core::{color::Color, pos::Pos};
use crate::core::common::{ChooseCords};
use crate::core::widget::{UsedCord, Widget, WidgetBase};
use tiny_skia::{PixmapMut, Paint, Rect, Color as SkiaColor};

use crate::core::common::{LayoutEnum, LayoutStrat, Side, intersect_rects};

pub struct Frame {
    pub base: WidgetBase,
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
            // Если ребенок — это фрейм, ищем внутри него
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

    //colors
    pub fn color(mut self, color: Color) -> Self {
        self.base.bgcolor = color;
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
        let mut max_child_width = 0;
        let mut current_total_height = 0;

        // Сначала просим всех детей посчитать себя (Label посчитает свои 104px)
        for child in &mut self.children {
            child.update_layout(); 
            let child_size = child.get_size();
            
            if child_size.width > max_child_width {
                max_child_width = child_size.width;
            }
            current_total_height += child_size.height;
        }

        // ВАЖНО: Если фрейм должен подстраиваться под контент, меняем его размер
        // Добавь проверку на стратегию или просто делай это для начала
        if self.base.size.width < max_child_width {
            self.base.size.width = max_child_width;
        }
        if self.base.size.height < current_total_height {
            self.base.size.height = current_total_height;
        }

        let parent_width = self.base.size.width;

        // Теперь, когда Frame расширился, расставляем детей по центру
        for child in &mut self.children {
            let layoutstrat = child.get_layout_strat();
            let child_size = child.get_size();

            if layoutstrat.method == LayoutEnum::AUTO && layoutstrat.side == Side::MIDDLE {
                let middlepos = (parent_width - child_size.width) / 2;
                child.set_pos(Pos::new(middlepos, self.usedmiddle.used_y));
                self.usedmiddle.used_y += child_size.height;
            }
        }
    }

    pub fn add_widget<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget));
        // После добавления нового виджета сразу обновляем позиции всех остальных
        self.update_layout();
    }
}

impl Widget for Frame {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn get_id(&self) -> &str { &self.base.id }
    fn draw(&self, pixmap: &mut PixmapMut, pos_off: Pos, clip: Rect) {
        let abs_x = (pos_off.x + self.base.pos.x) as f32;
        let abs_y = (pos_off.y + self.base.pos.y) as f32;

        let my_rect = Rect::from_xywh(
            abs_x, 
            abs_y, 
            self.base.size.width as f32, 
            self.base.size.height as f32
        ).unwrap();

        // 2. Находим ПЕРЕСЕЧЕНИЕ нашего ректа и того, что прислал родитель
        // Это и будет новая разрешенная зона для детей
        let inner_clip = match intersect_rects(clip, my_rect) {
            Some(r) => r,
            None => return, // Если мы вообще вне зоны видимости — не рисуем ничего
        };

        //BGRA is needed here
        let mut paint = Paint::default();
        paint.set_color(SkiaColor::from_rgba8(self.base.bgcolor.b, self.base.bgcolor.g, self.base.bgcolor.r, self.base.bgcolor.a));
        
        // В tiny-skia нет простого ClipStack, поэтому мы эмулируем его через маску или обрезая геометрию
        if let Some(visible_part) = intersect_rects(my_rect, clip) {
             pixmap.fill_rect(visible_part, &paint, tiny_skia::Transform::identity(), None);
        }

        // 4. Передаем эстафету детям с НОВЫМ ограничением
        for child in &self.children {
            child.draw(pixmap, Pos::new(abs_x as i32, abs_y as i32), inner_clip);
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