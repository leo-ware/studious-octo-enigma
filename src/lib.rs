use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn hello_world() -> PyResult<String> {
    Ok("Hello World".to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn foo_project(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    Ok(())
}