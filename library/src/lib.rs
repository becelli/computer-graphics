use common::ObjectPoint;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod common;
mod operations;
use common::{Edge, HomogeneousEdge, Image, Point, Rgba};

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
fn flood_fill(image: Image, p0: Point, color: Rgba, n4: bool) -> PyResult<Image> {
    Ok(operations::flood_fill(image, p0, color, n4))
}

#[pyfunction]
fn select_area(image: Image, p0: Point, p1: Point) -> PyResult<Image> {
    Ok(operations::select_area(image, p0, p1))
}

#[pyfunction]
fn cohen_sutherland(
    image: Image,
    p0: Point,
    p1: Point,
    color: Rgba,
    boundary: Edge,
) -> PyResult<Image> {
    Ok(operations::cohen_sutherland(image, p0, p1, color, boundary))
}

#[pyfunction]
fn translate_object(
    image: Image,
    edges: Vec<HomogeneousEdge>,
    axis: [f64; 3],
) -> PyResult<(Image, Vec<HomogeneousEdge>)> {
    Ok(operations::translate_object(image, edges, axis))
}

#[pyfunction]
fn scale_object(
    image: Image,
    edges: Vec<HomogeneousEdge>,
    scale: [f64; 4],
) -> PyResult<(Image, Vec<HomogeneousEdge>)> {
    Ok(operations::scale_object(image, edges, scale))
}

#[pyfunction]
fn shear_object(
    image: Image,
    edges: Vec<HomogeneousEdge>,
    matrix: [[f64; 4]; 4],
) -> PyResult<(Image, Vec<HomogeneousEdge>)> {
    Ok(operations::shear_object(image, edges, matrix))
}

#[pyfunction]
fn rotate_object(
    image: Image,
    edges: Vec<HomogeneousEdge>,
    degrees: f64,
    axis: char,
    center: bool,
) -> PyResult<(Image, Vec<HomogeneousEdge>)> {
    Ok(operations::rotate_object(
        image, edges, degrees, axis, center,
    ))
}

#[pyfunction]
fn get_object(
    range_y: Edge,
    range_x: Edge,
    color: Rgba,
    object_type: u16,
) -> PyResult<(Vec<ObjectPoint>)> {
    Ok(operations::get_object(
        range_y, range_x, color, object_type
    ))
}

#[pyfunction]
fn print_objects_in_screen(
    mut image: Image, points: Vec<ObjectPoint>
) -> PyResult<(Image)> {
    Ok(operations::print_objects_in_screen(
        image, points
    ))
}

#[pyfunction]
fn rotate_3d_object(
    points: Vec<ObjectPoint>,
    rotation_degrees: f64,
    rotation_axis: char,
    rotate_around_center: bool,
) -> PyResult<(Vec<ObjectPoint>)> {
    Ok(operations::rotate_3d_object(
        &points, rotation_degrees, rotation_axis, rotate_around_center
    ))
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
    m.add_function(wrap_pyfunction!(translate_object, m)?)?;
    m.add_function(wrap_pyfunction!(scale_object, m)?)?;
    m.add_function(wrap_pyfunction!(shear_object, m)?)?;
    m.add_function(wrap_pyfunction!(rotate_object, m)?)?;
    m.add_function(wrap_pyfunction!(get_object, m)?)?;
    m.add_function(wrap_pyfunction!(print_objects_in_screen, m)?)?;
    m.add_function(wrap_pyfunction!(rotate_3d_object, m)?)?;
    Ok(())
}
