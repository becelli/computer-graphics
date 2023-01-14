use crate::common::*;

use ndarray::{arr1, arr2, ArrayBase, Dim, OwnedRepr};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
#[allow(unused_imports)]
use std::{f64::consts::PI, time::Instant, vec};

pub fn draw_line(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
    let mut new_image = image.clone();
    draw_line_helper(&mut new_image, &p0, &p1, &color);
    new_image
}

fn draw_line_helper(image: &mut Image, p0: &Point, p1: &Point, color: &Rgba) {
    let delta_x: i32 = p1.0 - p0.0;
    let delta_y: i32 = p1.1 - p0.1;
    if delta_x.abs() > delta_y.abs() {
        let m = delta_y as f64 / delta_x as f64;
        let mut p0_x = p0.0;
        let p1_x = p1.0;
        let increment = if p1_x > p0_x { 1 } else { -1 };

        while p0_x != p1_x {
            let new_point: Point = (
                p0_x,
                (p0.1 as f64 + (p0_x - p0.0) as f64 * m).floor() as i32,
            );
            image[new_point.1 as usize][new_point.0 as usize] = *color;
            p0_x += increment;
        }
    } else {
        let m = delta_x as f64 / delta_y as f64;
        let mut p0_y = p0.1;
        let p1_y = p1.1;
        let increment = if p1_y > p0_y { 1 } else { -1 };
        while p0_y != p1_y {
            let new_point: Point = (
                (p0.0 as f64 + (p0_y - p0.1) as f64 * m).floor() as i32,
                p0_y,
            );
            image[new_point.1 as usize][new_point.0 as usize] = *color;
            p0_y += increment;
        }
    }
}

pub fn draw_line_bresenham(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
    let mut new_image: Image = image.clone();
    draw_line_bresenham_helper(&mut new_image, &p0, &p1, &color);
    new_image
}

//draw a line using the bresenham algorithm
fn draw_line_bresenham_helper(image: &mut Image, p0: &Point, p1: &Point, color: &Rgba) {
    let delta_x: i32 = (p1.0 - p0.0) as i32;
    let delta_y: i32 = (p1.1 - p0.1) as i32;
    // Determine the increments
    let (inc_x, inc_y) = match (delta_x, delta_y) {
        (x, y) if x >= 0 && y >= 0 => (1, 1),
        (x, y) if x >= 0 && y < 0 => (1, -1),
        (x, y) if x < 0 && y >= 0 => (-1, 1),
        _ => (-1, -1),
    };
    if delta_x.abs() >= delta_y.abs() {
        let mut yp: i32 = p0.1;
        let mut d: i32 = 2 * (inc_y * delta_y) - (inc_x * delta_x);
        let mut x: i32 = p0.0;
        while x != p1.0 {
            if d > 0 {
                yp += inc_y;
                let new_point: Point = (x, yp);
                image[new_point.1 as usize][new_point.0 as usize] = *color;
                d += 2 * (inc_y * delta_y - inc_x * delta_x);
            } else {
                let new_point: Point = (x, yp);
                image[new_point.1 as usize][new_point.0 as usize] = *color;
                d += 2 * inc_y * delta_y;
            }
            x += inc_x;
        }
    } else {
        let mut xp: i32 = p0.0 as i32;
        let mut d: i32 = (2 * (inc_x * delta_x) - (inc_y * delta_y)) as i32;
        let mut y: i32 = p0.1 as i32;
        while y != p1.1 as i32 {
            if d > 0 {
                xp += inc_x;
                let new_point: Point = (xp, y);
                image[new_point.1 as usize][new_point.0 as usize] = *color;
                d += 2 * (inc_x * delta_x - inc_y * delta_y);
            } else {
                let new_point: Point = (xp, y);
                image[new_point.1 as usize][new_point.0 as usize] = *color;
                d += 2 * inc_x * delta_x;
            }
            y += inc_y;
        }
    }
}

fn calculate_radius(p0: Point, p1: Point) -> i32 {
    // radius for a circle from p0 to p1
    let delta_x = p1.0 - p0.0;
    let delta_y = p1.1 - p0.1;
    let radius = ((delta_x.pow(2) + delta_y.pow(2)) as f64).sqrt();
    radius as i32
}

//p0 is the center, p1 is a point which belongs to the circunference
pub fn draw_circle(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
    let mut new_image: Image = image.clone();
    let height = image.len();
    let width = image[0].len();
    let radius = calculate_radius(p0, p1);

    for x in -radius..radius {
        let x_circunference: i32 = p0.0 - x;
        let temp_y: i32 = ((radius.pow(2) - x.pow(2)) as f64).sqrt() as i32;
        //reminder: an higher y implies in a lower pixel position
        let y_upper: i32 = p0.1 - temp_y;
        let y_lower: i32 = p0.1 + temp_y;

        if x_circunference >= 0 && x_circunference < width as i32 {
            if y_upper >= 0 && y_upper < height as i32 {
                new_image[y_upper as usize][x_circunference as usize] = color;
            }
            if y_lower >= 0 && y_lower < height as i32 {
                new_image[y_lower as usize][x_circunference as usize] = color;
            }
        }
    }
    new_image
}

pub fn draw_circle_bresenham(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
    let mut new_image: Image = image.clone();
    draw_circle_bresenham_helper(&mut new_image, &p0, &p1, &color);
    new_image
}

fn draw_circle_bresenham_helper(image: &mut Image, p0: &Point, p1: &Point, color: &Rgba) {
    let height = image.len();
    let width = image[0].len();
    let radius = calculate_radius(*p0, *p1);

    let mut x = 0;
    let mut y = radius;
    let mut d = 3 - 2 * radius;

    while x <= y {
        let plot_points = vec![
            (p0.0 + x, p0.1 + y),
            (p0.0 + x, p0.1 - y),
            (p0.0 - x, p0.1 + y),
            (p0.0 - x, p0.1 - y),
            (p0.0 + y, p0.1 + x),
            (p0.0 + y, p0.1 - x),
            (p0.0 - y, p0.1 + x),
            (p0.0 - y, p0.1 - x),
        ];

        for point in plot_points {
            if point.0 >= 0 && point.0 < width as i32 && point.1 >= 0 && point.1 < height as i32 {
                image[point.1 as usize][point.0 as usize] = *color;
            }
        }

        if d < 0 {
            d = d + 4 * x + 6;
        } else {
            d = d + 4 * (x - y) + 10;
            y = y - 1;
        }
        x = x + 1;
    }
}

pub fn draw_circle_parametric(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
    let mut new_image: Image = image.clone();
    let height = image.len() as i32;
    let width = image[0].len() as i32;
    let radius = calculate_radius(p0, p1);
    let mut a: f64 = 0.0;
    // step is proportional to the radius
    let step = 1.0 / (radius).pow(2) as f64;
    while a < 2.0 * PI {
        let x = (radius as f64 * a.cos()) as i32;
        let y = (radius as f64 * a.sin()) as i32;
        let new_point: Point = (p0.0 + x, p0.1 + y);
        if new_point.0 >= 0 && new_point.0 < width && new_point.1 >= 0 && new_point.1 < height {
            new_image[new_point.1 as usize][new_point.0 as usize] = color;
        }
        a += step;
    }
    new_image
}

pub fn draw_triangle(image: Image, p0: Point, p1: Point, p2: Point, color: Rgba) -> Image {
    let mut new_image = image.clone();
    draw_line_bresenham_helper(&mut new_image, &p0, &p1, &color);
    draw_line_bresenham_helper(&mut new_image, &p1, &p2, &color);
    draw_line_bresenham_helper(&mut new_image, &p2, &p0, &color);
    new_image
}

//convert a homogeneous point to point
fn homogeneous_point_to_point(h_point: HomogeneousPoint) -> Point {
    // verify if the polar coordinate is valid
    let (x, y, z, w) = h_point;
    let normalized_h_point = match w.abs() < std::f64::EPSILON {
        true => (x / w, y / w, z / w, 1.0),
        false => h_point,
    };

    let point: Point = (normalized_h_point.0 as i32, normalized_h_point.1 as i32);
    point
}

//apply the rotation matrix to the matrix
fn scale_matrix_3d(scale: [f64; 4]) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> {
    // rustfmt-ignore
    let s_matrix: [[f64; 4]; 4] = [
        [scale[0], 0., 0., 0.],
        [0., scale[1], 0., 0.],
        [0., 0., scale[2], 0.],
        [0., 0., 0., scale[3]],
    ];
    arr2(&s_matrix)
}

fn translation_matrix_3d(
    x_translation: f64,
    y_translation: f64,
    z_translation: f64,
) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> {
    // rustfmt-ignore
    let matrix: [[f64; 4]; 4] = [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [x_translation, y_translation, z_translation, 1.],
    ];
    let translation_matrix = arr2(&matrix);
    translation_matrix
}

fn calculate_center(edges: &Vec<HomogeneousEdge>) -> HomogeneousPoint {
    let center: HomogeneousPoint;
    let mut center_x: f64 = 0.;
    let mut center_y: f64 = 0.;
    let mut center_z: f64 = 0.;
    for edge in edges.iter() {
        center_x += edge.0 .0;
        center_x += edge.1 .0;
        center_y += edge.0 .1;
        center_y += edge.1 .1;
        center_z += edge.0 .2;
        center_z += edge.1 .2;
    }
    center = (
        center_x / (2. * edges.len() as f64),
        center_y / (2. * edges.len() as f64),
        center_z / (2. * edges.len() as f64),
        1.,
    );
    center
}

//apply the rotation matrix to the matrix
fn get_rotation_matrix_3d(
    edges: &Vec<HomogeneousEdge>,
    rotation_degrees: f64,
    rotation_axis: char,
    rotate_around_center: bool,
) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> {
    // rustfmt-ignore
    let mut matrix = [
        [0., 0., 0., 0.],
        [0., 0., 0., 0.],
        [0., 0., 0., 0.],
        [0., 0., 0., 1.],
    ];
    match rotation_axis {
        'x' => {
            matrix[0][0] += 1.;
            matrix[1][1] += rotation_degrees.to_radians().cos();
            matrix[1][2] += -rotation_degrees.to_radians().sin();
            matrix[2][1] += rotation_degrees.to_radians().sin();
            matrix[2][2] += rotation_degrees.to_radians().cos();
        }
        'y' => {
            matrix[0][0] += rotation_degrees.to_radians().cos();
            matrix[0][2] += -rotation_degrees.to_radians().sin();
            matrix[2][0] += rotation_degrees.to_radians().sin();
            matrix[2][2] += rotation_degrees.to_radians().cos();
            matrix[1][1] += 1.;
        }
        'z' => {
            matrix[0][0] += rotation_degrees.to_radians().cos();
            matrix[0][1] += -rotation_degrees.to_radians().sin();
            matrix[1][0] += rotation_degrees.to_radians().sin();
            matrix[1][1] += rotation_degrees.to_radians().cos();
            matrix[2][2] += 1.;
        }
        _ => {
            matrix[0][0] += 1.;
            matrix[1][1] += 1.;
            matrix[2][2] += 1.;
        }
    }
    //rotate around the center of the image
    let final_matrix: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>;
    if rotate_around_center {
        let center: HomogeneousPoint = calculate_center(&edges);
        //translate the matrix to the center, the apply the rotation and then translate it back to the original position
        let temp_matrix =
            translation_matrix_3d(-center.0, -center.1, -center.2).dot(&arr2(&matrix));
        final_matrix = temp_matrix.dot(&translation_matrix_3d(center.0, center.1, center.2));
    } else {
        final_matrix = arr2(&matrix);
    }
    final_matrix
}

//apply the transformation matrix to the set of edges.
fn apply_transformation(
    edges: &Vec<HomogeneousEdge>,
    transformation_matrix: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
) -> Vec<HomogeneousEdge> {
    let mut new_edges: Vec<HomogeneousEdge> = Vec::new();

    for edge in edges.iter() {
        let point1 = edge.0;
        let point2 = edge.1;

        //apply the transformation for the first point in the edge
        let product = arr1(&[
            point1.0 as f64,
            point1.1 as f64,
            point1.2 as f64,
            point1.3 as f64,
        ])
        .dot(&transformation_matrix);
        let new_point1: HomogeneousPoint = (
            product[0] / product[3],
            product[1] / product[3],
            product[2] / product[3],
            1.,
        );

        //apply the transformation for the second point in the edge
        let product = arr1(&[
            point2.0 as f64,
            point2.1 as f64,
            point2.2 as f64,
            point2.3 as f64,
        ])
        .dot(&transformation_matrix);
        let new_point2: HomogeneousPoint = (
            product[0] / product[3],
            product[1] / product[3],
            product[2] / product[3],
            1.,
        );

        //make a new set of homogeneous edge
        let new_edge = (new_point1, new_point2);
        new_edges.push(new_edge);
    }
    new_edges
}

//project the a 3d set of points to a 2d image.
fn project_to_2d(image: Image, new_edges: &Vec<HomogeneousEdge>) -> Image {
    let mut new_image: Image = image.clone();

    //invert the y axis
    let y_max = image.len() as i32 - 1;

    for edge in new_edges.iter() {
        let mut p0 = homogeneous_point_to_point(edge.0);
        let mut p1 = homogeneous_point_to_point(edge.1);

        p0.1 = y_max - p0.1;
        p1.1 = y_max - p1.1;

        let color = [0, 0, 0, 255];

        let boundary = ((0, 0), (image[0].len() as i32, image.len() as i32));

        new_image = cohen_sutherland(new_image, p0, p1, color, boundary);
    }
    new_image
}

//rotate an object
pub fn rotate_object(
    image: Image,
    edges: Vec<HomogeneousEdge>,
    degrees: f64,
    axis: char,
    center: bool,
) -> (Image, Vec<HomogeneousEdge>) {
    let transformation_matrix = get_rotation_matrix_3d(&edges, degrees, axis, center);

    //applying the transformation for each point in edged
    let new_edges: Vec<HomogeneousEdge> = apply_transformation(&edges, transformation_matrix);

    // let new_edges_clone = new_edges.clone();
    //drawing each edge of the drawing
    let new_image = project_to_2d(image, &new_edges);

    (new_image, new_edges)
}

//apply shearing to the object
pub fn shear_object(
    image: Image,
    edges: Vec<HomogeneousEdge>,
    matrix: [[f64; 4]; 4],
) -> (Image, Vec<HomogeneousEdge>) {
    let transformation_matrix = arr2(&matrix);

    //applying the transformation for each point in edge
    let new_edges: Vec<HomogeneousEdge> = apply_transformation(&edges, transformation_matrix);

    //drawing each edge of the drawing
    let new_image = project_to_2d(image, &new_edges);

    (new_image, new_edges)
}

//apply the given scales to the object
pub fn scale_object(
    image: Image,
    edges: Vec<HomogeneousEdge>,
    scale: [f64; 4],
) -> (Image, Vec<HomogeneousEdge>) {
    let transformation_matrix = scale_matrix_3d(scale);

    //applying the transformation for each point in edge
    let new_edges: Vec<HomogeneousEdge> = apply_transformation(&edges, transformation_matrix);
    //drawing each edge of the drawing
    let new_image = project_to_2d(image, &new_edges);

    (new_image, new_edges)
}

//translate an object around the screen
pub fn translate_object(
    image: Image,
    edges: Vec<HomogeneousEdge>,
    axis: [f64; 3],
) -> (Image, Vec<HomogeneousEdge>) {
    let transformation_matrix = translation_matrix_3d(axis[0], axis[1], axis[2]);

    //applying the transformation for each point in edge
    let new_edges: Vec<HomogeneousEdge> = apply_transformation(&edges, transformation_matrix);
    //drawing each edge of the drawing
    let new_image = project_to_2d(image, &new_edges);
    (new_image, new_edges)
}

//show the selected area
pub fn select_area(image: Image, p0: Point, p1: Point) -> Image {
    let mut new_image: Image = image.clone();
    let p2: Point = (p0.0, p1.1);
    let p3: Point = (p1.0, p0.1);
    let color: Rgba = [0, 0, 0, 0];
    new_image = draw_line_bresenham(new_image, p0, p2, color);
    new_image = draw_line_bresenham(new_image, p0, p3, color);
    new_image = draw_line_bresenham(new_image, p1, p2, color);
    new_image = draw_line_bresenham(new_image, p1, p3, color);
    new_image
}

fn assign_code_to_point(p0: &Point, borders: &Border) -> u8 {
    let mut code: u8 = 0b0000;
    if p0.1 >= borders.top {
        code += 0b1000;
    } else if p0.1 < borders.bottom {
        code += 0b0100;
    }

    if p0.0 >= borders.right {
        code += 0b0010;
    } else if p0.0 < borders.left {
        code += 0b0001;
    }
    code
}

//this function assumes the 3 points are colinear, but checks if p2 is between p0 and p1
fn is_inside_segment(p0: &Point, p1: &Point, p2: &Point) -> bool {
    let mut inside: bool = false;
    if p0.0 < p1.0 {
        if p2.0 > p0.0 && p2.0 < p1.0 {
            inside = true;
        }
    } else if p0.0 == p1.0 {
        //teste de linha vertical
        if p2.1 > p0.1.min(p1.1) && p2.1 < p0.1.max(p1.1) {
            inside = true;
        }
    } else {
        if p2.0 < p0.0 && p2.0 > p1.0 {
            inside = true;
        }
    }
    inside
}

fn points_in_screen(p0: &Point, p1: &Point, borders: &Border) -> Option<Edge> {
    //calculate which points will be inside the screen
    let delta_y = p1.1 - p0.1;
    let delta_x = p1.0 - p0.0;

    //calculate the points of the line which crosses the screen borders
    let xt: i32;
    let xb: i32;
    let yr: i32;
    let yl: i32;
    let mut points: Vec<Point> = vec![];
    if delta_x == 0 {
        xb = p0.0;
        xt = p0.0;
        yr = borders.top - 1;
        yl = borders.bottom;
        points.push((xb, yl));
        points.push((xt, yr));
    } else if delta_y == 0 {
        xb = borders.left;
        xt = borders.right - 1;
        yr = p0.1;
        yl = p0.1;
        points.push((xb, yl));
        points.push((xt, yr));
    } else {
        let m: f64 = delta_y as f64 / delta_x as f64;
        xt = ((1.0 / m) * (borders.top - p0.1) as f64 + p0.0 as f64).round() as i32;
        xb = ((1.0 / m) * (borders.bottom - p0.1) as f64 + p0.0 as f64).round() as i32;
        yr = (m * (borders.right - p0.0) as f64 + p0.1 as f64).round() as i32;
        yl = (m * (borders.left - p0.0) as f64 + p0.1 as f64).round() as i32;
        points.push((borders.left, yl));
        points.push((xt, borders.top - 1));
        points.push((xb, borders.bottom));
        points.push((borders.right - 1, yr));
    }

    // create a array of points localized at the edges of the screen
    //let points: [Point; 4] = [(borders.left, yl), (xt, borders.top), (xb, borders.bottom), (borders.right, yr)];

    let mut line_points: Vec<Point> = vec![];

    // verify which points are inside the screen
    for point in points.iter() {
        if assign_code_to_point(point, &borders) == 0 {
            line_points.push(*point);
        }
    }

    //check if at least one of the original point is inside the screen
    let new_line: Edge;
    let code_p0 = assign_code_to_point(p0, borders);
    let code_p1 = assign_code_to_point(p1, borders);

    //check if 2 points are inside the screen, if not, then return none
    if line_points.len() > 0 {
        let mut new_p0: Point = line_points[0];
        let mut new_p1: Point = line_points[1];

        //check if one of the given points is already inside the screen
        if code_p0 == 0 {
            //this point is inside the screen
            new_p0 = *p0;
            new_p1 = *p0;
            for point in line_points.iter() {
                if is_inside_segment(p0, p1, point) {
                    new_p1 = *point;
                    break;
                }
            }
        } else if code_p1 == 0 {
            //this point is inside the screen
            new_p0 = *p1;
            new_p1 = *p1;
            for point in line_points.iter() {
                if is_inside_segment(p0, p1, point) {
                    new_p0 = *point;
                    break;
                }
            }
        }
        new_line = (new_p0, new_p1);

        Some(new_line)
    } else {
        None
    }
}

pub fn cohen_sutherland(image: Image, p0: Point, p1: Point, color: Rgba, boundary: Edge) -> Image {
    let mut new_image: Image = image.clone();
    //find the points of the borders of the screen

    let (xl, xr) = (
        boundary.0 .0.min(boundary.1 .0),
        boundary.0 .0.max(boundary.1 .0),
    );
    let (yt, yb) = (
        boundary.0 .1.max(boundary.1 .1),
        boundary.0 .1.min(boundary.1 .1),
    );

    let borders = Border {
        top: yt,
        bottom: yb,
        right: xr,
        left: xl,
    };

    //assign the code to each point of the line
    let code_p0 = assign_code_to_point(&p0, &borders);
    let code_p1 = assign_code_to_point(&p1, &borders);
    // show the 4 bits of the code

    //check if the line is entirely inside screen, if not clip it
    if code_p0 == 0 && code_p1 == 0 {
        new_image = draw_line_bresenham(new_image, p0, p1, color);
    } else {
        if (code_p0 & code_p1) == 0 {
            let clipped_line: Option<Edge> = points_in_screen(&p0, &p1, &borders);

            if clipped_line.is_some() {
                let (new_p0, new_p1) = clipped_line.unwrap();
                new_image = draw_line_bresenham(new_image, new_p0, new_p1, color);
            }
        }
    }
    new_image
}

// Code for the 2 part of the assignment
// Code for the 2 part of the assignment
// Code for the 2 part of the assignment
// Code for the 2 part of the assignment
// Code for the 2 part of the assignment

fn is_similar_color(color1: Rgba, color2: Rgba, tolerance: f64) -> bool {
    let r1 = f64::from(color1[0]);
    let g1 = f64::from(color1[1]);
    let b1 = f64::from(color1[2]);
    let r2 = f64::from(color2[0]);
    let g2 = f64::from(color2[1]);
    let b2 = f64::from(color2[2]);
    let delta_r = (r1 - r2).abs();
    let delta_g = (g1 - g2).abs();
    let delta_b = (b1 - b2).abs();
    let max = 255.0;
    let delta_r = delta_r / max;
    let delta_g = delta_g / max;
    let delta_b = delta_b / max;
    let delta = (delta_r + delta_g + delta_b) / 3.0;
    delta < tolerance
}

fn get_neighbors_4(point: Point) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::new();
    let x = point.0;
    let y = point.1;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            if i == 0 || j == 0 {
                neighbors.push((x + i, y + j));
            }
        }
    }
    neighbors
}

fn get_neighbors_8(point: Point) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::new();
    let x = point.0;
    let y = point.1;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            neighbors.push((x + i, y + j));
        }
    }
    neighbors
}

pub fn flood_fill(image: Image, p0: Point, color: Rgba, n4: bool) -> Image {
    let mut new_image: Image = image.clone();
    let height = image.len();
    let width = image[0].len();
    // fill the region that is 5% similar to the color of the point.
    let tolerance = 0.05;

    let old_color = new_image[p0.1 as usize][p0.0 as usize];
    let mut queue: Vec<Point> = Vec::new();

    let get_neighbors = match n4 {
        true => get_neighbors_4,
        false => get_neighbors_8,
    };

    queue.push(p0);
    while queue.len() > 0 {
        let point = queue.pop().unwrap();
        let (x, y) = point;

        if new_image[y as usize][x as usize] == color {
            continue;
        }

        new_image[y as usize][x as usize] = color;

        let neighbors = get_neighbors(point);

        for neighbor in neighbors {
            if neighbor.0 >= 0
                && neighbor.0 < width as i32
                && neighbor.1 >= 0
                && neighbor.1 < height as i32
            {
                let neighbor_color = new_image[neighbor.1 as usize][neighbor.0 as usize];
                if !neighbor_color.eq(&color)
                    && is_similar_color(neighbor_color, old_color, tolerance)
                {
                    queue.push(neighbor);
                }
            }
        }
    }

    new_image
}

fn get_bounding_box(image: &Image, color: &Rgba) -> (i32, i32, i32, i32) {
    let (min_x, max_x, min_y, max_y) = image
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, pixel)| pixel.eq(&color))
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .fold(
            (std::i32::MAX, std::i32::MIN, std::i32::MAX, std::i32::MIN),
            |(min_x, max_x, min_y, max_y), (x, y)| {
                (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
            },
        );
    (min_x - 1, max_x + 1, min_y - 1, max_y + 1)
}

pub fn edge_fill(image: Image, color: Rgba) -> Image {
    let mut new_image: Image = image.clone();
    let (min_x, max_x, min_y, max_y) = get_bounding_box(&image, &color);

    for i in min_x + 1..max_x {
        let is_horizontal_line: bool;
        let mut still_in_borders: bool = false;
        let mut count = 0;
        for j in min_y + 1..max_y {
            let old_color = new_image[j as usize][i as usize];
            if old_color.eq(&color) {
                if !still_in_borders {
                    count += 1;
                }
                still_in_borders = true;
            } else {
                still_in_borders = false;
            }
        }
        is_horizontal_line = if count >= 2 { false } else { true };

        if !is_horizontal_line {
            let mut is_inside: bool = false;
            let mut still_in_borders: bool = false;
            for j in min_y + 1..max_y {
                let old_color = new_image[j as usize][i as usize];

                let (x, y) = (i, j);

                if !old_color.eq(&color) {
                    if is_inside {
                        new_image[y as usize][x as usize] = color;
                    }
                    if still_in_borders {
                        still_in_borders = false;
                    }
                } else {
                    if !still_in_borders {
                        is_inside = !is_inside;
                    }
                    still_in_borders = true;
                }
            }
        }
    }

    new_image
}

//create a 3d object via a rotation sweep
pub fn rotate_plane_sweep(image: Image, color: Rgba) -> Image {
    let width = image[0].len() as i32;
    let mut new_image: Image = image.clone();
    let mut points_to_sweep: Vec<ObjectPoint> = vec![];
    let (xl, xr) = (0, image[0].len() as i32);
    let (yt, yb) = (0, image.len() as i32);

    for y in yt..yb - 1 {
        for x in xl..xr - 1 {
            if new_image[y as usize][x as usize] == color {
                let point: ObjectPoint =
                    ((((width / 2) - x).abs() as f64, y as f64, 0., 1.), color);
                points_to_sweep.push(point);
                new_image[y as usize][x as usize] =
                    [255 - color[0], 255 - color[1], 255 - color[2], 255];
            }
        }
    }

    let mut sweeped_points = vec![];

    for i in 0..360 {
        let sweeped = &mut rotate_3d_object(&points_to_sweep, i as f64, 'y', false);
        sweeped_points.append(sweeped);
    }

    // Sweeping 30 degrees in the x axis to visualize the 3d object
    sweeped_points = rotate_3d_object(&sweeped_points, 30., 'x', false);
    // Translate the object to the center of the screen
    sweeped_points = translate_3d_object(&sweeped_points, ((width / 2) as f64, 0., 0., 1.));

    new_image = print_objects_in_screen(new_image, sweeped_points, false);

    new_image
}

//apply the z-buffer to a set of points
fn z_buffer(object: Vec<ObjectPoint>) -> HashMap<(i32, i32), ObjectPoint> {
    let mut buffered_points: HashMap<(i32, i32), ObjectPoint> = HashMap::new();

    for point in object {
        let h_point = point.0;
        let b_point = buffered_points.get(&(h_point.0 as i32, h_point.1 as i32));
        if b_point.is_none() || (h_point.2 < b_point.unwrap().0 .2) {
            buffered_points.insert((h_point.0 as i32, h_point.1 as i32), point);
        }
    }

    buffered_points
}

//print a 3d object in 2d
pub fn print_objects_in_screen(image: Image, points: Vec<ObjectPoint>, invert: bool) -> Image {
    let mut new_image = image.clone();
    let z_buffered_objects = z_buffer(points);
    let height = image.len() as i32;
    let width: i32 = image[0].len() as i32;
    for point in z_buffered_objects.values() {
        let new_point = homogeneous_point_to_point(point.0);
        let color = point.1;
        if invert {
            if new_point.0 >= 0 && new_point.0 < width && new_point.1 > 0 && new_point.1 <= height {
                new_image[(height - new_point.1) as usize][(new_point.0) as usize] = color;
            }
        } else {
            if new_point.0 >= 0 && new_point.0 < width && new_point.1 >= 0 && new_point.1 < height {
                new_image[(new_point.1) as usize][(new_point.0) as usize] = color;
            }
        }
    }
    new_image
}

//get an type of object
pub fn get_object(object_type: u16) -> Vec<ObjectPoint> {
    let new_object: Vec<ObjectPoint> = match object_type {
        1 => get_z_buffer_objects(),
        _ => get_object_ramp(),
    };
    new_object
}

//return various objects to test in the z-buffer
fn get_z_buffer_objects() -> Vec<ObjectPoint> {
    let mut rendered_objects = generate_object_1(10, 30, 20, 40, [0, 0, 255, 255]);
    rendered_objects.append(&mut generate_object_2(50, 100, 30, 80, [255, 0, 0, 255]));
    rendered_objects.append(&mut generate_object_3(
        0.,
        2. * PI,
        0,
        50,
        [255, 255, 0, 255],
    ));
    rendered_objects.append(&mut generate_object_4(
        0.,
        2. * PI,
        0.,
        2. * PI,
        [0, 255, 0, 255],
    ));

    rendered_objects.append(&mut generate_object_5(20, (0, 0), [255, 255, 255, 255]));
    rendered_objects = translate_3d_object(&rendered_objects, (250., 250., 0., 1.));
    rendered_objects
}

//create a curve which is defined by z=x^2 + y
fn generate_object_1(
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    color: Rgba,
) -> Vec<ObjectPoint> {
    let mut new_object: Vec<ObjectPoint> = vec![];
    let delta = 0.1;
    let mut current_x = min_x as f64;
    while current_x <= max_x as f64 {
        let mut current_y = min_y as f64;
        while current_y <= max_y as f64 {
            let new_point: HomogeneousPoint = (
                current_x,
                current_y,
                (current_x * current_x + current_y),
                1.,
            );
            new_object.push((new_point, color) as ObjectPoint);
            current_y += delta;
        }
        current_x += delta;
    }

    new_object
}

//create a curve which is defined by z=3x-2y+5
fn generate_object_2(
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    color: Rgba,
) -> Vec<ObjectPoint> {
    let mut new_object: Vec<ObjectPoint> = Vec::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let new_point: HomogeneousPoint = (x.into(), y.into(), (3 * x - 2 * y + 5).into(), 1.);
            new_object.push((new_point, color) as ObjectPoint);
        }
    }
    new_object
}

//create a curve that is defined by
//x = 30+cos(a)t
//y = 50+sen(a)t
//z = 10 + t
fn generate_object_3(
    min_a: f64,
    max_a: f64,
    min_t: i32,
    max_t: i32,
    color: Rgba,
) -> Vec<ObjectPoint> {
    let mut new_object: Vec<ObjectPoint> = Vec::new();
    for t in min_t..=max_t {
        let mut current_a = min_a;
        loop {
            let new_point: HomogeneousPoint = (
                (30.0 + current_a.cos() * f64::from(t)).into(),
                (50.0 + current_a.sin() * f64::from(t)).into(),
                (10.0 + f64::from(t)).into(),
                1.,
            );
            new_object.push((new_point, color) as ObjectPoint);
            // current_a += 0.01;

            current_a += 0.1 / (t as f64 + 1.);
            if current_a > max_a {
                break;
            }
        }
    }
    new_object
}

//create a curve that is defined by
//x = 100+30cos(a)cos(b)
//y = 50+30cos(a)sen(b)
//z = 20+30sen(a)
fn generate_object_4(
    min_a: f64,
    max_a: f64,
    min_b: f64,
    max_b: f64,
    color: Rgba,
) -> Vec<ObjectPoint> {
    let mut new_object: Vec<ObjectPoint> = Vec::new();
    let mut current_b = min_b;
    loop {
        let mut current_a = min_a;
        loop {
            let new_point: HomogeneousPoint = (
                (100.0 + 30. * current_a.cos() * current_b.cos()).into(),
                (50.0 + 30. * current_a.cos() * current_b.sin()).into(),
                (20.0 + 30. * current_a.sin()).into(),
                1.,
            );
            new_object.push((new_point, color) as ObjectPoint);
            current_a += 0.01;
            if current_a > max_a {
                break;
            }
        }
        current_b += 0.01;
        if current_b > max_b {
            break;
        }
    }

    new_object
}

fn generate_object_5(side: i32, center: Point, color: Rgba) -> Vec<ObjectPoint> {
    //create a cube with a center in the point center and a side of side

    let mut new_object: Vec<ObjectPoint> = vec![];

    let (x, y) = center;

    for i in -side..=side {
        for j in -side..=side {
            let new_point: HomogeneousPoint = ((x + i).into(), (y + j).into(), -side as f64, 1.);
            new_object.push((new_point, color) as ObjectPoint);
        }
        for j in -side..=side {
            let new_point: HomogeneousPoint = ((x + i).into(), (y + j).into(), side as f64, 1.);
            new_object.push((new_point, color) as ObjectPoint);
        }
        for j in -side..=side {
            let new_point: HomogeneousPoint = ((x + i).into(), -side as f64, (y + j).into(), 1.);
            new_object.push((new_point, color) as ObjectPoint);
        }
        for j in -side..=side {
            let new_point: HomogeneousPoint = ((x + i).into(), side as f64, (y + j).into(), 1.);
            new_object.push((new_point, color) as ObjectPoint);
        }
        for j in -side..=side {
            let new_point: HomogeneousPoint = (side as f64, (x + i).into(), (y + j).into(), 1.);
            new_object.push((new_point, color) as ObjectPoint);
        }
        for j in -side..=side {
            let new_point: HomogeneousPoint = (-side as f64, (x + i).into(), (y + j).into(), 1.);
            new_object.push((new_point, color) as ObjectPoint);
        }
    }
    new_object
}

//calculate the center of an object
fn calculate_center_object(points: &Vec<ObjectPoint>) -> HomogeneousPoint {
    let center: HomogeneousPoint;
    let mut center_x: f64 = 0.;
    let mut center_y: f64 = 0.;
    let mut center_z: f64 = 0.;
    for point in points.iter() {
        center_x += point.0 .0;
        center_y += point.0 .1;
        center_z += point.0 .2;
    }
    center = (
        center_x / (points.len() as f64),
        center_y / (points.len() as f64),
        center_z / (points.len() as f64),
        1.,
    );
    center
}

//this function gets a rotation matrix for a 3d object, not a mesh like the previous function
fn get_rotation_matrix_3d_object(
    points: &Vec<ObjectPoint>,
    rotation_degrees: f64,
    rotation_axis: char,
    rotate_around_center: bool,
) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> {
    // rustfmt-ignore
    let mut matrix = [
        [0., 0., 0., 0.],
        [0., 0., 0., 0.],
        [0., 0., 0., 0.],
        [0., 0., 0., 1.],
    ];
    match rotation_axis {
        'x' => {
            matrix[0][0] += 1.;
            matrix[1][1] += rotation_degrees.to_radians().cos();
            matrix[1][2] += -rotation_degrees.to_radians().sin();
            matrix[2][1] += rotation_degrees.to_radians().sin();
            matrix[2][2] += rotation_degrees.to_radians().cos();
        }
        'y' => {
            matrix[0][0] += rotation_degrees.to_radians().cos();
            matrix[0][2] += -rotation_degrees.to_radians().sin();
            matrix[2][0] += rotation_degrees.to_radians().sin();
            matrix[2][2] += rotation_degrees.to_radians().cos();
            matrix[1][1] += 1.;
        }
        'z' => {
            matrix[0][0] += rotation_degrees.to_radians().cos();
            matrix[0][1] += -rotation_degrees.to_radians().sin();
            matrix[1][0] += rotation_degrees.to_radians().sin();
            matrix[1][1] += rotation_degrees.to_radians().cos();
            matrix[2][2] += 1.;
        }
        _ => {
            matrix[0][0] += 1.;
            matrix[1][1] += 1.;
            matrix[2][2] += 1.;
        }
    }
    //rotate around the center of the image
    let final_matrix: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>;
    if rotate_around_center {
        let center: HomogeneousPoint = calculate_center_object(&points);
        //translate the matrix to the center, the apply the rotation and then translate it back to the original position
        let temp_matrix =
            translation_matrix_3d(-center.0, -center.1, -center.2).dot(&arr2(&matrix));
        final_matrix = temp_matrix.dot(&translation_matrix_3d(center.0, center.1, center.2));
    } else {
        final_matrix = arr2(&matrix);
    }
    final_matrix
}

//this function rotates an 3d object, not an mesh, like the previous rotation function
pub fn rotate_3d_object(
    points: &Vec<ObjectPoint>,
    degrees: f64,
    axis: char,
    around_itself: bool,
) -> Vec<ObjectPoint> {
    let matrix = get_rotation_matrix_3d_object(points, degrees, axis, around_itself);
    let mut new_points: Vec<ObjectPoint> = Vec::new();
    for point in points {
        let product = arr1(&[
            point.0 .0 as f64,
            point.0 .1 as f64,
            point.0 .2 as f64,
            point.0 .3 as f64,
        ])
        .dot(&matrix);
        let new_point: HomogeneousPoint = (
            product[0] / product[3],
            product[1] / product[3],
            product[2] / product[3],
            1.,
        );
        new_points.push((new_point, point.1));
    }
    new_points
}

//move the object to a specific point
pub fn translate_3d_object(
    points: &Vec<ObjectPoint>,
    movement: HomogeneousPoint,
) -> Vec<ObjectPoint> {
    let mut new_points: Vec<ObjectPoint> = Vec::new();
    for point in points {
        let translated_point: HomogeneousPoint = (
            point.0 .0 + movement.0,
            point.0 .1 + movement.1,
            point.0 .2 + movement.2,
            1.,
        );
        new_points.push((translated_point, point.1));
    }
    new_points
}

//create a ramp
fn get_object_ramp() -> Vec<ObjectPoint> {
    let mut rendered_objects =
        get_object_plane_xy((0., 0., 0., 1.), (0, 20), (0, 40), [0, 255, 0, 255]);
    rendered_objects.append(&mut get_object_plane_xz(
        (0., 0., 0., 1.),
        (0, 20),
        (0, 80),
        [0, 255, 0, 255],
    ));
    rendered_objects.append(&mut get_object_plane_xy(
        (0., 0., 80., 1.),
        (0, 20),
        (0, 40),
        [0, 255, 0, 255],
    ));
    rendered_objects.append(&mut get_object_plane_yz(
        (20., 0., 0., 1.),
        (0, 40),
        (0, 80),
        [255, 255, 0, 255],
    ));
    rendered_objects.append(&mut get_object_plane_xy(
        (20., 0., 0., 1.),
        (0, 80),
        (0, 40),
        [255, 0, 0, 255],
    ));
    rendered_objects.append(&mut get_object_plane_xy(
        (100., 0., 0., 1.),
        (0, 20),
        (0, 40),
        [150, 75, 0, 255],
    ));
    rendered_objects.append(&mut get_object_plane_declined(
        (20., 0., 80., 1.),
        (0, 80),
        (0, 40),
        [0, 0, 255, 255],
    ));
    rendered_objects = translate_3d_object(&rendered_objects, (250., 250., 0., 1.));
    rendered_objects
}

fn get_object_sphere(center: HomogeneousPoint, radius: f64, color: Rgba) -> Vec<ObjectPoint> {
    let delta_theta = 2. * PI / (10. * radius);
    let delta_phi = 2. * PI / (10. * radius);
    let mut i = 0.;
    let mut rendered_sphere: Vec<ObjectPoint> = Vec::new();
    while i < 2. * PI {
        let mut j = 0.;
        while j < 2. * PI {
            let new_point: HomogeneousPoint = (
                center.0 + radius * i.cos() * j.cos(),
                center.1 + radius * i.cos() * j.sin(),
                center.2 + radius * i.sin(),
                center.3,
            );
            rendered_sphere.push((new_point, color));
            j += delta_phi;
        }
        i += delta_theta;
    }
    rendered_sphere
}

fn get_object_plane_xy(
    initial_translation: HomogeneousPoint,
    range_x: Point,
    range_y: Point,
    color: Rgba,
) -> Vec<ObjectPoint> {
    let mut new_object: Vec<ObjectPoint> = vec![];
    let delta = 0.3;
    let mut current_x = range_x.0 as f64;
    while current_x <= range_x.1 as f64 {
        let mut current_y = range_y.0 as f64;
        while current_y <= range_y.1 as f64 {
            let new_point: HomogeneousPoint = (
                initial_translation.0 + current_x,
                initial_translation.1 + current_y,
                initial_translation.2,
                1.,
            );
            new_object.push((new_point, color) as ObjectPoint);
            current_y += delta;
        }
        current_x += delta;
    }

    new_object
}

fn get_object_plane_xz(
    initial_translation: HomogeneousPoint,
    range_x: Point,
    range_z: Point,
    color: Rgba,
) -> Vec<ObjectPoint> {
    let mut new_object: Vec<ObjectPoint> = vec![];
    let delta = 0.3;
    let mut current_x = range_x.0 as f64;
    while current_x <= range_x.1 as f64 {
        let mut current_z = range_z.0 as f64;
        while current_z <= range_z.1 as f64 {
            let new_point: HomogeneousPoint = (
                initial_translation.0 + current_x,
                initial_translation.1,
                initial_translation.2 + current_z,
                1.,
            );
            new_object.push((new_point, color) as ObjectPoint);
            current_z += delta;
        }
        current_x += delta;
    }

    new_object
}

fn get_object_plane_yz(
    initial_translation: HomogeneousPoint,
    range_y: Point,
    range_z: Point,
    color: Rgba,
) -> Vec<ObjectPoint> {
    let mut new_object: Vec<ObjectPoint> = vec![];
    let delta = 0.3;
    let mut current_y = range_y.0 as f64;
    while current_y <= range_y.1 as f64 {
        let mut current_z = range_z.0 as f64;
        while current_z <= range_z.1 as f64 {
            let new_point: HomogeneousPoint = (
                initial_translation.0,
                initial_translation.1 + current_y,
                initial_translation.2 + current_z,
                1.,
            );
            new_object.push((new_point, color) as ObjectPoint);
            current_z += delta;
        }
        current_y += delta;
    }

    new_object
}

//create a declined plane
fn get_object_plane_declined(
    initial_translation: HomogeneousPoint,
    range_x: Point,
    range_y: Point,
    color: Rgba,
) -> Vec<ObjectPoint> {
    let mut new_object: Vec<ObjectPoint> = vec![];
    let delta = 0.3;
    let mut current_x = range_x.0 as f64;
    while current_x <= range_x.1 as f64 {
        let mut current_y = range_y.0 as f64;
        while current_y <= range_y.1 as f64 {
            let new_point: HomogeneousPoint = (
                initial_translation.0 + current_x,
                initial_translation.1 + current_y,
                initial_translation.2 - current_x,
                1.,
            );
            new_object.push((new_point, color) as ObjectPoint);
            current_y += delta;
        }
        current_x += delta;
    }

    new_object
}

fn array_norm(arr: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>) -> f64 {
    let mut norm: f64 = 0.;
    for num in arr {
        norm += num * num;
    }
    norm = norm.sqrt();
    norm
}

fn calculate_angle(vec1: HomogeneousPoint, vec2: HomogeneousPoint) -> f64 {
    let array1 = arr1(&[vec1.0 as f64, vec1.1 as f64, vec1.2 as f64]);
    let array2 = arr1(&[vec2.0 as f64, vec2.1 as f64, vec2.2 as f64]);
    let cos_delta: f64 = array1.dot(&array2) / (array_norm(array1) * array_norm(array2));
    cos_delta
}

fn calculate_distance(point_1: HomogeneousPoint, point_2: HomogeneousPoint) -> f64 {
    let diff = arr1(&[
        point_1.0 / point_1.3 - point_2.0 / point_2.3,
        point_1.1 / point_1.3 - point_2.1 / point_2.3,
        point_1.2 / point_1.3 - point_2.2 / point_2.3,
        0.,
    ]);
    let distance = array_norm(diff);
    distance
}

fn illumination_model_1(
    rendered_objects: Vec<ObjectPoint>,
    ia: f64,
    ka: f64,
    il: f64,
    kd: f64,
    lamp_pos: HomogeneousPoint,
) -> Vec<ObjectPoint> {
    let mut illuminated_object: Vec<ObjectPoint> = vec![];
    for point in rendered_objects {
        let normal: HomogeneousPoint = (0., 0., 1., 1.);
        // let cosine = calculate_angle(observer_pos, normal);
        let cosine = calculate_angle(lamp_pos, normal);
        // let cosine = calculate_angle(observer_pos, lamp_pos);
        let old_color = point.1;
        let illumination: f64 = ia * ka + il * kd * cosine;
        let new_color: Rgba = [
            (old_color[0] as f64 * illumination) as u8,
            (old_color[1] as f64 * illumination) as u8,
            (old_color[2] as f64 * illumination) as u8,
            old_color[3],
        ];
        illuminated_object.push((point.0, new_color));
    }
    illuminated_object
}

fn illumination_model_2(
    rendered_objects: Vec<ObjectPoint>,
    ia: f64,
    ka: f64,
    il: f64,
    kd: f64,
    ks: f64,
    k: f64,
    observer_pos: HomogeneousPoint,
    lamp_pos: HomogeneousPoint,
    n: f64,
) -> Vec<ObjectPoint> {
    let mut illuminated_object: Vec<ObjectPoint> = vec![];
    for point in rendered_objects {
        let normal: HomogeneousPoint = (0., 0., 1., 1.);
        let cosine_alfa = calculate_angle(observer_pos, lamp_pos);
        let cosine_delta = calculate_angle(lamp_pos, normal);

        let old_color = point.1;
        let d: f64 = calculate_distance(point.0, observer_pos);
        let illumination: f64 =
            ia * ka + il * (kd * cosine_delta + ks * cosine_alfa.powf(n)) / (d + k);
        let new_color: Rgba = [
            (old_color[0] as f64 * illumination) as u8,
            (old_color[1] as f64 * illumination) as u8,
            (old_color[2] as f64 * illumination) as u8,
            old_color[3],
        ];
        illuminated_object.push((point.0, new_color));
    }
    illuminated_object
}

fn illumination_model_1_sphere(
    rendered_objects: Vec<ObjectPoint>,
    ia: f64,
    ka: f64,
    il: f64,
    kd: f64,
    lamp_pos: HomogeneousPoint,
) -> Vec<ObjectPoint> {
    let mut illuminated_object: Vec<ObjectPoint> = vec![];
    for point in rendered_objects {
        let normal: HomogeneousPoint = point.0;
        let cosine = calculate_angle(lamp_pos, normal);
        let old_color = point.1;
        let illumination: f64 = ia * ka + il * kd * cosine;
        let new_color: Rgba = [
            (old_color[0] as f64 * illumination) as u8,
            (old_color[1] as f64 * illumination) as u8,
            (old_color[2] as f64 * illumination) as u8,
            old_color[3],
        ];
        illuminated_object.push((point.0, new_color));
    }
    illuminated_object
}

fn illumination_model_2_sphere(
    rendered_objects: Vec<ObjectPoint>,
    ia: f64,
    ka: f64,
    il: f64,
    kd: f64,
    ks: f64,
    k: f64,
    observer_pos: HomogeneousPoint,
    lamp_pos: HomogeneousPoint,
    n: f64,
) -> Vec<ObjectPoint> {
    let mut illuminated_object: Vec<ObjectPoint> = vec![];
    for point in rendered_objects {
        let normal: HomogeneousPoint = point.0;
        let cosine_alfa = calculate_angle(observer_pos, lamp_pos);
        let cosine_delta = calculate_angle(lamp_pos, normal);
        let old_color = point.1;
        let d: f64 = calculate_distance(point.0, observer_pos);
        let illumination: f64 =
            ia * ka + il * (kd * cosine_delta + ks * cosine_alfa.powf(n)) / (d + k);
        let new_color: Rgba = [
            (old_color[0] as f64 * illumination) as u8,
            (old_color[1] as f64 * illumination) as u8,
            (old_color[2] as f64 * illumination) as u8,
            old_color[3],
        ];
        illuminated_object.push((point.0, new_color));
    }
    illuminated_object
}

pub fn apply_luminosity(
    image: Image,
    model: u8,
    kd_1: f64,
    ks_1: f64,
    kd_2: f64,
    ks_2: f64,
    k: f64,
    ia: f64,
    ka: f64,
    il: f64,
    n: f64,
) -> Image {
    let center: HomogeneousPoint = (0., 0., 0., 1.);
    let radius = 50.;
    let lamp_pos: HomogeneousPoint = (100., 0., 100., 1.);
    let observer_pos: HomogeneousPoint = (0., 0., 100., 1.);
    let rendered_objects =
        get_object_plane_xy((0., 0., 0., 1.), (0, 100), (0, 100), [0, 0, 255, 255]);
    let rendered_sphere = get_object_sphere(center, radius, [255, 90, 255, 255]);

    let (mut object, mut sphere) = match model {
        1 => (
            illumination_model_1(rendered_objects, ia, ka, il, kd_1, lamp_pos),
            illumination_model_1_sphere(rendered_sphere, ia, ka, il, kd_2, lamp_pos),
        ),
        2 => (
            illumination_model_2(
                rendered_objects,
                ia,
                ka,
                il,
                kd_1,
                ks_1,
                k,
                observer_pos,
                lamp_pos,
                n,
            ),
            illumination_model_2_sphere(
                rendered_sphere,
                ia,
                ka,
                il,
                kd_2,
                ks_2,
                k,
                observer_pos,
                lamp_pos,
                n,
            ),
        ),
        _ => (rendered_objects, rendered_sphere),
    };

    object.append(&mut sphere);
    object = translate_3d_object(&object, (250., 250., 0., 1.));
    print_objects_in_screen(image, object, true)
}
