use std::collections::HashSet;

use crate::cell_shape::{Tile, CellShape};
use crate::transform::{Transform, RIGID_SYMMETRIES};

#[derive(Debug, Copy, Clone)]
pub enum Pentomino {
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

pub const PENTOMINOES: [Pentomino; 12] = [
    Pentomino::F,
    Pentomino::I,
    Pentomino::L,
    Pentomino::N,
    Pentomino::P,
    Pentomino::T,
    Pentomino::U,
    Pentomino::V,
    Pentomino::W,
    Pentomino::X,
    Pentomino::Y,
    Pentomino::Z,
];

impl Pentomino {
    /// Returns all possible orientations for this pentamino 
    pub fn shapes(self) -> HashSet<CellShape> {
        use Tile::{
            Empty as o,
            Filled as F,
        };
        // Get a representative shape for the pentamino
        let rep = match self {
            Pentomino::F => CellShape::from_2darray([
                [o, F, F],
                [F, F, o],
                [o, F, o],
            ]),
            Pentomino::I => CellShape::from_2darray([
                [F],
                [F],
                [F],
                [F],
                [F],
            ]),
            Pentomino::L => CellShape::from_2darray([
                [F, o],
                [F, o],
                [F, o],
                [F, F],
            ]),
            Pentomino::N => CellShape::from_2darray([
                [F, o],
                [F, F],
                [o, F],
                [o, F],
            ]),
            Pentomino::P => CellShape::from_2darray([
                [F, F],
                [F, F],
                [F, o],
            ]),
            Pentomino::T => CellShape::from_2darray([
                [F, F, F],
                [o, F, o],
                [o, F, o],
            ]),
            Pentomino::U => CellShape::from_2darray([
                [F, o, F],
                [F, F, F],
            ]),
            Pentomino::V => CellShape::from_2darray([
                [F, o, o],
                [F, o, o],
                [F, F, F],
            ]),
            Pentomino::W => CellShape::from_2darray([
                [F, o, o],
                [F, F, o],
                [o, F, F],
            ]),
            Pentomino::X => CellShape::from_2darray([
                [o, F, o],
                [F, F, F],
                [o, F, o],
            ]),
            Pentomino::Y => CellShape::from_2darray([
                [F, o],
                [F, F],
                [F, o],
                [F, o],
            ]),
            Pentomino::Z => CellShape::from_2darray([
                [F, F, o],
                [o, F, o],
                [o, F, F],
            ]),
        };
        create_all_orientations(rep, RIGID_SYMMETRIES.as_slice())
    }
}

// Creates all unique orientations of a given shape and transforms
fn create_all_orientations(rep: CellShape, symmetries: &[Transform]) -> HashSet<CellShape> {
    symmetries
        .iter()
        .map(|t| t.transform_shape(rep.clone()))
        .collect()
}
