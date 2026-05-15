use tiny_skia::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutEnum {
    MANUAL,
    AUTO,
    SIDEAUTO,
    GRID,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeEnum {
    MANUAL,
    AUTO,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    LEFT,
    MIDDLE,
    RIGHT,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChooseCords {
    X,
    Y,
    BOTH,
    NONE,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SizeStrat {
    pub method: SizeEnum,
    pub fill: ChooseCords,
}

impl Default for SizeStrat {
    fn default() -> Self {
        Self { method: SizeEnum::AUTO, fill: ChooseCords::NONE }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutStrat {
    pub method: LayoutEnum,
    pub side: Side,
}

impl Default for LayoutStrat {
    fn default() -> Self {
        Self { method: LayoutEnum::AUTO, side: Side::MIDDLE }
    }
}

pub fn intersect_rects(a: Rect, b: Rect) -> Option<Rect> {
    let left = a.left().max(b.left());
    let top = a.top().max(b.top());
    let right = a.right().min(b.right());
    let bottom = a.bottom().min(b.bottom());

    if left < right && top < bottom {
        Rect::from_ltrb(left, top, right, bottom)
    } else {
        None
    }
}