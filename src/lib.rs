use pyo3::prelude::*;

#[pyfunction]
fn hello_world(int_arg: i32, float_arg: f32, str_arg: &str) -> PyResult<i32> {

    println!("This is a Rust function running from Python");
    println!("A few arguments are accepted");
    println!("An int_arg");
    println!("A float_arg");
    println!("A str_arg");
    println!("The passed in argument values are printed to the terminal");
    println!("float_arg {}", float_arg);
    println!("int_arg {}", int_arg);
    println!("str_arg {}", str_arg);
    println!("The returned value is the int_arg * int_arg");

    Ok(int_arg * int_arg)
}


#[pymodule]
fn example_python_package_with_rust_backend(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    Ok(())
}