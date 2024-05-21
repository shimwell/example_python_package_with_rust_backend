use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use nalgebra::{DMatrix, DVector};
// use num_complex::Complex;
// use nalgebra::Matrix2;
// use nalgebra::Matrix;
use num_complex::Complex;
// use nalgebra::{Matrix, U1, U8, VecStorage};
// use num_complex::Complex;
use lazy_static::lazy_static;
// use nalgebra::DMatrix;
// use num_complex::Complex;

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
    // Convert the inputs to the appropriate types
    let a = DMatrix::from_row_slice(a.len(), a.len(), &a.concat());
    let n0 = DVector::from_vec(n0);

    // Call the Rust function
    let result = _cram(&a, &n0, dt);

    // Convert the result to a Python list and return it
    Ok(result.as_slice().to_vec())
}

fn _cram(a: &DMatrix<f64>, n0: &DVector<f64>, dt: f64) -> DVector<f64> {
    // Implement the _cram function here
    // This is a placeholder implementation
    a * n0 * dt
}

#[pymodule]
fn example_python_package_with_rust_backend(_py: Python, m: &PyModule) -> PyResult<()> {
    // Define your Python functions, classes, and variables here.
    m.add_function(wrap_pyfunction!(cram, m)?)?;
    Ok(())
}

// #[pymodule]
// fn pydepkit(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(cram, m)?)?;
//     Ok(())
// }