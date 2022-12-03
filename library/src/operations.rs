use crate::common::*;

use ndarray::{arr1, arr2, ArrayBase, Dim, OwnedRepr};
use std::{collections::VecDeque, f64::consts::PI};

pub fn draw_line(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
    let mut new_image: Image = image.clone();
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
            new_image[new_point.1 as usize][new_point.0 as usize] = color;
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
            new_image[new_point.1 as usize][new_point.0 as usize] = color;
            p0_y += increment;
        }
    }
    new_image
}

//draw a line using the bresenham algorithm
pub fn draw_line_bresenham(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
    let mut new_image: Image = image.clone();
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
                new_image[new_point.1 as usize][new_point.0 as usize] = color;
                d += 2 * (inc_y * delta_y - inc_x * delta_x);
            } else {
                let new_point: Point = (x, yp);
                new_image[new_point.1 as usize][new_point.0 as usize] = color;
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
                new_image[new_point.1 as usize][new_point.0 as usize] = color;
                d += 2 * (inc_x * delta_x - inc_y * delta_y);
            } else {
                let new_point: Point = (xp, y);
                new_image[new_point.1 as usize][new_point.0 as usize] = color;
                d += 2 * inc_x * delta_x;
            }
            y += inc_y;
        }
    }
    new_image
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
    let height = image.len();
    let width = image[0].len();
    let radius = calculate_radius(p0, p1);

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
                new_image[point.1 as usize][point.0 as usize] = color;
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

    new_image
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
    let mut new_image = draw_line_bresenham(image, p0, p1, color);
    new_image = draw_line_bresenham(new_image, p1, p2, color);
    new_image = draw_line_bresenham(new_image, p2, p0, color);
    new_image
}

fn is_similar_color(color1: Rgba, color2: Rgba, tolerance: f64) -> bool {
    let r1 = color1[0] as f64;
    let g1 = color1[1] as f64;
    let b1 = color1[2] as f64;
    let r2 = color2[0] as f64;
    let g2 = color2[1] as f64;
    let b2 = color2[2] as f64;
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

pub fn flood_fill(image: Image, p0: Point, color: Rgba) -> Image {
    let mut new_image: Image = image.clone();
    let height = image.len();
    let width = image[0].len();

    // fill the region that is 10% similar to the color of the point.
    let tolerance = 0.2;
    let old_color = new_image[p0.1 as usize][p0.0 as usize];
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(p0);
    while queue.len() > 0 {
        let point = queue.pop_front().unwrap();
        let x = point.0;
        let y = point.1;
        if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
            if is_similar_color(old_color, new_image[y as usize][x as usize], tolerance) {
                new_image[y as usize][x as usize] = color;
                queue.push_back((x + 1, y));
                queue.push_back((x - 1, y));
                queue.push_back((x, y + 1));
                queue.push_back((x, y - 1));
            }
        }
    }
    new_image
}

//convert a homogeneous point to point
fn homogeneous_point_to_point(h_point: HomogeneousPoint) -> Point {
    let normalized_h_point: HomogeneousPoint;

    // if h_point.3 is 0, then the point is at infinity
    // use safe float comparison

    if (h_point.3 - 0.0).abs() < std::f64::EPSILON {
        normalized_h_point = (
            h_point.0 / h_point.3,
            h_point.1 / h_point.3,
            h_point.2 / h_point.3,
            1.0,
        );
    } else {
        normalized_h_point = h_point;
    }

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
fn project_to_2d(
    image: Image,
    new_edges: &Vec<HomogeneousEdge>,
    original_edges: &Vec<HomogeneousEdge>,
) -> Image {
    let mut new_image: Image = image.clone();

    // calculate the center of the old edges
    let center: HomogeneousPoint = calculate_center(&original_edges);

    //invert the y axis
    let y_max = image.len() as i32 - 1;
    let x_max = image[0].len() as i32 - 1;
    // for each edge, print it

    for edge in new_edges.iter() {
        let mut p0 = homogeneous_point_to_point(edge.0);
        let mut p1 = homogeneous_point_to_point(edge.1);
        
        p0.0 = x_max / 2 + p0.0 - center.0 as i32;
        p1.0 = x_max / 2 + p1.0 - center.0 as i32;
        p0.1 = y_max / 2 - p0.1 + center.1 as i32;
        p1.1 = y_max / 2 - p1.1 + center.1 as i32;
        /*
        p0.0 = x_max / 2 + p0.0;
        p1.0 = x_max / 2 + p1.0;
        p0.1 = y_max / 2 - p0.1;
        p1.1 = y_max / 2 - p1.1;
        */
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
    o_edges: Vec<HomogeneousEdge>,
    degrees: f64,
    axis: char,
    center: bool,
) -> (Image, Vec<HomogeneousEdge>) {
    let transformation_matrix = get_rotation_matrix_3d(&edges, degrees, axis, center);

    //applying the transformation for each point in edge
    let new_edges: Vec<HomogeneousEdge> = apply_transformation(&edges, transformation_matrix);

    // let new_edges_clone = new_edges.clone();
    //drawing each edge of the drawing
    let new_image = project_to_2d(image, &new_edges, &o_edges);

    (new_image, new_edges)
}

//apply shearing to the object
pub fn shear_object(
    image: Image,
    edges: Vec<HomogeneousEdge>,
    o_edges: Vec<HomogeneousEdge>,
    matrix: [[f64; 4]; 4],
) -> (Image, Vec<HomogeneousEdge>) {
    let transformation_matrix = arr2(&matrix);

    //applying the transformation for each point in edge
    let new_edges: Vec<HomogeneousEdge> = apply_transformation(&edges, transformation_matrix);
    //drawing each edge of the drawing
    let new_image = project_to_2d(image, &new_edges, &o_edges);
    (new_image, new_edges)
}

//apply the given scales to the object
pub fn scale_object(
    image: Image,
    edges: Vec<HomogeneousEdge>,
    o_edges: Vec<HomogeneousEdge>,
    scale: [f64; 4],
) -> (Image, Vec<HomogeneousEdge>) {
    let transformation_matrix = scale_matrix_3d(scale);

    //applying the transformation for each point in edge
    let new_edges: Vec<HomogeneousEdge> = apply_transformation(&edges, transformation_matrix);
    //drawing each edge of the drawing
    let new_image = project_to_2d(image, &new_edges, &o_edges);

    (new_image, new_edges)
}

//translate an object around the screen
pub fn translate_object(
    image: Image,
    edges: Vec<HomogeneousEdge>,
    o_edges: Vec<HomogeneousEdge>,
    axis: [f64; 3],
) -> (Image, Vec<HomogeneousEdge>) {
    let transformation_matrix = translation_matrix_3d(axis[0], axis[1], axis[2]);

    //applying the transformation for each point in edge
    let new_edges: Vec<HomogeneousEdge> = apply_transformation(&edges, transformation_matrix);
    //drawing each edge of the drawing
    let new_image = project_to_2d(image, &new_edges, &o_edges);
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
            new_p0 = *p0;
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
    /*
    let yt = boundary.0 .0;
    let yb = boundary.1 .0;
    let xr = boundary.1 .1;
    let xl = boundary.0 .1;
    */
    let borders = Border {
        top: yt,
        bottom: yb,
        right: xr,
        left: xl,
    };

    //assign the code to each point of the line
    let code_p0 = assign_code_to_point(&p0, &borders);
    let code_p1 = assign_code_to_point(&p1, &borders);

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
