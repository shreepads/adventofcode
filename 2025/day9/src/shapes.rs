// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Segment {
    pub start: Point,
    pub end: Point,
}

impl Segment {
    pub fn new(xstart: i32, ystart: i32, xend: i32, yend: i32) -> Segment {
        Segment {
            start: Point {
                x: xstart,
                y: ystart,
            },
            end: Point { x: xend, y: yend },
        }
    }
}

// Ignore 'thin' rectangles here i.e. unit width or length
#[derive(Debug)]
pub struct ThickInnerRectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl ThickInnerRectangle {
    pub fn new(xi: i32, yi: i32, xj: i32, yj: i32) -> Option<ThickInnerRectangle> {
        // Check it is thick
        if (xi - xj).abs() < 3 {
            return None;
        }

        if (yi - yj).abs() < 3 {
            return None;
        }

        // Find top left and bottom right corners of inner rectangle
        let (xtl, ytl, xbr, ybr) = if xi < xj && yi < yj {
            (xi + 1, yi + 1, xj - 1, yj - 1)
        } else if xi > xj && yi > yj {
            (xj + 1, yj + 1, xi - 1, yi - 1)
        } else if xi < xj && yi > yj {
            (xi + 1, yj + 1, xj - 1, yi - 1)
        } else if xi > xj && yi < yj {
            (xj + 1, yi + 1, xi - 1, yj - 1)
        } else {
            panic!("Unexpected rectange corners");
        };

        // Double check tl br
        assert!(xtl < xbr);
        assert!(ytl < ybr);

        Some(ThickInnerRectangle {
            top_left: Point { x: xtl, y: ytl },
            bottom_right: Point { x: xbr, y: ybr },
        })
    }

    pub fn intersects(&self, segment: &Segment) -> bool {
        // Check if lies left
        if segment.start.x < self.top_left.x && segment.end.x < self.top_left.x {
            return false;
        }

        // Check if lies right
        if segment.start.x > self.bottom_right.x && segment.end.x > self.bottom_right.x {
            return false;
        }

        // Check if lies above
        if segment.start.y < self.top_left.y && segment.end.y < self.top_left.y {
            return false;
        }

        // Check if lies below
        if segment.start.y > self.bottom_right.y && segment.end.y > self.bottom_right.y {
            return false;
        }

        true
    }
}
