use nalgebra as na;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use num_complex::Complex;
use sprs::{CsMat, CsVec};
// use sprs::TriMat;


const IPF16_COEFFS: IpfCoeffs<f64, 8> = IpfCoeffs {
    THETA: [
        Complex::new(3.509103608414918, 8.436198985884374),
        Complex::new(5.948152268951177, 3.587457362018322),
        Complex::new(-5.264971343442647, 1.622022147316793),
        Complex::new(1.419375897185666, 1.092536348449672),
        Complex::new(6.416177699099435, 1.194122393370139),
        Complex::new(4.993174737717997, 5.996881713603942),
        Complex::new(-1.413928462488886, 1.349772569889275),
        Complex::new(-1.084391707869699, 1.927744616718165),
    ],
    ALPHA: [
        Complex::new(5.464930576870210, -3.797983575308356),
        Complex::new(9.045112476907548, -1.115537522430261),
        Complex::new(2.344818070467641, -4.228020157070496),
        Complex::new(9.453304067358312, -2.951294291446048),
        Complex::new(7.283792954673409, -1.205646080220011),
        Complex::new(3.648229059594851, -1.155509621409682),
        Complex::new(2.547321630156819, -2.639500283021502),
        Complex::new(2.394538338734709, -5.650522971778156),
    ],
    ALPHA0: 2.124853710495224E-16,
};

struct IpfCoeffs<T, const N: usize> {
    THETA: [Complex<T>; N],
    ALPHA: [Complex<T>; N],
    ALPHA0: T,
}

fn _cram<T: na::ComplexField, const N: usize>(
    a: CsMat<T>,
    n0: CsVec<T>,
    dt: T,
    coeffs: &IpfCoeffs<T, N>,
) -> CsVec<T> {
    let size = n0.dim();
    let dense = n0.to_dense().map(|x| Complex::new(x, T::zero()));
    let dim = dense.len();
    let data = dense.into_raw_vec();
    let indices: Vec<usize> = (0..dim).collect();
    let mut n1: CsVec<Complex<T>> = sprs::CsVec::new(dim, indices, data);
    // let mut n1: CsVec<Complex<T>> = n0.to_dense().map(|x| Complex::new(x, T::zero())).to_sparse();
    // let eye = TriMat::<Complex<T>>::new_diag(size, vec![Complex::new(T::one(), T::zero()); size]).to_csc();
    let eye = sprs::CsMat::eye(size).map(|x| Complex::new(x, T::zero()));
    // let mtx = (a * dt).map(|x| Complex::new(x, T::zero()));
    let mtx = a.scale(dt).map(|x| Complex::new(x, T::zero()));
    let theta = coeffs.THETA;
    let mut alpha = coeffs.ALPHA.iter();
    for theta in theta.iter() {
        let system = mtx.clone() - eye.clone().scale(*theta);
        let lu = system.clone().ilu();
        let res = lu.solve(&n1).unwrap();
        n1 = (res.scale(Complex::new(T::from_real(2.0), T::zero())) * alpha.next().unwrap()).to_dense().real().to_sparse();
    }
    n1.scale(coeffs.ALPHA0)
}

#[pyfunction]
fn cram_16(a: CsMat<f64>, n0: CsVec<f64>, dt: f64) -> PyResult<CsVec<f64>> {
    Ok(_cram(a, n0, dt, &IPF16_COEFFS))
}

#[pymodule]
fn pydepkit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cram_16, m)?)?;
    m.add("__depkit_version_tuple__", (1, 0, 0), "depkit version")?;
    Ok(())
}