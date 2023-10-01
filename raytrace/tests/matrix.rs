use std::f32::consts::PI;

use raytrace::*;

#[test]
#[rustfmt::skip]
pub fn test_matrix() {
    let data = vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.5, 6.5, 7.5, 8.5],
        vec![9.0, 10.0, 11.0, 12.0],
        vec![13.5, 14.5, 15.5, 16.5],
    ];
    let matrix = Matrix::from_vec(data);

    assert_eq!(matrix[0][3], 4.0);
    assert_eq!(matrix[1][0], 5.5);
    assert_eq!(matrix[3][2], 15.5);
}

#[test]
#[rustfmt::skip]
pub fn test_matrix_eq() {
    let data_1 = vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.5, 6.5, 7.5, 8.5],
        vec![9.0, 10.0, 11.0, 12.0],
        vec![13.5, 14.5, 15.5, 16.5],
    ];
    let data_2 = vec![
        vec![1.0, 3.0, 2.0, 4.0],
        vec![5.5, 6.5, 7.5, 8.5],
        vec![9.0, 15.5, 11.0, 12.0],
        vec![13.5, 14.5, 10.0, 16.5],
    ];

    let matrix_1 = Matrix::from_vec(data_1.clone());
    let matrix_2 = Matrix::from_vec(data_1);

    assert_eq!(matrix_1, matrix_2);

    let matrix_3 = Matrix::from_vec(data_2);

    assert!(matrix_1 != matrix_3)
}

#[test]
#[rustfmt::skip]
pub fn test_matrix_mul() {
    let data_1 = vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
        vec![9.0, 8.0, 7.0, 6.0],
        vec![5.0, 4.0, 3.0, 2.0],
    ];

    let data_2 = vec![
        vec![-2.0, 1.0, 2.0, 3.0],
        vec![3.0, 2.0, 1.0, -1.0],
        vec![4.0, 3.0, 6.0, 5.0],
        vec![1.0, 2.0, 7.0, 8.0],
    ];

    let matrix_1 = Matrix::from_vec(data_1);
    let matrix_2 = Matrix::from_vec(data_2);

    let result = vec![
        vec![20.0, 22.0, 50.0, 48.0],
        vec![44.0, 54.0, 114.0, 108.0],
        vec![40.0, 58.0, 110.0, 102.0],
        vec![16.0, 26.0, 46.0, 42.0],
    ];

    let result_m = Matrix::from_vec(result);

    assert_eq!(matrix_1 * matrix_2, result_m);
}

#[test]
#[rustfmt::skip]
pub fn test_matrix_transpose() {
    let data_1 = vec![
        vec![0.0, 9.0, 3.0, 0.0],
        vec![9.0, 8.0, 0.0, 8.0],
        vec![1.0, 8.0, 5.0, 3.0],
        vec![0.0, 0.0, 5.0, 8.0],
    ];

    let matrix_1 = Matrix::from_vec(data_1);

    let data_2 = vec![
        vec![0.0, 9.0, 1.0, 0.0],
        vec![9.0, 8.0, 8.0, 0.0],
        vec![3.0, 0.0, 5.0, 5.0],
        vec![0.0, 8.0, 3.0, 8.0],
    ];

    let matrix_2 = Matrix::from_vec(data_2);

    assert_eq!(matrix_1.transposed(), matrix_2);
}

#[test]
#[rustfmt::skip]
pub fn test_matrix_determinant() {
    let matrix_1 = Matrix::from_vec(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
    assert_eq!(matrix_1.get_determinant(), 17.0);

    let matrix_2 = Matrix::from_vec(vec![
        vec![1.0, 5.0, 0.0],
        vec![-3.0, 2.0, 7.0],
        vec![0.0, 6.0, -3.0],
        ]);

    let test_2 = Matrix::from_vec(vec![
        vec![-3.0, 2.0],
        vec![0.0, 6.0],
    ]);

    assert_eq!(matrix_2.get_submatrix(0, 2), test_2);

    let matrix_3 = Matrix::from_vec(vec![
        vec![3.0, 5.0, 0.0],
        vec![2.0, -1.0, -7.0],
        vec![6.0, -1.0, 5.0]
    ]);

    assert_eq!(matrix_3.minor(1, 0), 25.0);
    assert_eq!(matrix_3.cofactor(1, 0), -25.0);
    assert_eq!(matrix_3.minor(0, 0), -12.0);
    assert_eq!(matrix_3.cofactor(0, 0), -12.0);

    let matrix_4 = Matrix::from_vec(vec![
        vec![-2.0, -8.0, 3.0, 5.0],
        vec![-3.0, 1.0, 7.0, 3.0],
        vec![1.0, 2.0, -9.0, 6.0],
        vec![-6.0, 7.0, 7.0, -9.0],
    ]);

    assert_eq!(matrix_4.cofactor(0, 0), 690.0);
    assert_eq!(matrix_4.cofactor(0, 1), 447.0);
    assert_eq!(matrix_4.cofactor(0, 2), 210.0);
    assert_eq!(matrix_4.cofactor(0, 3), 51.0);
    assert_eq!(matrix_4.get_determinant(), -4071.0);
}

#[test]
#[rustfmt::skip]
pub fn test_matrix_invert() {
    let matrix = Matrix::from_vec(vec![
        vec![-5.0, 2.0, 6.0, -8.0],
        vec![1.0, -5.0, 1.0, 8.0],
        vec![7.0, 7.0, -6.0, -7.0],
        vec![1.0, -3.0, 7.0, 4.0]
    ]);

    let test = Matrix::from_vec(vec![
        vec![0.218805, 0.45113, 0.24060, -0.04511],
        vec![-0.80827, -1.45677, -0.44361, 0.52068],
        vec![-0.07895, -0.22368, -0.05263, 0.19737],
        vec![-0.52256, -0.81391, -0.30075, 0.30639],
    ]);

    assert!(matrix.invert().unwrap() == test);

    let matrix_2 = Matrix::from_vec(vec![
        vec![8.0, -5.0, 9.0, 2.0],
        vec![7.0, 5.0, 6.0, 1.0],
        vec![-6.0, 0.0, 9.0, 6.0],
        vec![-3.0, 0.0, -9.0, -4.0]
    ]);

    let test_2 = Matrix::from_vec(vec![
        vec![-0.15385, -0.15385, -0.28205, -0.53846],
        vec![-0.07692, 0.12308, 0.02564, 0.03077],
        vec![0.35897, 0.35897, 0.43590, 0.92308],
        vec![-0.69231, -0.69231, -0.76923, -1.92308],
    ]);

        assert!(matrix_2.invert().unwrap() == test_2);

}

#[test]
#[rustfmt::skip]
pub fn test_matrix_translation() {
    let point = Pos3::new(-3.0, 4.0, 5.0);
    let transform: Matrix = Matrix::translation(5.0, -3.0, 2.0);

    assert_eq!(transform.clone() * point, Pos3::new(2.0, 1.0, 7.0));
    assert_eq!(transform.invert().unwrap() * point, Pos3::new(-8.0, 7.0, 3.0))
}

#[test]
#[rustfmt::skip]
pub fn test_matrix_scaling() {
    let point = Pos3::new(-4.0, 6.0, 8.0);
    let scale = Matrix::scaling(2.0, 3.0, 4.0);

    assert_eq!(scale.clone() * point, Pos3::new(-8.0, 18.0, 32.0));

    let vec = Vec3::new(-4.0, 6.0, 8.0);
    assert_eq!(scale.clone() * vec, Vec3::new(-8.0, 18.0, 32.0));
    assert_eq!(scale.invert().unwrap() * vec, Vec3::new(-2.0, 2.0, 2.0));
}

#[test]
#[rustfmt::skip]
pub fn test_matrix_rotation() {
    let px = Pos3::new(0.0, 1.0, 0.0);
    let rot_x = Matrix::rotation_x(PI / 4.0);

    assert_eq!(rot_x * px, Pos3::new(0.0, 2.0f32.sqrt() / 2.0, 2.0f32.sqrt() / 2.0));

    let py = Pos3::new(0.0, 0.0, 1.0);
    let rot_y = Matrix::rotation_y(PI / 4.0);

    assert_eq!(rot_y * py, Pos3::new(2.0f32.sqrt() / 2.0, 0.0, 2.0f32.sqrt() / 2.0));

    let pz = Pos3::new(0.0, 1.0, 0.0);
    let rot_z = Matrix::rotation_z(PI / 4.0);

    assert_eq!(rot_z * pz, Pos3::new(-(2.0f32.sqrt()) / 2.0, 2.0f32.sqrt() / 2.0, 0.0));
}

#[test]
#[rustfmt::skip]
pub fn test_matrix_skew() {
    let point = Pos3::new(2.0, 3.0, 4.0);
    let skew_xy = Matrix::skew(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let skew_yz = Matrix::skew(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let skew_zx = Matrix::skew(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

    assert_eq!(skew_xy * point, Pos3::new(5.0, 3.0, 4.0));
    assert_eq!(skew_yz * point, Pos3::new(2.0, 7.0, 4.0));
    assert_eq!(skew_zx * point, Pos3::new(2.0, 3.0, 6.0));
}
