use crate::cell_shape::{Tile, CellShape};
use crate::transform::Transform;

#[derive(Debug, Copy, Clone)]
pub enum Pentamino {
    F,
    I,
    L,
    N,
    P,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Pentamino {
    /// Returns a representative tile grid for this pentamino. 
    pub fn tile_grid(self) -> CellShape {
        use Tile::{
            Empty as o,
            Filled as F,
        };
        match self {
            Pentamino::F => CellShape::from_2darray([
                [o, F, F],
                [F, F, o],
                [o, F, o],
            ]),
            Pentamino::I => CellShape::from_2darray([
                [F],
                [F],
                [F],
                [F],
                [F],
            ]),
            Pentamino::L => CellShape::from_2darray([
                [F, o],
                [F, o],
                [F, o],
                [F, F],
            ]),
            Pentamino::N => CellShape::from_2darray([
                [F, o],
                [F, F],
                [o, F],
                [o, F],
            ]),
            Pentamino::P => CellShape::from_2darray([
                [F, F],
                [F, F],
                [F, o],
            ]),
            Pentamino::T => CellShape::from_2darray([
                [F, F, F],
                [o, F, o],
                [o, F, o],
            ]),
            Pentamino::U => CellShape::from_2darray([
                [F, o, F],
                [F, F, F],
            ]),
            Pentamino::V => CellShape::from_2darray([
                [F, o, o],
                [F, o, o],
                [F, F, F],
            ]),
            Pentamino::W => CellShape::from_2darray([
                [F, o, o],
                [F, F, o],
                [o, F, F],
            ]),
            Pentamino::X => CellShape::from_2darray([
                [o, F, o],
                [F, F, F],
                [o, F, o],
            ]),
            Pentamino::Y => CellShape::from_2darray([
                [F, o],
                [F, F],
                [F, o],
                [F, o],
            ]),
            Pentamino::Z => CellShape::from_2darray([
                [F, F, o],
                [o, F, o],
                [o, F, F],
            ]),
        }
    }



}
