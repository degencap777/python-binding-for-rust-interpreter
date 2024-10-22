use pyo3::prelude::*;
use test_boson_core::operators::Operators;

#[pymodule]
fn test_boson_pyo3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[pyfn(m)]
    fn add(a: usize, b: usize) -> usize {
        Operators::add(a, b)
    }
    Ok(())
}