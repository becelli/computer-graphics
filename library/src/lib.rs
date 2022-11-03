use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod common;
mod operations;
use common::{Image, Point, Rgba};

// #[pyfunction]
// fn draw_line(image: Image) -> PyResult<Image> {
//     Ok(operations::draw_line(image))
// }
#[pyfunction]
fn draw_line(image: Image, p0: Point, p1: Point, color: Rgba) -> PyResult<Image> {
    Ok(operations::draw_line(image, p0, p1, color))
}

#[pyfunction]
fn draw_line_bresenham(image: Image, p0: Point, p1: Point, color: Rgba) -> PyResult<Image> {
    Ok(operations::draw_line_bresenham(image, p0, p1, color))
}

#[pyfunction]
fn draw_circle(image: Image, p0: Point, p1: Point, color: Rgba) -> PyResult<Image> {
    Ok(operations::draw_circle(image, p0, p1, color))
}

#[pyfunction]
fn draw_circle_bresenham(image: Image, p0: Point, p1: Point, color: Rgba) -> PyResult<Image> {
    Ok(operations::draw_circle_bresenham(image, p0, p1, color))
}

#[pyfunction]
fn draw_circle_parametric(image: Image, p0: Point, p1: Point, color: Rgba) -> PyResult<Image> {
    Ok(operations::draw_circle_parametric(image, p0, p1, color))
}

#[pyfunction]
fn draw_triangle(image: Image, p0: Point, p1: Point, p2: Point, color: Rgba) -> PyResult<Image> {
    Ok(operations::draw_triangle(image, p0, p1, p2, color))
}

#[pyfunction]
fn flood_fill(image: Image, p0: Point, color: Rgba) -> PyResult<Image> {
    Ok(operations::flood_fill(image, p0, color))
}

#[pymodule]
fn cglib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(draw_line, m)?)?;
    m.add_function(wrap_pyfunction!(draw_line_bresenham, m)?)?;
    m.add_function(wrap_pyfunction!(draw_circle, m)?)?;
    m.add_function(wrap_pyfunction!(draw_circle_bresenham, m)?)?;
    m.add_function(wrap_pyfunction!(draw_circle_parametric, m)?)?;
    m.add_function(wrap_pyfunction!(draw_triangle, m)?)?;
    m.add_function(wrap_pyfunction!(flood_fill, m)?)?;
    Ok(())
}
