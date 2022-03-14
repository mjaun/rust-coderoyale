use std::ops::Add;
use std::ops::Sub;

#[derive(Copy, Clone)]
struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    fn len(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn mul(self, f: f64) -> Vector2 {
        Self {x: self.x * f, y: self.y * f}
    }

    fn div(self, f: f64) -> Vector2 {
        Self {x: self.x / f, y: self.y / f}
    }

    fn norm(self) -> Vector2 {
        self.div(self.len())
    }

    fn dot(v1: Vector2, v2: Vector2) -> f64 {
        v1.x * v2.x + v1.y * v2.y
    }

    fn comp(self, other: Vector2) -> f64 {
        Vector2::dot(self, other) / self.len()
    }

    fn proj(self, other: Vector2) -> Vector2 {
        self.norm().mul(self.comp(other))
    }

    fn angle(v1: Vector2, v2: Vector2) -> f64 {
        Vector2::dot(v1.norm(), v2.norm()).acos()
    }

    fn perp_cw(self) -> Vector2 {
        Vector2 { x: self.y, y: -self.x }
    }

    fn perp_ccw(self) -> Vector2 {
        Vector2 { x: -self.y, y: self.x }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

