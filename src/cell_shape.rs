use crate::{coord::{Coord, Vec2D}, transform::Transform};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Filled,
    Empty
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// An Cell Shape, anywhere on the grid. 
/// Different Oritentations and reflections are considered distinct, but not translations.
pub struct CellShape {
    // easiest to just represent it as a list of filled tiles for now
    // sorted by coord_cmp, adjusted so the minimum coordinates are 0
    tiles: Vec<Coord>,
}

fn coord_cmp(lhs: &Coord, rhs: &Coord) -> std::cmp::Ordering {
    lhs.x.cmp(&rhs.x)
        .then(lhs.y.cmp(&rhs.y))
}

impl CellShape {
    pub fn empty() -> CellShape {
        CellShape {
            tiles: Vec::new(),
        }
    }
    pub fn from_2darray<const N: usize, const M: usize>(grid: [[Tile; M]; N]) -> CellShape {
        let mut filled_tiles = Vec::new();
        for y in 0..N {
            for x in 0..M {
                let coord = Coord {
                    x: x as isize, 
                    y: y as isize
                };
                if grid[y][x] == Tile::Filled {
                    filled_tiles.push(coord);
                }
            }
        }
        CellShape::from_coordinate_list(filled_tiles)
    }
    pub fn from_coordinate_list(mut coords: Vec<Coord>) -> CellShape {
        if coords.is_empty() {
            return CellShape::empty();
        }
        let min_x = coords.iter().map(|c| c.x).min().unwrap();
        let min_y = coords.iter().map(|c| c.y).min().unwrap();
        coords
            .iter_mut()
            .for_each(|c| {
                c.x -= min_x;
                c.y -= min_y;
            });

        coords.sort_by(coord_cmp);
        coords.dedup();

        CellShape {
            tiles: coords
        }
    }
    /// Iterates over the filled tiles, in the local coordinate system
    pub fn filled_tiles(&self) -> impl Iterator<Item = Coord> + '_ {
        self.tiles.iter().copied()
    }
}

impl std::ops::Index<Coord> for CellShape {
    type Output = Tile;

    fn index(&self, coord: Coord) -> &Self::Output {
        if self.tiles.contains(&coord) {
            &Tile::Filled
        } else {
            &Tile::Empty
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works() {
        use Tile::*;

        let tg = CellShape::from_2darray([
            [Empty,  Empty,  Filled],
            [Empty,  Filled, Filled],
            [Filled,  Empty, Filled],
        ]);

        assert_eq!(tg[Coord { x: -1, y: 0 }], Empty);
        assert_eq!(tg[Coord { x: -1, y: 1 }], Empty);
        assert_eq!(tg[Coord { x: -1, y: 2 }], Empty);

        assert_eq!(tg[Coord { x: 0, y: -1 }], Empty);
        assert_eq!(tg[Coord { x: 1, y: -1 }], Empty);
        assert_eq!(tg[Coord { x: 2, y: -1 }], Empty);

        assert_eq!(tg[Coord { x: 0, y: 0 }], Empty);
        assert_eq!(tg[Coord { x: 1, y: 0 }], Empty);
        assert_eq!(tg[Coord { x: 2, y: 0 }], Filled);

        assert_eq!(tg[Coord { x: 0, y: 1 }], Empty);
        assert_eq!(tg[Coord { x: 1, y: 1 }], Filled);
        assert_eq!(tg[Coord { x: 2, y: 1 }], Filled);

        assert_eq!(tg[Coord { x: 0, y: 2 }], Filled);
        assert_eq!(tg[Coord { x: 1, y: 2 }], Empty);
        assert_eq!(tg[Coord { x: 2, y: 2 }], Filled);

        assert_eq!(tg[Coord { x: 0, y: 3 }], Empty);
        assert_eq!(tg[Coord { x: 1, y: 3 }], Empty);
        assert_eq!(tg[Coord { x: 2, y: 3 }], Empty);

        assert_eq!(tg[Coord { x: 3, y: 0 }], Empty);
        assert_eq!(tg[Coord { x: 3, y: 1 }], Empty);
        assert_eq!(tg[Coord { x: 3, y: 2 }], Empty);

    }

    #[test]
    fn it_works_rectangular() {
        use Tile::*;

        let tg = CellShape::from_2darray([
            [Empty,  Empty],
            [Empty,  Filled],
            [Filled,  Empty],
        ]);

        assert_eq!(tg[Coord { x: 0, y: 0 }], Empty);
        assert_eq!(tg[Coord { x: 1, y: 0 }], Filled);

        assert_eq!(tg[Coord { x: 0, y: 1 }], Filled);
        assert_eq!(tg[Coord { x: 1, y: 1 }], Empty);
    }


    #[test]
    fn equal_upto_vertical_shift() {
        use Tile::*;

        let lhs = CellShape::from_2darray([
            [Empty,  Filled],
            [Filled,  Empty],
        ]);

        let rhs = CellShape::from_2darray([
            [Empty,   Empty],
            [Empty,   Empty],
            [Empty,   Empty],
            [Empty,  Filled],
            [Filled,  Empty],
            [Empty,   Empty],
        ]);

        assert_eq!(lhs, rhs);

    }

    #[test]
    fn equal_upto_horizontal_shift() {
        use Tile::*;

        let lhs = CellShape::from_2darray([
            [Empty,  Filled],
            [Filled,  Empty],
        ]);

        let rhs = CellShape::from_2darray([
            [Empty, Empty, Empty,  Filled, Empty],
            [Empty, Empty, Filled,  Empty, Empty],
            [Empty, Empty, Empty,   Empty, Empty],
        ]);

        assert_eq!(lhs, rhs);

    }

    /*

    #[test]
    fn fill() {
        use Tile::*;

        let mut tg = CellShape::from_2darray([
            [Empty,  Empty,  Filled],
            [Empty,  Filled, Filled],
            [Filled,  Empty, Filled],
        ]);

        let coord = Coord { x: 0, y: 0 };
        assert_eq!(tg[coord], Empty);

        let already_filled = tg.fill(coord);

        assert!(!already_filled);
        assert_eq!(tg[coord], Filled);


        let already_filled = tg.fill(coord);
        assert!(already_filled);
        assert_eq!(tg[coord], Filled);
    }

    #[test]
    fn place_onto_empty() {
        use Tile::*;

        let mut empty = CellShape::from_2darray([
            [Empty,  Empty,  Empty],
            [Empty,  Empty,  Empty],
            [Empty,  Empty,  Empty],
        ]);

        let tg = CellShape::from_2darray([
            [Filled, Empty],
            [Filled, Empty],
        ]);


        let success = empty.try_join_at(&tg, Coord::origin());

        assert!(success);

        assert_eq!(empty[Coord { x: 0, y: 0 }], Filled);
        assert_eq!(empty[Coord { x: 1, y: 0 }], Empty);
        assert_eq!(empty[Coord { x: 2, y: 0 }], Empty);

        assert_eq!(empty[Coord { x: 0, y: 1 }], Filled);
        assert_eq!(empty[Coord { x: 1, y: 1 }], Empty);
        assert_eq!(empty[Coord { x: 2, y: 1 }], Empty);

        assert_eq!(empty[Coord { x: 0, y: 2 }], Empty);
        assert_eq!(empty[Coord { x: 1, y: 2 }], Empty);
        assert_eq!(empty[Coord { x: 2, y: 2 }], Empty);

    }


    #[test]
    fn place_onto_empty_non_overlapping() {
        use Tile::*;

        let mut empty = CellShape::from_2darray([
            [Empty,  Empty,  Empty],
            [Empty,  Empty,  Empty],
            [Empty,  Empty,  Empty],
        ]);

        let tg = CellShape::from_2darray([
            [Filled, Empty],
            [Filled, Empty],
        ]);


        let success = empty.try_join_at(&tg, Coord { x: 0, y: 2 });

        assert!(success);

        assert_eq!(empty[Coord { x: 0, y: 0 }], Empty);
        assert_eq!(empty[Coord { x: 1, y: 0 }], Empty);
        assert_eq!(empty[Coord { x: 2, y: 0 }], Empty);

        assert_eq!(empty[Coord { x: 0, y: 1 }], Empty);
        assert_eq!(empty[Coord { x: 1, y: 1 }], Empty);
        assert_eq!(empty[Coord { x: 2, y: 1 }], Empty);

        assert_eq!(empty[Coord { x: 0, y: 2 }], Filled);
        assert_eq!(empty[Coord { x: 1, y: 2 }], Empty);
        assert_eq!(empty[Coord { x: 2, y: 2 }], Empty);

        assert_eq!(empty[Coord { x: 0, y: 3 }], Filled);
        assert_eq!(empty[Coord { x: 1, y: 3 }], Empty);
        assert_eq!(empty[Coord { x: 2, y: 3 }], Empty);

        assert_eq!(empty[Coord { x: 0, y: 4 }], Empty);
        assert_eq!(empty[Coord { x: 1, y: 4 }], Empty);
        assert_eq!(empty[Coord { x: 2, y: 4 }], Empty);

    }


     #[test]
    fn place_onto_not_possible() {
        use Tile::*;

        let mut empty = CellShape::from_2darray([
            [Empty,  Empty,  Empty],
            [Filled, Filled,  Empty],
            [Empty,  Empty,  Empty],
        ]);

        let tg = CellShape::from_2darray([
            [Filled, Empty],
            [Filled, Empty],
        ]);


        let success = empty.try_join_at(&tg, Coord { x: 0, y: 0 });

        assert!(!success);

        // not modified at all
        assert_eq!(empty[Coord { x: 0, y: 0 }], Empty);
        assert_eq!(empty[Coord { x: 1, y: 0 }], Empty);
        assert_eq!(empty[Coord { x: 2, y: 0 }], Empty);

        assert_eq!(empty[Coord { x: 0, y: 1 }], Filled);
        assert_eq!(empty[Coord { x: 1, y: 1 }], Filled);
        assert_eq!(empty[Coord { x: 2, y: 1 }], Empty);

        assert_eq!(empty[Coord { x: 0, y: 2 }], Empty);
        assert_eq!(empty[Coord { x: 1, y: 2 }], Empty);
        assert_eq!(empty[Coord { x: 2, y: 2 }], Empty);
    }

    */
}
