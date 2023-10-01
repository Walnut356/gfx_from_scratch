use std::{iter::zip, slice::ChunksExact, cell::OnceCell};

use crate::{float_eq, Pos3, Vec3};

#[derive(Debug, Clone)]
pub struct Matrix {
    pub inner: Vec<Vec<f32>>,
    pub width: usize,
    pub height: usize,
}

impl Matrix {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            inner: vec![vec![0.0; cols]; rows],
            width: cols,
            height: rows,
        }
    }

    pub fn from_vec(data: Vec<Vec<f32>>) -> Self {
        let width = data[0].len();
        let height = data.len();
        Self {
            inner: data,
            width,
            height,
        }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self::from_vec(vec![
            vec![1.0, 0.0, 0.0, x],
            vec![0.0, 1.0, 0.0, y],
            vec![0.0, 0.0, 1.0, z],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        Self::from_vec(vec![
            vec![x, 0.0, 0.0, 0.0],
            vec![0.0, y, 0.0, 0.0],
            vec![0.0, 0.0, z, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_x(rads: f32) -> Self {
        let (sin, cos) = rads.sin_cos();
        Self::from_vec(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, cos, -sin, 0.0],
            vec![0.0, sin, cos, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_y(rads: f32) -> Self {
        let (sin, cos) = rads.sin_cos();
        Self::from_vec(vec![
            vec![cos, 0.0, sin, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![-sin, 0.0, cos, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_z(rads: f32) -> Self {
        let (s, c) = rads.sin_cos();
        Self::from_vec(vec![
            vec![c, -s, 0.0, 0.0],
            vec![s, c, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Stretches the object. The first letter of each named argument determines the axis to stretch
    /// and the second letter denotes which axis it stretches in proportion to.
    ///
    /// e.g. xy stretches the X axis in proportion to the Y axis
    pub fn skew(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        Self::from_vec(vec![
            vec![1.0, xy, xz, 0.0],
            vec![yx, 1.0, yz, 0.0],
            vec![zx, zy, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn get_rows(&self) -> &Vec<Vec<f32>> {
        &self.inner
    }

    pub fn get_cols(&self) -> Vec<Vec<f32>> {
        let mut result = vec![vec![0.0; self.height]; self.width];

        for i in 0..self.width {
            for j in 0..self.height {
                result[i][j] = self.inner[j][i]
            }
        }

        result
    }


    #[rustfmt::skip]
    pub fn get_identity(&self) -> Matrix {
        // hard code the few that we'll be using a lot of
        match (self.width, self.height) {
            (2, 2) => Matrix::from_vec(vec![
                vec![1.0, 0.0],
                vec![0.0, 1.0],
            ]),
            (3, 3) => Matrix::from_vec(vec![
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0],
            ]),
            (4, 4) => Matrix::from_vec(vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ]),
            (c, r) => panic!("shouldn't ever need to handle matricies larger than 4x4"),
        }
    }

    pub fn transposed(&self) -> Matrix {
        let mut result = Matrix::new(self.width, self.height);

        for i in 0..self.width {
            for j in 0..self.height {
                result[i][j] = self.inner[j][i]
            }
        }

        result
    }

    pub fn get_determinant(&self) -> f32 {
        match (self.height, self.width) {
            (2, 2) => (self.inner[0][0] * self.inner[1][1]) - (self.inner[0][1] * self.inner[1][0]),
            (r, c) => self[0]
                .iter()
                .enumerate()
                .map(|(i, x)| *x * self.cofactor(0, i))
                .sum(),
        }
    }

    pub fn get_submatrix(&self, row: usize, column: usize) -> Matrix {
        let mut temp = self.inner.clone();

        temp.remove(row);
        for c in temp.iter_mut() {
            c.remove(column);
        }

        Matrix::from_vec(temp)
    }

    pub fn minor(&self, row: usize, column: usize) -> f32 {
        self.get_submatrix(row, column).get_determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        let result = self.minor(row, column);
        if (row + column) % 2 == 0 {
            result
        } else {
            -result
        }
    }

    /// Returns an inverted matrix if an inversion is possible, otherwise returns None
    pub fn invert(&self) -> Option<Matrix> {
        let det = self.get_determinant();
        if det == 0.0 {
            return None;
        }

        let mut result = Matrix::new(self.width, self.height);
        for r in 0..self.height {
            for c in 0..self.width {
                let cof = self.cofactor(r, c);
                result[c][r] = cof / det;
            }
        }

        Some(result)
    }
}

impl From<Vec3> for Matrix {
    fn from(value: Vec3) -> Self {
        Self::from_vec(vec![vec![value.x], vec![value.y], vec![value.z], vec![1.0]])
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.width == other.width && self.height == other.height {
            for (a, b) in zip(self.inner.iter().flatten(), other.inner.iter().flatten()) {
                if !float_eq(*a, *b) {
                    return false;
                }
            }
            return true;
        }

        false
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;

    /// Multiplies 2 equally sized square matrices together
    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.width == rhs.height);

        let mut result = Matrix::new(rhs.width, self.height);

        for i in 0..self.height {
            for j in 0..rhs.width {
                let mut val = 0.0;
                for k in 0..self.width {
                    let temp1 = &self[i];
                    let a = temp1[k];
                    let temp2 = &rhs[k];
                    let b = temp2[j];
                    val += a * b;
                }

                result[i][j] = val;
            }
        }

        result
    }
}

impl std::ops::Mul<Matrix> for &Matrix {
    type Output = Matrix;

    /// Multiplies 2 equally sized square matrices together
    fn mul(self, rhs: Matrix) -> Self::Output {
        assert!(self.width == rhs.height);

        let mut result = Matrix::new(rhs.width, self.height);

        for i in 0..self.height {
            for j in 0..rhs.width {
                let mut val = 0.0;
                for k in 0..self.width {
                    let temp1 = &self[i];
                    let a = temp1[k];
                    let temp2 = &rhs[k];
                    let b = temp2[j];
                    val += a * b;
                }

                result[i][j] = val;
            }
        }

        result
    }
}

impl std::ops::Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.inner.len(),
            rhs.inner.len(),
            "matrices must be of equal size to add"
        );

        let mut result = Matrix::new(self.width, self.height);

        for ((a, b), r) in zip(
            zip(self.inner.iter().flatten(), rhs.inner.iter().flatten()),
            result.inner.iter_mut().flatten(),
        ) {
            *r = a + b;
        }

        result
    }
}

impl std::ops::Mul<Pos3> for Matrix {
    type Output = Pos3;

    fn mul(self, rhs: Pos3) -> Self::Output {
        let as_matrix = Matrix::from_vec(vec![vec![rhs.x], vec![rhs.y], vec![rhs.z], vec![1.0]]);

        let result = self * as_matrix;

        Pos3::new(result[0][0], result[1][0], result[2][0])
    }
}

impl std::ops::Mul<Vec3> for Matrix {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let as_matrix = Matrix::from_vec(vec![vec![rhs.x], vec![rhs.y], vec![rhs.z], vec![0.0]]);

        let result = self * as_matrix;

        Vec3::new(result[0][0], result[1][0], result[2][0])
    }
}

impl std::ops::Mul<Pos3> for &Matrix {
    type Output = Pos3;

    fn mul(self, rhs: Pos3) -> Self::Output {
        let as_matrix = Matrix::from_vec(vec![vec![rhs.x], vec![rhs.y], vec![rhs.z], vec![1.0]]);

        let result = self * as_matrix;

        Pos3::new(result[0][0], result[1][0], result[2][0])
    }
}

impl std::ops::Mul<Vec3> for &Matrix {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let as_matrix = Matrix::from_vec(vec![vec![rhs.x], vec![rhs.y], vec![rhs.z], vec![0.0]]);

        let result = self * as_matrix;

        Vec3::new(result[0][0], result[1][0], result[2][0])
    }
}
