use crate::coord::Coord;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Filled,
    Empty
}

#[derive(Debug, Clone)]
pub struct TileGrid {
    // easiest to just represent it as a list of filled tiles for now
    filled_tiles: Vec<Coord>,
}
impl TileGrid {
    pub fn from_2darray<const N: usize>(grid: [[Tile; N]; N]) -> TileGrid {
        let mut filled_tiles = Vec::new();
        for y in 0..N {
            for x in 0..N {
                let coord = Coord {
                    x: x as isize, 
                    y: y as isize
                };
                if grid[y][x] == Tile::Filled {
                    filled_tiles.push(coord);
                }
            }
        }
        TileGrid {
            filled_tiles
        }
    }
    /// Fill the requested position, returning true if the tile was already filled there
    pub fn fill(&mut self, coord: Coord) -> bool {
        if self[coord] == Tile::Filled {
            return true
        }
        self.filled_tiles.push(coord);
        false
    }


    /// Attempts to place the other grid onto this one, without overlapping any already filled cells.
    /// Returns true if we were successfully modified
    //TODO: confusing name
    pub fn try_place_over(&mut self, other: &TileGrid, offset: Coord) -> bool {

        // check that we can do the place over
        for other_coord in other.filled_tiles.iter() {
            let coord = *other_coord + offset;
            if self.filled_tiles.contains(&coord) {
                return false;
            }

        }

        // do it
        for other_coord in other.filled_tiles.iter() {
            let coord = *other_coord + offset;
            self.filled_tiles.push(coord);
        }

        true
    }

}

impl std::ops::Index<Coord> for TileGrid {
    type Output = Tile;

    fn index(&self, coord: Coord) -> &Self::Output {
        if self.filled_tiles.contains(&coord) {
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

        let tg = TileGrid::from_2darray([
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
    fn fill() {
        use Tile::*;

        let mut tg = TileGrid::from_2darray([
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

        let mut empty = TileGrid::from_2darray([
            [Empty,  Empty,  Empty],
            [Empty,  Empty,  Empty],
            [Empty,  Empty,  Empty],
        ]);

        let tg = TileGrid::from_2darray([
            [Filled, Empty],
            [Filled, Empty],
        ]);


        let success = empty.try_place_over(&tg, Coord::origin());

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

        let mut empty = TileGrid::from_2darray([
            [Empty,  Empty,  Empty],
            [Empty,  Empty,  Empty],
            [Empty,  Empty,  Empty],
        ]);

        let tg = TileGrid::from_2darray([
            [Filled, Empty],
            [Filled, Empty],
        ]);


        let success = empty.try_place_over(&tg, Coord { x: 0, y: 2 });

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

        let mut empty = TileGrid::from_2darray([
            [Empty,  Empty,  Empty],
            [Filled, Filled,  Empty],
            [Empty,  Empty,  Empty],
        ]);

        let tg = TileGrid::from_2darray([
            [Filled, Empty],
            [Filled, Empty],
        ]);


        let success = empty.try_place_over(&tg, Coord { x: 0, y: 0 });

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
}
