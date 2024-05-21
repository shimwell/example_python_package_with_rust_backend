use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use nalgebra::{DMatrix};
use num_complex::Complex;
use lazy_static::lazy_static;
use ndarray::Array1;
use sprs::{CsMat, CsVec};
use std::iter::FromIterator;
// use nalgebra::base::Matrix;
use nalgebra::linalg::LU;
use nalgebra::DVector;

pub struct IPF16Coeffs {
    pub coeffs1: DMatrix<Complex<f64>>,
    pub coeffs2: DMatrix<Complex<f64>>,
    pub value: f64,
}

lazy_static! {
    pub static ref IPF16_COEFFS: IPF16Coeffs = {
        let coeffs1 = DMatrix::from_row_slice(1, 8, &[
            Complex::new(3.509103608414918E0, 8.436198985884374E0),
            Complex::new(5.948152268951177E0, 3.587457362018322E0),
            Complex::new(-5.264971343442647E0, 1.622022147316793E1),
            Complex::new(1.419375897185666E0, 1.092536348449672E1),
            Complex::new(6.416177699099435E0, 1.194122393370139E0),
            Complex::new(4.993174737717997E0, 5.996881713603942E0),
            Complex::new(-1.413928462488886E0, 1.349772569889275E1),
            Complex::new(-1.084391707869699E1, 1.927744616718165E1),
        ]);
        let coeffs2 = DMatrix::from_row_slice(1, 8, &[
            Complex::new(5.464930576870210E3, -3.797983575308356E4),
            Complex::new(9.045112476907548E1, -1.115537522430261E3),
            Complex::new(2.344818070467641E2, -4.228020157070496E2),
            Complex::new(9.453304067358312E1, -2.951294291446048E2),
            Complex::new(7.283792954673409E2, -1.205646080220011E5),
            Complex::new(3.648229059594851E1, -1.155509621409682E2),
            Complex::new(2.547321630156819E1, -2.639500283021502E1),
            Complex::new(2.394538338734709E1, -5.650522971778156E0),
        ]);
        IPF16Coeffs {
            coeffs1,
            coeffs2,
            value: 2.124853710495224E-16,
        }
    };
}

#[pyfunction]
fn cram(a: Vec<Vec<f64>>, n0: Vec<f64>, dt: f64) -> PyResult<Vec<f64>> {
    let size = n0.len();
    let mut n1: Array1<Complex<f64>> = Array1::from_iter(n0.iter().map(|&x| Complex::new(x, 0.0)));

    let eye: CsMat<Complex<f64>> = CsMat::eye(size);

    let rows = a.len();
    let cols = a[0].len();
    let mut data: Vec<Complex<f64>> = Vec::new();
    let mut row_indices: Vec<usize> = Vec::new();
    let mut col_ptrs: Vec<usize> = vec![0];
    
    for (j, col) in a.iter().enumerate() {
        for (i, &value) in col.iter().enumerate() {
            data.push(Complex::new(value * dt, 0.0));
            row_indices.push(i);
        }
        col_ptrs.push(data.len());
    }
    
    let mtx = CsMat::new_csc((rows, cols), col_ptrs, row_indices, data.clone());
    // let mtx = CsMat::from_iterator(size, size, a.iter().flat_map(|row| row.iter()).map(|&v| Complex::new(v * dt, 0.0)));

    // let solver = sprs::direct::SparseLU::new(mtx.view());
    let dense_mtx = DMatrix::from_iterator(rows, cols, data.clone());
    let solver = LU::new(dense_mtx);

    for (theta, alpha) in IPF16_COEFFS.coeffs1.iter().cloned().zip(IPF16_COEFFS.coeffs2.iter().cloned()) {
    // for (theta, alpha) in IPF16_COEFFS.coeffs1.iter().zip(&IPF16_COEFFS.coeffs2.iter()) {
        let theta = Complex::new(theta.re, 0.0);
        let system = &mtx - &eye.map(|&x| x * theta);
        let n1_vec = DVector::from_iterator(n1.len(), n1.iter().cloned());
        let res = solver.solve(&n1_vec).unwrap();
        let res_arr = Array1::from_shape_vec(res.shape().0, res.iter().cloned().collect()).unwrap();
        n1 = n1 + res_arr.map(|x| x * Complex::new(alpha.re * 2.0, 0.0));
    }

    Ok(n1.mapv(|x| x.re * IPF16_COEFFS.value).to_vec())
}

#[pymodule]
fn example_python_package_with_rust_backend(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cram, m)?)?;
    Ok(())
}