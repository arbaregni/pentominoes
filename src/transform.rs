use crate::{cell_shape::CellShape, coord::Coord};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Transform {
    // A 3x3 square matrix
    elems: [[isize; 3]; 3]
}

// There are 8 rigid symmetries (Dihedral group with 8 elements)
pub const RIGID_SYMMETRIES: [Transform; 8] = [
    Transform::identity(),
    Transform::mirror_horizontal(),
    Transform::mirror_vertical(),
    Transform::mirror_diagonal(),
    Transform::mirror_diagonal2(),
    Transform::rotate90(),
    Transform::rotate180(),
    Transform::rotate270(),
];

impl Transform {
    /// Preserves original tile grid
    pub const fn identity() -> Transform {
        Transform { elems: [
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ]}
    }
    /// Applies a reflection over the X axis
    pub const fn mirror_horizontal() -> Transform {
        Transform { elems: [
            [-1, 0, 0],
            [0,  1, 0],
            [0,  0, 1],
        ]}
    }

    /// Applies a reflection over the Y axis
    pub const fn mirror_vertical() -> Transform {
        //              [ _ b _ ]
        // [ 0 a ] ---> [ _ 0 a ]
        // [ b _ ]      [ _ _ _ ]
        Transform { elems: [
            [1,  0, 0],
            [0, -1, 0],
            [0,  0, 1],
        ] }
    }

    // Applies a reflection over the line Y=X
    pub const fn mirror_diagonal() -> Transform {
        //              [ _ a _ ]
        // [ 0 a ] ---> [ b 0 _ ]
        // [ b _ ]      [ _ _ _ ]
        Transform { elems: [
            [ 0, -1, 0],
            [-1,  0, 0],
            [ 0,  0, 1],
        ]}
    }

    // Applies a reflection over the line Y=-X
    pub const fn mirror_diagonal2() -> Transform {
        //              [ _ _ _ ]
        // [ 0 a ] ---> [ _ 0 b ]
        // [ b _ ]      [ _ a _ ]
        Transform { elems: [
            [0, 1, 0],
            [1, 0, 0],
            [0, 0, 1],
        ] }
    }

    /// Applies one quarter turn, 90 degree counter clockwise about the origin
    pub const fn rotate90() -> Transform {
        // Keep in mind that the positive y axis points downwards, so 
        //              [ a _ ]
        // [ _ a ] ---> [ _ b ]
        // [ b _ ]      [ _ _ ]
        Transform { elems: [
            [ 0, 1, 0],
            [-1, 0, 0],
            [ 0, 0, 1],
        ]}
    }
    /// Applies one half turn, 180 degree counter clockwise about the origin
    pub const fn rotate180() -> Transform {
        Transform { elems: [
            [-1,  0, 0],
            [ 0, -1, 0],
            [ 0,  0, 1],
        ]}
    }
    /// Applies a three quarters turn, 270 degree counter clockwise about the origin
    pub const fn rotate270() -> Transform {
        // Keep in mind the y axis points downwards
        Transform { elems: [
            [ 0, -1, 0],
            [ 1,  0, 0],
            [ 0,  0, 1],
        ]}
    }


    /// Apply the transform to a coordinate point
    pub const fn transform_coord(self, coord: Coord) -> Coord {
        let coord = Coord {
            x: self.elems[0][0] * coord.x + self.elems[0][1] * coord.y + self.elems[0][2],
            y: self.elems[1][0] * coord.x + self.elems[1][1] * coord.y + self.elems[1][2]
        };
        // TODO: the homogenous coordinate :*
        coord
    }

    pub fn transform_shape(self, cell_shape: CellShape) -> CellShape {
        let coords = cell_shape.filled_tiles()
            .map(|c| self.transform_coord(c))
            .collect();
        CellShape::from_coordinate_list(coords)
    }


    const fn at(self, i: usize, j: usize) -> isize {
        self.elems[j][i]
    }
    
    const fn compose(self, rhs: Self) -> Transform {

        const fn dot(lhs: Transform, rhs: Transform, i: usize, j: usize) -> isize {
              lhs.at(i, 0) * rhs.at(0, j)
            + lhs.at(i, 1) * rhs.at(1, j)
            + lhs.at(i, 2) * rhs.at(2, j)
        }

        Transform { elems: [
            [dot(self, rhs, 0, 0), dot(self, rhs, 1, 0), dot(self, rhs, 2, 0)],
            [dot(self, rhs, 0, 1), dot(self, rhs, 1, 1), dot(self, rhs, 2, 1)],
            [dot(self, rhs, 0, 2), dot(self, rhs, 1, 2), dot(self, rhs, 2, 2)],
        ]}


    }
}


impl std::ops::Mul for Transform {
    type Output = Transform;

    fn mul(self, rhs: Self) -> Self::Output {
        self.compose(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_transform_coord() {
        let t = Transform::identity();

        assert_eq!(t.transform_coord(Coord::new(-2, 3)),  Coord::new(-2, 3));
        assert_eq!(t.transform_coord(Coord::new(-10, 1)), Coord::new(-10, 1));
        assert_eq!(t.transform_coord(Coord::new(7, 0)),   Coord::new(7, 0));
        assert_eq!(t.transform_coord(Coord::new(0, 6)),   Coord::new(0, 6));
    }

    #[test]
    fn identity_transform_shape() {
        use crate::cell_shape::Tile::{
            Empty as e,
            Filled as F
        };
        let t = Transform::identity();

        let before = CellShape::from_2darray([
            [e, F, e],
            [e, F, e],
            [e, F, F],
            [F, F, e],
        ]);

        let after = before.clone(); // in this case

        assert_eq!(t.transform_shape(before), after);

    }


    // ====================
    //   REFLECTIONS
    // ====================

    #[test]
    fn mirror_horizontal_transform_coord() {
        let t = Transform::mirror_horizontal();

        assert_eq!(t.transform_coord(Coord::new(0, 0)), Coord::new( 0, 0));
        assert_eq!(t.transform_coord(Coord::new(1, 0)), Coord::new(-1, 0));
        assert_eq!(t.transform_coord(Coord::new(0, 1)), Coord::new( 0, 1));

        assert_eq!(t.transform_coord(Coord::new(-2, 3)), Coord::new(2, 3));
        assert_eq!(t.transform_coord(Coord::new(-10, 1)), Coord::new(10, 1));
        assert_eq!(t.transform_coord(Coord::new(7, 0)), Coord::new(-7, 0));
        assert_eq!(t.transform_coord(Coord::new(0, 6)), Coord::new(0, 6));
    }


    #[test]
    fn mirror_horizontal_transform_shape() {
        use crate::cell_shape::Tile::{
            Empty as e,
            Filled as F
        };
        let t = Transform::mirror_horizontal();

        let before = CellShape::from_2darray([
            [e, F, e],
            [e, F, e],
            [e, F, F],
            [F, F, e],
        ]);

        let after = CellShape::from_2darray([
            [e, F, e],
            [e, F, e],
            [F, F, e],
            [e, F, F],
        ]);

        assert_eq!(t.transform_shape(before), after);
    }

    #[test]
    fn mirror_vertical_transform_coord() {
        let t = Transform::mirror_vertical();

        assert_eq!(t.transform_coord(Coord::new(0, 0)), Coord::new( 0, 0));
        assert_eq!(t.transform_coord(Coord::new(1, 0)), Coord::new( 1, 0));
        assert_eq!(t.transform_coord(Coord::new(0, 1)), Coord::new( 0, -1));

        assert_eq!(t.transform_coord(Coord::new(-2, 3)),  Coord::new(-2, -3));
        assert_eq!(t.transform_coord(Coord::new(-10, 1)), Coord::new(-10, -1));
        assert_eq!(t.transform_coord(Coord::new(7, 0)),   Coord::new(7, 0));
        assert_eq!(t.transform_coord(Coord::new(0, 6)),   Coord::new(0, -6));
    }


    #[test]
    fn mirror_vertical_transform_shape() {
        use crate::cell_shape::Tile::{
            Empty as e,
            Filled as F
        };
        let t = Transform::mirror_vertical();

        let before = CellShape::from_2darray([
            [e, F, e],
            [e, F, e],
            [e, F, F],
            [F, F, e],
        ]);

        let after = CellShape::from_2darray([
            [F, F, e],
            [e, F, F],
            [e, F, e],
            [e, F, e],
        ]);

        assert_eq!(t.transform_shape(before), after);
    }

    #[test]
    fn mirror_diagonal_transform_coord() {
        let t = Transform::mirror_diagonal();

        assert_eq!(t.transform_coord(Coord::new(0, 0)), Coord::new(0, 0));
        assert_eq!(t.transform_coord(Coord::new(1, 0)), Coord::new(0, -1));
        assert_eq!(t.transform_coord(Coord::new(0, 1)), Coord::new(-1, 0));
    }

    #[test]
    fn mirror_diagonal_transform_shape() {
        use crate::cell_shape::Tile::{
            Empty as e,
            Filled as F
        };
        let t = Transform::mirror_diagonal();

        let before = CellShape::from_2darray([
            [e, F, e],
            [e, F, e],
            [e, F, F],
            [F, F, e],
        ]);

        let after = CellShape::from_2darray([
            [e, F, e, e],
            [F, F, F, F],
            [F, e, e, e],
        ]);

        assert_eq!(t.transform_shape(before), after);
    }

    #[test]
    fn mirror_diagonal2_transform_coord() {
        let t = Transform::mirror_diagonal2();

        assert_eq!(t.transform_coord(Coord::new(0, 0)), Coord::new(0, 0));
        assert_eq!(t.transform_coord(Coord::new(1, 0)), Coord::new(0, 1));
        assert_eq!(t.transform_coord(Coord::new(0, 1)), Coord::new(1, 0));
    }

    #[test]
    fn mirror_diagonal2_transform_shape() {
        use crate::cell_shape::Tile::{
            Empty as e,
            Filled as F
        };
        let t = Transform::mirror_diagonal2();

        let before = CellShape::from_2darray([
            [e, F, e],
            [e, F, e],
            [e, F, F],
            [F, F, e],
        ]);

        let after = CellShape::from_2darray([
            [e, e, e, F],
            [F, F, F, F],
            [e, e, F, e],
        ]);

        assert_eq!(t.transform_shape(before), after);
    }

    // ======================
    //    ROTATIONS
    // ======================

    #[test]
    fn rotate90_transform_coord() {
        let t = Transform::rotate90();

        assert_eq!(t.transform_coord(Coord::new(0, 0)), Coord::new(0, 0));
        assert_eq!(t.transform_coord(Coord::new(0, 1)), Coord::new(1, 0));
        assert_eq!(t.transform_coord(Coord::new(1, 0)), Coord::new(0, -1));

    }

    #[test]
    fn rotate90_transform_shape() {
        use crate::cell_shape::Tile::{
            Empty as e,
            Filled as F
        };
        let t = Transform::rotate90();

        let before = CellShape::from_2darray([
            [e, F, e],
            [e, F, e],
            [e, F, F],
            [F, F, e],
        ]);

        let after = CellShape::from_2darray([
            [e, e, F, e],
            [F, F, F, F],
            [e, e, e, F],
        ]);

        assert_eq!(t.transform_shape(before), after);
    }


    #[test]
    fn rotate180_transform_coord() {
        let t = Transform::rotate180();

        assert_eq!(t.transform_coord(Coord::new(0, 0)), Coord::new(0, 0));
        assert_eq!(t.transform_coord(Coord::new(1, 0)), Coord::new(-1, 0));
        assert_eq!(t.transform_coord(Coord::new(0, 1)), Coord::new(0, -1));

        assert_eq!(t.transform_coord(Coord::new(-2, 3)), Coord::new(2, -3));
        assert_eq!(t.transform_coord(Coord::new(-10, 1)), Coord::new(10, -1));
        assert_eq!(t.transform_coord(Coord::new(7, 0)), Coord::new(-7, 0));
        assert_eq!(t.transform_coord(Coord::new(0, 6)), Coord::new(0, -6));
    }


    #[test]
    fn rotate180_transform_shape() {
        use crate::cell_shape::Tile::{
            Empty as e,
            Filled as F
        };
        let t = Transform::rotate180();

        let before = CellShape::from_2darray([
            [e, F, e],
            [e, F, e],
            [e, F, F],
            [F, F, e],
        ]);

        let after = CellShape::from_2darray([
            [e, F, F],
            [F, F, e],
            [e, F, e],
            [e, F, e],
        ]);

        assert_eq!(t.transform_shape(before), after);
    }


    #[test]
    fn rotate270_transform_coord() {
        let t = Transform::rotate270();

        assert_eq!(t.transform_coord(Coord::new(0, 0)), Coord::new( 0, 0));
        assert_eq!(t.transform_coord(Coord::new(1, 0)), Coord::new( 0, 1));
        assert_eq!(t.transform_coord(Coord::new(0, 1)), Coord::new(-1, 0));

    }

    #[test]
    fn rotate270_transform_shape() {
        use crate::cell_shape::Tile::{
            Empty as e,
            Filled as F
        };
        let t = Transform::rotate270();

        let before = CellShape::from_2darray([
            [e, F, e],
            [e, F, e],
            [e, F, F],
            [F, F, e],
        ]);

        let after = CellShape::from_2darray([
            [F, e, e, e],
            [F, F, F, F],
            [e, F, e, e],
        ]);

        assert_eq!(t.transform_shape(before), after);
    }

    // ==============================
    //     group relationships
    // ==============================

    const R: Transform = Transform::rotate90();
    const H: Transform = Transform::mirror_horizontal();

    #[test]
    fn mirror_vertical_is_rrh() {
        assert_eq!(Transform::mirror_vertical(), R * R * H)
    }
    #[test]
    fn mirror_diagonal_is_rh() {
        assert_eq!(Transform::mirror_diagonal(), R * H)
    }
    #[test]
    fn mirror_diagonal2_is_rrrh() {
        assert_eq!(Transform::mirror_diagonal2(), R * R * R * H)
    }
    #[test]
    fn rotate180_is_rr() {
        assert_eq!(Transform::rotate180(), R * R);
    }
    #[test]
    fn rotate270_is_rrr() {
        assert_eq!(Transform::rotate270(), R * R * R);
    }
    #[test]
    fn identity_is_rrrr() {
        assert_eq!(Transform::identity(), R * R * R * R);
    }
    #[test]
    fn identity_is_hh() {
        assert_eq!(Transform::identity(), H * H);
    }
}
