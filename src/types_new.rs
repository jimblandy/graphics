//! Contains types used in this library
use std::convert::From;
use std::ops::{ Add, Mul, Sub };

pub use math::{ Matrix2d, Scalar, Vec2d };

/// The size of a shape.
#[derive(Clone, Copy, Debug)]
pub struct Size {
    /// The horizontal length of the shape (width).
    pub w: Scalar,
    /// The vertical length of the shape (height).
    pub h: Scalar,
}

impl From<Vec2d> for Size {
    fn from(v: Vec2d) -> Size {
        Size { w: v[0], h: v[1] }
    }
}

impl Size {
    /// Convert size to a vector.
    pub fn to_vec2d(&self) -> Vec2d {
        [self.w, self.h]
    }
}

impl Mul<Vec2d> for Size {
    type Output = Size;

    fn mul(self, v: Vec2d) -> Size {
        Size { w: self.w * v[0], h: self.h * v[1] }
    }
}

impl Mul<Scalar> for Size {
    type Output = Size;

    fn mul(self, s: Scalar) -> Size {
        Size { w: self.w * s, h: self.h * s }
    }
}

/// A point in the Cartesian plane.
#[derive(Clone, Copy, Debug)]
pub struct Point {
    /// The x coordinate.
    pub x: Scalar,
    /// The y coordinate.
    pub y: Scalar,
}

impl Add<Scalar> for Point {
    type Output = Point;

    fn add(self, s: Scalar) -> Point {
        Point { x: self.x + s, y: self.y + s }
    }
}

impl Add<Vec2d> for Point {
    type Output = Point;

    fn add(self, v: Vec2d) -> Point {
        Point { x: self.x + v[0], y: self.y + v[1] }
    }
}

impl From<Vec2d> for Point {
    fn from(v: Vec2d) -> Point {
        Point { x: v[0], y: v[1] }
    }
}

impl Sub<Scalar> for Point {
    type Output = Point;

    fn sub(self, s: Scalar) -> Point {
        Point { x: self.x - s, y: self.y - s }
    }
}

impl Sub<Vec2d> for Point {
    type Output = Point;

    fn sub(self, v: Vec2d) -> Point {
        Point { x: self.x - v[0], y: self.y - v[1] }
    }
}

impl Point {
    /// Convert the point to a vector.
    pub fn to_vec2d(&self) -> Vec2d {
        [self.x, self.y]
    }
}

/// A rectangle.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    /// The position of the top left corner of the rectangle.
    pub pos: Point,
    /// The width and height of the rectangle.
    pub size: Size,
}

impl From<(Point, Size)> for Rect {
    /// Creates a rectangle from the position of its top left corner and its size.
    fn from(rectangle: (Point, Size)) -> Rect {
        let (pos, size) = rectangle;
        Rect { pos: pos, size: size }
    }
}

impl From<[Scalar; 4]> for Rect {
    /// Creates a rectangle from an array.
    fn from(v: [Scalar; 4]) -> Rect {
        Rect {
            pos: Point { x: v[0], y: v[1] },
            size: Size { w: v[2], h: v[3] },
        }
    }
}

impl Rect {
    /// Returns the position of the bottom side of the rectangle.
    pub fn bottom(&self) -> Scalar {
        self.pos.y + self.size.h
    }

    /// Computes a rectangle with quadruple the surface area of self and with center
    /// (self.x, self.y).
    pub fn centered(&self) -> Rect {
        Rect {
            pos: Point {
                 x: self.pos.x - self.size.w,
                 y: self.pos.y - self.size.h,
            },
            size: self.size * 2.0,
        }
    }

    /// Compute whether or not the point is inside the rectangle.
    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        self.left() < point.x && point.x < self.right() &&
        self.top() < point.y && point.y < self.bottom()
    }

    /// Create a rectangle that circumscribes the given circle.
    pub fn new_circle(center: Point, radius: Scalar) -> Rect {
        Rect {
            pos: Point {
                x: center.x - radius,
                y: center.y - radius,
            },
            size: Size {
                w: 2.0 * radius,
                h: 2.0 * radius,
            },
        }
    }

    /// Create a square rectangle with sides of length len and top left corner at pos.
    pub fn new_square(pos: Point, len: Scalar) -> Rect {
        Rect {
            pos: pos,
            size: Size { w: len, h: len },
        }
    }

    /// Converts a rectangle into [x, y, w, h].
    pub fn into_array(self) -> [Scalar; 4] {
        [self.pos.x, self.pos.y, self.size.w, self.size.h]
    }

    /// Returns the position of the left side of the rectangle.
    pub fn left(&self) -> Scalar {
        self.pos.x
    }

    /// Computes a rectangle whose perimeter forms the inside edge of margin with size m for self.
    #[inline(always)]
    pub fn margin(&self, m: Scalar) -> Rect {
        let w = self.size.w - 2.0 * m;
        let h = self.size.h - 2.0 * m;
        let (x, w)
            =   if w < 0.0 {
                    (self.pos.x + 0.5 * self.size.w, 0.0)
                } else {
                    (self.pos.x + m, w)
                };
        let (y, h)
            =   if h < 0.0 {
                    (self.pos.y + 0.5 * self.size.h, 0.0)
                } else {
                    (self.pos.y + m, h)
                };

        Rect {
            pos: Point { x: x, y: y },
            size: Size { w: w, h: h },
        }
    }

    /// Computes a rectangle translated (slid) in the direction of the vector a distance relative
    /// to the size of the rectangle. For example, self.relative([1.0, 1.0]) returns a rectangle
    /// one rectangle to the right and down from the original.
    #[inline(always)]
    pub fn relative(&self, v: Vec2d) -> Rect {
        Rect {
            pos: self.pos + (self.size * v).to_vec2d(),
            size: self.size,
        }
    }

    /// Returns the position of the right side of the rectangle.
    pub fn right(&self) -> Scalar {
        self.pos.x + self.size.w
    }

    /// Computes a scaled rectangle with the same position as self.
    pub fn scaled(&self, v: Vec2d) -> Rect {
        Rect {
            pos: self.pos,
            size: self.size * v,
        }
    }

    /// Converts a rectangle to [x, y, w, h].
    pub fn to_array(&self) -> [Scalar; 4] {
        [self.pos.x, self.pos.y, self.size.w, self.size.h]
    }

    /// Returns the position of the top side of the rectangle.
    pub fn top(&self) -> Scalar {
        self.pos.y
    }
}
