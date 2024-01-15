use pyo3::prelude::*;

#[pyfunction]
fn add(left: usize, right: usize) -> usize {
    left + right
}

#[pyfunction]
fn _merge_splits(splits: Vec<(&str, bool)>, chunk_size: usize) -> Vec<String> {
    println!("{:?}", splits);
    println!("{:?}", chunk_size);
    vec![]
}

#[pymodule]
#[pyo3(name = "_rust")]
fn module_extension(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(_merge_splits, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
