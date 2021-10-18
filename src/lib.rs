use pyo3::prelude::{pyclass, pymethods, pymodule, pyproto, PyModule, PyResult, Python};
use pyo3::{PyAny, PyNativeType, PyNumberProtocol, PyObject};

#[pyclass()]
#[derive(Clone)]
struct A;

#[pymethods]
impl A {
    #[new]
    fn new() -> A {
        A {}
    }
}

#[pyproto]
impl PyNumberProtocol for A {
    fn __mod__(lhs: Self, other: &PyAny) -> PyResult<PyObject> {
        println!("A.__mod__ is called");
        Ok(other.py().NotImplemented())
    }

    fn __rmod__(&self, other: &PyAny) -> PyResult<PyObject> {
        println!("A.__rmod__ is called");
        Ok(other.py().NotImplemented())
    }
}

#[pyclass()]
#[derive(Clone)]
struct B;

#[pymethods]
impl B {
    #[new]
    fn new() -> B {
        B {}
    }
}

#[pyproto]
impl PyNumberProtocol for B {
    fn __mod__(lhs: Self, other: &PyAny) -> PyResult<PyObject> {
        println!("B.__mod__ is called");
        Ok(other.py().NotImplemented())
    }

    fn __rmod__(&self, other: &PyAny) -> PyResult<PyObject> {
        println!("B.__rmod__ is called");
        Ok(other.py().NotImplemented())
    }
}

#[pymodule]
fn pyo3_missing_rmod(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<A>()?;
    module.add_class::<B>()?;
    Ok(())
}
