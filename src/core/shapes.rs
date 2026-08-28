use crate::core::{pos::Pos, size::Size};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
}

impl Rect {
    pub fn from_ps(pos: Pos, size: Size) -> Option<Self> {
        Some(Rect { x: pos.x, y: pos.y, width: size.width, height: size.height })
    }

    pub fn from_xywh(x: i32, y: i32, width: i32, height: i32) -> Option<Self> {
        if width < 0 || height < 0 {
            return None;
        }
        Some(Rect { x: x, y: y, width: width, height: height })
    }

    pub fn from_ltrb(left: i32, top: i32, right: i32, bottom: i32) -> Option<Self> {
        if right < left || bottom < top {
            return None;
        }
        Some(Rect { x: left, y: top, width: right - left, height: bottom - top})
    }

    #[inline] pub fn left(&self) -> i32 { self.x }
    #[inline] pub fn top(&self) -> i32 { self.y }
    #[inline] pub fn right(&self) -> i32 { self.x + self.width }
    #[inline] pub fn bottom(&self) -> i32 { self.y + self.height }

    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
    }
}
