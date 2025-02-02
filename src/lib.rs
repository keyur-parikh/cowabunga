use pyo3::{prelude::*, wrap_pymodule};

mod constants;
mod bufferpool;
mod table;
mod helpers;
mod errors;

#[pymodule]
fn buffer_pool_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<bufferpool::BufferPool>()?;
    Ok(())
}

#[pymodule]
fn table_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<table::Table>()?;
    Ok(())
}

#[pymodule]
fn record_type_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<table::PyRecord>()?;
    Ok(())
}

#[pymodule]
fn cowabunga_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(table_module))?;
    m.add_wrapped(wrap_pymodule!(buffer_pool_module))?;
    m.add_wrapped(wrap_pymodule!(record_type_module))?;
    Ok(())
}
