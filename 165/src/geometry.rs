use core::cmp;

use num_rational;
use num_rational::Rational64;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub top: Rational64,
    pub left: Rational64,
    pub bottom: Rational64,
    pub right: Rational64,
}

impl Rectangle {
    pub fn intersect(&self, other: &Rectangle) -> bool {
        let above = self.above(other);
        let below = self.below(other);
        let left_of = self.left_of(other);
        let right_of = self.right_of(other);

        !above && !below && !left_of && !right_of
    }

    pub fn above(&self, other: &Rectangle) -> bool {
        self.bottom < other.top
    }

    pub fn below(&self, other: &Rectangle) -> bool {
        self.top > other.bottom
    }

    pub fn left_of(&self, other: &Rectangle) -> bool {
        self.right < other.left
    }

    pub fn right_of(&self, other: &Rectangle) -> bool {
        self.left > other.right
    }

    pub fn contains(&self, x: Rational64, y: Rational64) -> bool {
        x >= self.left &&
        x <= self.right &&
        y >= self.top &&
        y <= self.bottom
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Coordinates {
    x1: Rational64,
    y1: Rational64,
    x2: Rational64,
    y2: Rational64,
}

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub coords: Coordinates,
    pub bounding_box: Rectangle,
    slope: Rational64,
    y_intercept: Rational64,
    is_vertical: bool
}

impl Segment {
    pub fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        let bbox = Rectangle {
            top: Rational64::from_integer(cmp::min(y1, y2)),
            left: Rational64::from_integer(cmp::min(x1, x2)),
            bottom: Rational64::from_integer(cmp::max(y1, y2)),
            right: Rational64::from_integer(cmp::max(x1, x2)),
        };
        let mut slope = Rational64::default();
        let mut y_intercept = Rational64::default();
        if x1 != x2 {
            slope = Rational64::from_integer(y2 - y1) / Rational64::from_integer(x2 - x1);
            y_intercept = Rational64::from_integer(y1) - slope * Rational64::from_integer(x1);
        }
        Segment {
            coords: Coordinates {
                x1: Rational64::from_integer(x1),
                y1: Rational64::from_integer(y1),
                x2:Rational64::from_integer(x2),
                y2:Rational64::from_integer(y2),
            },
            bounding_box: bbox,
            slope,
            y_intercept,
            is_vertical: x1 == x2,
        }
    }

    /**
     * intersect()
     *
     * Calculates if two line segments intersect.
     *
     * @param other - The other line segment to check for intersection
     *
     * @return - true if the two line segments intersect, otherwise false
     */
    pub fn intersect(&self, other: &Segment) -> (Rational64, Rational64, bool) {
        // Check if the segments have the same slope
        if self.slope == other.slope {
            return (Rational64::default(), Rational64::default(), false);
        }

        // Check if the bounding boxes of the segments intersect
        if !self.bounding_box.intersect(&other.bounding_box) {
            return (Rational64::default(), Rational64::default(), false);
        }

        // Calculate intersection point of the segment lines
        let mut x: Rational64;
        let mut y: Rational64;
        if self.is_vertical {
            x = self.coords.x1;
            y = other.slope * x + other.y_intercept;
        } else if other.is_vertical {
            x = other.coords.x1;
            y = self.slope * x + self.y_intercept;
        } else {
            x = (other.y_intercept - self.y_intercept) / (self.slope - other.slope);
            y = self.slope * x + self.y_intercept;
        }

        // Check if the intersection point is an end point of one of the segments
        if (x, y) == (self.coords.x1, self.coords.y1) ||
           (x,y) == (self.coords.x2, self.coords.y2) ||
           (x, y) == (other.coords.x1, other.coords.y1) ||
           (x,y) == (other.coords.x2, other.coords.y2) {
            return (Rational64::default(), Rational64::default(), false);
        }

        // If the intersection point is in the bounding box of both segments then they intersect
        if !self.bounding_box.contains(x, y) || !other.bounding_box.contains(x, y) {
            return (Rational64::default(), Rational64::default(), false);
        }
        (x, y, true)
    }
}
