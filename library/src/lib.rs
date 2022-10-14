use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod common;
mod operations;
use common::{Image, Point, Rgba};

#[pyfunction]
fn draw_line(image: Image, p0: Point, p1: Point, color: Rgba) -> PyResult<Image> {
    Ok(operations::draw_line(image, p0, p1, color))
}

#[pyfunction]
fn bresenham_line_algorithm(image: Image, p0: Point, p1: Point, color: Rgba) -> PyResult<Image> {
    Ok(operations:: bresenham_line_algorithm(image, p0, p1, color))
}

#[pymodule]
fn libkayn(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(draw_line, m)?)?;
    Ok(())
}

