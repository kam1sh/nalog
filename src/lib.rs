use std::str::FromStr;

use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;
// use pyo3::exceptions::TypeError as PyTypeError;
use pyo3::exceptions::ValueError as PyValueError;
use pyo3::wrap_pyfunction;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum LogLevels {
    TRACE = 10,
    DEBUG = 20,
    INFO = 30,
    WARN = 40,
    ERROR = 50,
}

impl LogLevels {
    fn py_from_str(s: &str) -> PyResult<LogLevels> {
        match LogLevels::from_str(s) {
            Ok(v) => Ok(v),
            Err(_) => return Err(PyValueError::py_err(format!(
                "Error parsing log level '{}'",
                s
            )))
        }
    }
}

impl FromStr for LogLevels {
    type Err = ();
    fn from_str(s: &str) -> Result<LogLevels, ()> {
        let uppercase = s.to_lowercase();

        match uppercase.as_ref() {
            "trace" => Ok(LogLevels::TRACE),
            "debug" => Ok(LogLevels::DEBUG),
            "info" => Ok(LogLevels::INFO),
            "warn" => Ok(LogLevels::WARN),
            "error" => Ok(LogLevels::ERROR),
            _ => Err(()),
        }
    }
}

#[pyclass(module = "nalog")]
struct Logger {
    name: String,
    level: LogLevels,
}

#[pymethods]
impl Logger {
    #[new]
    fn new(obj: &PyRawObject, name: String, level: String) -> PyResult<()> {
        obj.init(Logger {
            name: name,
            level: LogLevels::py_from_str(&level)?,
        });
        Ok(())
    }

    fn log(&self, message: String, level: String) -> PyResult<()> {
        let lvl = LogLevels::py_from_str(&level)?;
        if lvl >= self.level {
            println!("[{}] {}", self.name, message)
        }
        Ok(())
    }
}

#[pyfunction]
fn get_level(level: String) -> PyResult<u8> {
    match LogLevels::py_from_str(&level) {
        Ok(v) => Ok(v as u8),
        Err(v) => return Err(v)
    }
}

#[pymodule]
fn nalog(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pyfunction!(get_level))?;
    m.add_class::<Logger>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_loglevels() {
        use super::*;
        assert_eq!(
            LogLevels::from_str("trace") > LogLevels::from_str("debug"),
            true
        );
    }
}
