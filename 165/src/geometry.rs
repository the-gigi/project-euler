use core::cmp;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
}

impl Rectangle {
    pub fn intersect(&self, other: &Rectangle) -> bool {
        !self.above(other) && !self.below(other) && !self.left_of(other) && !self.right_of(other)
    }

    pub fn above(&self, other: &Rectangle) -> bool {
        self.bottom >= other.top
    }

    pub fn below(&self, other: &Rectangle) -> bool {
        self.top <= other.bottom
    }

    pub fn left_of(&self, other: &Rectangle) -> bool {
        self.right <= other.left
    }

    pub fn right_of(&self, other: &Rectangle) -> bool {
        self.left >= other.right
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        self.left as f64 <= x &&
            self.right as f64 >= x &&
            self.top as f64 <= y &&
            self.bottom as f64 >= y
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Coordinates {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub coords: Coordinates,
    pub bounding_box: Rectangle,
    slope: f64,
    y_intersept: f64,
}

impl Segment {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        let bbox = Rectangle {
            top: cmp::min(y1, y2),
            left: cmp::min(x1, x2),
            bottom: cmp::max(y1, y2),
            right: cmp::max(x1, x2),
        };
        let slope = (y2 - y1) as f64 / (x2 - x1) as f64;
        Segment {
            coords: Coordinates { x1, y1, x2, y2 },
            bounding_box: bbox,
            slope,
            y_intersept: y1 as f64 - slope * x1 as f64,
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
    pub fn intersect(&self, other: &Segment) -> bool {
        // Check if the segments have the same slope
        if self.slope == other.slope {
            return false;
        }

        // Check if the bounding boxes of the segments intersect
        if !self.bounding_box.intersect(&other.bounding_box) {
            return false;
        }

        // Calculate intersection point of the segment lines
        let x = (other.y_intersept - self.y_intersept) / (self.slope - other.slope);
        let y = self.slope * x + self.y_intersept;

        // If the intersection is a segment end then it is not a true intersection point
        let endpoints: Vec<(f64, f64)> = vec![
            (self.coords.x1, self.coords.y1),
            (self.coords.x2, self.coords.y2),
            (other.coords.x1, other.coords.y1),
            (other.coords.x2, other.coords.y2)]
            .iter()
            .map(|(a, b)| (*a as f64, *b as f64))
            .collect();
        if endpoints.contains(&(x, y)) {
            return false;
        }

        // If the intersection point is in the bounding box of both segments then they intersect
        self.bounding_box.contains(x, y) && other.bounding_box.contains(x, y)
    }
}
