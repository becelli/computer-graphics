use crate::common::*;

// use std::f64::consts::PI;
//use crate::nalgebra;
// use crate::transformations;
// use std::thread;



pub fn draw_line(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
    let width = image.len();
    let height = image[0].len();
    let mut new_image: Image = image.clone();
    let increment: i32;
    println!("x0 = {} y0 = {}", p0.0, p0.1);
    println!("x1 = {} y1 = {}", p1.0, p1.1);
    println!("");
    let delta_x:i32 = p1.0 as i32 - p0.0 as i32;
    let delta_y:i32 = p1.1 as i32 - p0.1 as i32;
    if delta_x.abs() >= delta_y.abs() {
        let m = delta_y as f32 / delta_x as f32;
        let mut p0_x = p0.0 as i32;
        let p1_x = p1.0 as i32;
        if p1_x > p0_x {
            increment = 1;
        } else {
            increment = -1;
        }
        
        while p0_x != p1_x  {
            let new_point: Point = (p0_x as u32, (p0.1 as f32 +  (p0_x as f32 - p0.0 as f32) * m).floor() as u32);
            new_image[new_point.1 as usize][new_point.0 as usize] = color;
            p0_x = p0_x + increment;
            println!("{} {}", new_point.0, new_point.1);
            println!("");
        }
    } else {
        let m = delta_x as f32/ delta_y as f32;
        let mut p0_y = p0.1 as i32;
        let p1_y = p1.1 as i32;
        if p1_y > p0_y {
            increment = 1;
        } else {
            increment = -1;
        }
        while p0_y != p1_y {
            let new_point: Point = ((p0.0 as f32 + (p0_y as f32 - p0.1 as f32) * m).floor() as u32, p0_y as u32);
            new_image[new_point.1 as usize][new_point.0 as usize] = color;
            p0_y = p0_y + increment;
            println!("{} {}", new_point.0, new_point.1);
            println!("");
        }
    }
    new_image
}

//draw a line using the bresenham algorithm
pub fn draw_line_bresenham(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
    let mut new_image: Image = image.clone();
    let increment_y: i32;
    let increment_x: i32;
    let delta_x: i32 = (p1.0 - p0.0) as i32;
    let delta_y: i32 = (p1.1 - p0.1) as i32;
    if delta_x.abs() >= delta_y.abs() {
        if delta_x >= 0 {
            //first or eighth octet
            increment_x = 1;
            if delta_y >= 0 {
                increment_y = 1;
            } else {
                increment_y = -1;
            }
        } else {
            //fourth or fifth octet
            increment_x = -1;
            if delta_y >= 0 {
                increment_y = -1;
            } else {
                increment_y = 1;
            }
        }
        let mut yp: i32 = p0.1 as i32;

        let mut d: i32 = (2 * (p1.1 - p0.1) - (p1.0 - p0.0)) as i32;
        let mut i: i32 = p0.0 as i32;
        while i <= p1.0 as i32 {
            if d > 0 {
                let new_point: Point = (i as u32, yp as u32);
                new_image[new_point.0 as usize][new_point.1 as usize] = color;
                d += (p1.1 - p0.1) as i32;
            } else {
                yp += increment_y;
                let new_point: Point = (i as u32, yp as u32);
                new_image[new_point.0 as usize][new_point.1 as usize] = color;
                d += ((p1.1 - p0.1) + (p1.0 - p0.0)) as i32;
            }
            i += increment_x;
        }
    } else {
        if delta_y >= 0 {
            //second or third octet
            increment_y = 1;
            if delta_x >= 0 {
                increment_x = 1;
            } else {
                increment_x = -1;
            }
        } else {
            //sixth or seventh octet
            increment_y = -1;
            if delta_x >= 0 {
                increment_x = -1;
            } else {
                increment_x = 1;
            }
        }
        let mut xp: i32 = p0.0 as i32;
        let mut d: i32 = (2 * (p1.0 - p0.0) - (p1.1 - p0.1)) as i32;
        let mut i: i32 = p0.1 as i32;
        while i < p1.1 as i32 {
            if d > 0 {
                let new_point: Point = (i as u32, xp as u32);
                new_image[new_point.0 as usize][new_point.1 as usize] = color;
                d += (p1.0 - p0.0) as i32;
            } else {
                xp += increment_x;
                let new_point: Point = (i as u32, xp as u32);
                new_image[new_point.0 as usize][new_point.1 as usize] = color;
                d += ((p1.0 - p0.0) + (p1.1 - p0.1)) as i32;
            }
            i += increment_y;
        }
    }
    new_image
}

fn calculate_radius(p0: Point, p1: Point) -> i32 {
    // radius for a circle from p0 to p1
    let delta_x = p1.0 - p0.0;
    let delta_y = p1.1 - p0.1;
    let radius = ((delta_x.pow(2) + delta_y.pow(2)) as f32).sqrt() as i32;
    radius
}

//p0 is the center, p1 is a point which belongs to the circunference
pub fn draw_circle(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
    let mut new_image: Image = image.clone();
    let radius = calculate_radius(p0, p1);

    for x in -1 * radius..radius {
        let x_circunference: i32 = p0.0 as i32 - x;
        let temp_y: i32 = ((radius.pow(2) + x.pow(2)) as f64).sqrt() as i32;
        //reminder: an higher y implies in a lower pixel position
        let y_upper: i32 = p0.1 as i32 - temp_y;
        let y_lower: i32 = p0.1 as i32 + temp_y;

        //do not draw if the circunference surpass the images boundaries
        if !(x_circunference > image.len() as i32
            || x_circunference < 0
            || y_upper < 0
            || y_lower > image[0].len() as i32)
        {
            //draws the upper and lower half of the circunference
            let new_point_1: Point = (x_circunference as u32, y_upper as u32);
            new_image[new_point_1.0 as usize][new_point_1.1 as usize] = color;

            let new_point_2: Point = (x_circunference as u32, y_lower as u32);
            new_image[new_point_2.0 as usize][new_point_2.1 as usize] = color;
        }
    }
    new_image
}

// pub fn parametric_circunference(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
//     let mut new_image: Image = image.clone();
//     let radius = calculate_radius(p0, p1);
//     let mut a: f32 = 0.0;
//     let step = 0.01;
//     while a < PI * 2.0 {
//         let temp_x: f32 = radius * a.cos();
//         let temp_y: f32 = radius * a.sin();
//         let x_circunference: i32 = p0.0 + temp_x;
//         let y_circunference: i32 = p0.1 + temp_y;
//         if !(x_circunference > image.len()
//             || x_circunference < 0
//             || y_circunference < 0
//             || y_circunference > image[0].len())
//         {
//             let new_point_1: Point = (x_circunference as u32, y_circunference as u32);
//             new_image[new_point_1.0 as usize][new_point_1.1 as usize] = color;
//         }
//         a += step;
//     }
// }


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
            (x, y),
            (x, -y),
            (-x, y),
            (-x, -y),
            (y, x),
            (y, -x),
            (-y, x),
            (-y, -x),
        ];

        for (x, y) in plot_points {
            let x = x + p0.0 as i32;
            let y = y + p0.1 as i32;
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                new_image[y as usize][x as usize] = color;
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