#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}
impl Coord {
    pub fn origin() -> Coord {
        Coord { x: 0, y: 0 }
    }
}

impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl std::ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
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
        let rhs = Coord { x: -1, y: 7 };
        assert_eq!(lhs + rhs, Coord { x: 2, y: 12 });
    }

    #[test]
    fn subtraction() {
        let lhs = Coord { x: 3, y: 5 };
        let rhs = Coord { x: -1, y: 7 };
        assert_eq!(lhs - rhs, Coord { x: 4, y: -2 });
    }
}
