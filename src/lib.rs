use pyo3::exceptions::{PyTypeError};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::{PyErr, Python};
use url::Url;

#[pyclass]
struct MyStruct {
    #[pyo3(get)]
    scheme: String,
    #[pyo3(get)]
    netloc: String,
    #[pyo3(get)]
    path: String,
    #[pyo3(get)]
    params: String,
    #[pyo3(get)]
    query: String,
    #[pyo3(get)]
    fragment: String,
}
#[pyfunction]
fn parse_url(input_url: &str) -> PyResult<MyStruct> {
    let parsed = Url::parse(input_url);
    match parsed {
        Ok(url) => {
            Ok(MyStruct {
                scheme: url.scheme().to_string(),
                netloc: url.domain().unwrap_or("").to_string(),
                path: url.path().to_string(),
                params: "".to_string(),
                query: url.query().unwrap_or("").to_string(),
                fragment: url.fragment().unwrap_or("").to_string(),
            })
            // Ok(String::from(url.as_str()))
        }
        Err(e) => {
            let error_message = format!("Error parsing! {}", e.to_string());
            let gil = Python::acquire_gil();
            let py = gil.python();
            PyTypeError::new_err("Error").restore(py);
            assert!(PyErr::occurred(py));
            drop(PyErr::fetch(py));
            return Err(PyErr::fetch(py));
        }
    }
}

#[pymodule]
fn pyrurl(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_url, m)?)?;

    Ok(())
}

// impl std::convert::From<ParseError> for PyErr {
//     fn from(err: ParseError) -> Self {
//         PyOSError::new_err(err.to_string())
//     }
// }
//
// #[pyfunction]
// fn parse_url2(input_url: &str) -> Result<String, PyErr> {
//     Ok(Url::parse(input_url)?.to_string())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        let a = parse_url("test");
    }
}
