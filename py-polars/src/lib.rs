#[macro_use]
extern crate polars;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::lazy::dsl::PyExpr;
use crate::{
    dataframe::PyDataFrame,
    lazy::{
        dataframe::{PyLazyFrame, PyLazyGroupBy},
        dsl,
    },
    series::PySeries,
};

pub mod apply;
pub mod arrow_interop;
pub mod conversion;
pub mod dataframe;
pub mod datatypes;
pub mod error;
pub mod file;
pub mod lazy;
pub mod npy;
pub mod prelude;
pub mod series;
pub mod utils;

use mimalloc::MiMalloc;
use std::iter::FromIterator;
use crate::utils::str_to_polarstype;
use polars::prelude::*;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[pyfunction]
fn col(name: &str) -> dsl::PyExpr {
    dsl::col(name)
}

#[pyfunction]
fn except_(name: &str) -> dsl::PyExpr {
    dsl::except(name)
}

#[pyfunction]
fn lit(value: &PyAny) -> dsl::PyExpr {
    dsl::lit(value)
}

#[pyfunction]
fn range(low: i64, high: i64, dtype: &PyAny) -> dsl::PyExpr {
    dsl::range(low, high, dtype)
}

#[pyfunction]
fn binary_expr(l: dsl::PyExpr, op: u8, r: dsl::PyExpr) -> dsl::PyExpr {
    dsl::binary_expr(l, op, r)
}

#[pyfunction]
fn binary_function(
    a: dsl::PyExpr,
    b: dsl::PyExpr,
    lambda: PyObject,
    output_type: &PyAny,
) -> dsl::PyExpr {
    dsl::binary_function(a, b, lambda, output_type)
}

#[pyfunction]
fn pearson_corr(a: dsl::PyExpr, b: dsl::PyExpr) -> dsl::PyExpr {
    polars::lazy::functions::pearson_corr(a.inner, b.inner).into()
}

#[pyfunction]
fn cov(a: dsl::PyExpr, b: dsl::PyExpr) -> dsl::PyExpr {
    polars::lazy::functions::cov(a.inner, b.inner).into()
}

#[pyfunction]
fn when(predicate: PyExpr) -> dsl::When {
    dsl::when(predicate)
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
#[pyfunction]
fn version() -> &'static str {
    VERSION
}

#[pyfunction]
fn toggle_string_cache(toggle: bool) {
    polars::toggle_string_cache(toggle)
}


#[pyfunction]
fn series_from_range(low: i64, high: i64, dtype: &PyAny) -> PySeries {
    let str_repr = dtype.str().unwrap().to_str().unwrap();
    let dtype = str_to_polarstype(str_repr);

    match dtype {
        DataType::UInt32 => Series::from_iter((low as u32)..(high as u32)).into(),
        DataType::Int32 => Series::from_iter((low as i32)..(high as i32)).into(),
        DataType::Int64 => Series::from_iter((low as i64)..(high as i64)).into(),
        _ => unimplemented!()
    }
}

#[pymodule]
fn polars(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySeries>().unwrap();
    m.add_class::<PyDataFrame>().unwrap();
    m.add_class::<PyLazyFrame>().unwrap();
    m.add_class::<PyLazyGroupBy>().unwrap();
    m.add_class::<dsl::PyExpr>().unwrap();
    m.add_wrapped(wrap_pyfunction!(col)).unwrap();
    m.add_wrapped(wrap_pyfunction!(lit)).unwrap();
    m.add_wrapped(wrap_pyfunction!(binary_expr)).unwrap();
    m.add_wrapped(wrap_pyfunction!(binary_function)).unwrap();
    m.add_wrapped(wrap_pyfunction!(pearson_corr)).unwrap();
    m.add_wrapped(wrap_pyfunction!(cov)).unwrap();
    m.add_wrapped(wrap_pyfunction!(when)).unwrap();
    m.add_wrapped(wrap_pyfunction!(version)).unwrap();
    m.add_wrapped(wrap_pyfunction!(toggle_string_cache))
        .unwrap();
    m.add_wrapped(wrap_pyfunction!(except_)).unwrap();
    m.add_wrapped(wrap_pyfunction!(range)).unwrap();
    m.add_wrapped(wrap_pyfunction!(series_from_range)).unwrap();
    Ok(())
}
