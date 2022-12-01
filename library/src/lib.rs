use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod common;
mod operations;
use common::{Image, Point, Rgba, Edge, HomogeneousEdge};

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

#[pyfunction]
fn select_area(image: Image, p0: Point, p1: Point) -> PyResult<Image> {
    Ok(operations::select_area(image, p0, p1))
}

#[pyfunction]
fn cohen_sutherland(image: Image, p0: Point, p1: Point, color: Rgba, boundary: Edge) -> PyResult<Image> {
    Ok(operations::cohen_sutherland(image, p0, p1,color, boundary))
}

#[pyfunction]
fn project_to_2d(image: Image, edges: Vec<HomogeneousEdge>, matrix: [[f32;4];4], scale:[f32;4], rotation_degrees: f32, rotation_axis: char, rotate_around_center: bool)-> PyResult<Image> {
    Ok(operations::project_to_2d(image, edges, matrix, scale, rotation_degrees, rotation_axis, rotate_around_center))
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
    m.add_function(wrap_pyfunction!(select_area, m)?)?;
    m.add_function(wrap_pyfunction!(cohen_sutherland, m)?)?;
    m.add_function(wrap_pyfunction!(project_to_2d, m)?)?;
    Ok(())
}
