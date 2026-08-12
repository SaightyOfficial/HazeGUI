use tiny_skia::Rect;

use crate::core::errors::RectError;

/// Enum used for storing layout strategy in [`LayoutStrat`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutEnum {
    MANUAL,
    AUTO,
}

/// Enum used for storing size strategy in [`SizeStrat`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeEnum {
    MANUAL,
    FILL,
    AUTO,
}

/// Enum used for storing sides, mainly used in [`SizeStrat`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    LEFT,
    MIDDLE,
    RIGHT,
}

/// Enum used for storing coordinate ways, mainly used in [`LayoutStrat`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    X,
    Y,
    BOTH,
    NONE,
}

/// Struct used for size strategy storing and managing
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SizeStrat {
    pub method: SizeEnum,
    pub fill: Axis,
    pub max_width: Option<i32>,
    pub max_height: Option<i32>,
    pub min_width: Option<i32>,
    pub min_height: Option<i32>,
}

impl Default for SizeStrat {
    fn default() -> Self {
        Self {
            method: SizeEnum::AUTO,
            fill: Axis::NONE,
            max_height: None,
            max_width: None,
            min_height: None,
            min_width: None,
        }
    }
}

/// Struct used for layout strategy storing and managing
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutStrat {
    pub method: LayoutEnum,
    pub side: Side,
    //pub is_spacer: bool,
}

impl Default for LayoutStrat {
    fn default() -> Self {
        Self {
            method: LayoutEnum::AUTO,
            side: Side::MIDDLE,
            //is_spacer: false,
        }
    }
}

/// Checks if [`tiny_skia::Rect`] are intersecting each other
pub fn intersect_rects(a: Rect, b: Rect) -> Option<Rect> {
    let left = a.left().max(b.left()); //Getting maximum left value
    let top = a.top().max(b.top()); //Getting maximum top value
    let right = a.right().min(b.right()); //Getting maximum right value
    let bottom = a.bottom().min(b.bottom()); //Getting maximum bottom value

    if left < right && top < bottom {
        //Checking if rects are intersecting
        Rect::from_ltrb(left, top, right, bottom)
    } else {
        None
    }
}

/// Merges two [`tiny_skia::Rect`] into a bigger one by making a bigger one from max/min coordinates
pub fn merge_rects(a: Rect, b: Rect) -> Result<Rect, RectError> {
    let left = a.left().min(b.left()); //Getting minimum left value
    let top = a.top().min(b.top()); //Getting minimum top value
    let right = a.right().max(b.right()); //Getting maximum right value
    let bottom = a.bottom().max(b.bottom()); //Getting maximum bottom value

    let merged = tiny_skia::Rect::from_xywh(left, top, right - left, bottom - top)
        .ok_or(RectError::InvalidRectSize)?;

    Ok(merged)
}