use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyObjectProtocol};
use pyo3::{PyErr, Python};
use url::Url;

struct PyNetloc<'a> {
    username: &'a str,
    password: Option<&'a str>,
    domain: String,
    port_or_default: Option<u16>,
}


impl PyNetloc<'_> {
    fn to_py_str(&self) -> String {
        let mut user_pass = "".to_string();
        match self.username == "" && self.password != None {
            true => {
                user_pass = format!("{}:{}", self.username, self.password.unwrap())
                }
            false => {}
        }

        match self.port_or_default {
            None => {
                format!("{}@{}", user_pass, self.domain)
            }
            Some(p) => {
                format!("{}@{}:{}", user_pass, self.domain, p)
            }
        }
    }
}

#[pyclass]
struct PyrUrl {
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
    #[pyo3(get)]
    username: String,
    // #[pyo3(get)]
    // password: String,
    // #[pyo3(get)]
    // hostname: String,
    #[pyo3(get)]
    port: u16,
}

#[pyproto]
impl PyObjectProtocol for PyrUrl {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "PyUrl(scheme='{}', netloc='{}', path='{}', params='{}', query='{}', fragment='{}')",
            self.scheme, self.netloc, self.path, self.params, self.query, self.fragment
        ))
    }
}

#[pyfunction]
fn parse_url(input_url: &str) -> PyResult<PyrUrl> {
    let parsed = Url::parse(input_url);
    match parsed {
        Ok(url) => {
            Ok(PyrUrl {
                scheme: url.scheme().to_string(),
                netloc: PyNetloc {
                    domain: url.domain().unwrap_or("").to_string(),
                    username: url.username(),
                    password: url.password(),
                    port_or_default: url.port_or_known_default()
                }.to_py_str(),
                path: url.path().to_string(),
                params: "".to_string(),
                query: url.query().unwrap_or("").to_string(),
                fragment: url.fragment().unwrap_or("").to_string(),
                username: url.username().to_string(),
                // password: url.password().to_string(),
                // hostname: url.host_str().to_string(),
                port: url.port().unwrap_or(0)
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
fn pyrurl(_py: Python, m: &PyModule) -> PyResult<()> {
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
    use url::Url;
    use crate::PyNetloc;

    #[test]
    fn test_url() {
        let input = "http://NetLoc:80/path;parameters/path2;parameters2?query=argument#fragment";
        let url = Url::parse(input).unwrap();
        let p = PyNetloc {
                    domain: url.domain().unwrap_or("").to_string(),
                    username: url.username(),
                    password: url.password(),
                    port_or_default: url.port_or_known_default()
                }.to_py_str();
    }
}
