#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}
impl Coord {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub const fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl std::ops::Add<Vec2D> for Coord {
    type Output = Coord;

    fn add(self, rhs: Vec2D) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl std::ops::Sub for Coord {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2D {
    pub x: isize,
    pub y: isize,
}
impl Vec2D {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl std::ops::Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl std::ops::Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let lhs = Coord { x: 3, y: 5 };
        let rhs = Vec2D { x: -1, y: 7 };
        assert_eq!(lhs + rhs, Coord { x: 2, y: 12 });
    }

    #[test]
    fn subtraction() {
        let lhs = Coord { x: 3, y: 5 };
        let rhs = Coord { x: -1, y: 7 };
        assert_eq!(lhs - rhs, Vec2D { x: 4, y: -2 });
    }
}
