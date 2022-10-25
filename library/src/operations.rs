use crate::common::*;
//use crate::nalgebra;
// use crate::transformations;
// use std::thread;

//draw a line based on the formula: y = ax + b
pub fn draw_line(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
    //let width = image.len();
    //let height = image[0].len();
    let mut new_image: Image = image.clone();    
    let increment: i32;
    
    
    let delta_x = p1.0 - p0.0;
    let delta_y = p1.1 - p0.1;
    if delta_x >= delta_y{
        let m = ((p1.1 - p0.1)/(p1.0 - p0.0)) as f32;
        if p1.0 > p0.0 {
            increment = -1;
        }else{
            increment = 1;
        }
        let mut temp_x1 = p1.0;
        let temp_x0 = p0.0;
        while temp_x1 != temp_x0{
            let new_point: Point = (temp_x1, (temp_x1 as f32 * m)as u32);
            new_image[new_point.0 as usize][new_point.1 as usize] = color;
            temp_x1 += increment as u32;
        }
    }else{
        let m = ((p1.0 - p0.0)/(p1.1 - p0.1)) as f32;
        if p1.1 > p0.1 {
            increment = -1;
        }else{
            increment = 1;
        }
        let mut temp_y1 = p1.1;
        let temp_y0 = p0.1;
        while temp_y1 != temp_y0{
            let new_point: Point = (temp_y1, (temp_y1 as f32 * m)as u32);
            new_image[new_point.0 as usize][new_point.1 as usize] = color;
            temp_y1 += increment as u32;
        }
    }
    new_image
}

//draw a line using the bresenham algorithm
pub fn bresenham_line_algorithm(image: Image, p0: Point, p1: Point, color: Rgba) -> Image{
    let mut new_image: Image = image.clone();
    let increment_y:i32;
    let increment_x:i32;
    let delta_x:i32 = (p1.0 - p0.0) as i32;
    let delta_y:i32 = (p1.1 - p0.1) as i32;
    if delta_x.abs() >= delta_y.abs(){
        if delta_x >= 0{
            //first or eighth octet
            increment_x = 1;
            if delta_y >= 0{
                increment_y = 1;
            }else{
                increment_y = -1;
            }
        }else{//fourth or fifth octet
            increment_x = -1;
            if delta_y >= 0{
                increment_y = -1;
            }else{
                increment_y = 1;
            }
        }
        let mut yp:i32 = p0.1 as i32;
        
        let mut d:i32 = (2*(p1.1-p0.1) - (p1.0-p0.0)) as i32;
        let mut i:i32 = p0.0 as i32;
        while i <= p1.0 as i32{
            if d>0{
                let new_point: Point = (i as u32, yp as u32);
                new_image[new_point.0 as usize][new_point.1 as usize] = color;
                d += (p1.1-p0.1) as i32;
            }else{
                yp+=increment_y;
                let new_point: Point = (i as u32, yp as u32);
                new_image[new_point.0 as usize][new_point.1 as usize] = color;
                d += ((p1.1-p0.1) + (p1.0-p0.0)) as i32;
            }
            i+=increment_x;
        }
    }else{
        if delta_y >= 0{//second or third octet
            increment_y = 1;
            if delta_x >= 0{
                increment_x = 1;
            }else{
                increment_x = -1;
            }
        }else{//sixth or seventh octet
            increment_y = -1;
            if delta_x >= 0{
                increment_x = -1;
            }else{
                increment_x = 1;
            }
        }
        let mut xp:i32 = p0.0 as i32;
        let mut d:i32 = (2*(p1.0-p0.0) - (p1.1-p0.1)) as i32;
        let mut i:i32 = p0.1 as i32;
        while i < p1.1 as i32{
            if d>0{
                let new_point: Point = (i as u32, xp as u32);
                new_image[new_point.0 as usize][new_point.1 as usize] = color;
                d += (p1.0-p0.0) as i32;
            }else{
                xp+=increment_x;
                let new_point: Point = (i as u32, xp as u32);
                new_image[new_point.0 as usize][new_point.1 as usize] = color;
                d += ((p1.0-p0.0) + (p1.1-p0.1)) as i32;
            }
            i+=increment_y;
        }
    }
    new_image
}

pub fn calculate_radius(p0:Point, p1:Point) -> i32{
    let delta_x:i32 = (p1.0-p0.0) as i32;
    let delta_y:i32 = (p1.1-p0.1) as i32;
    let radius = ((delta_x.pow(2) + delta_y.pow(2)) as f64).sqrt();
    let int_radius = (radius).floor() as i32;
    int_radius
}

//p0 is the center, p1 is a point which belongs to the circunference
pub fn draw_circunference(image: Image, p0: Point, p1: Point, color: Rgba) -> Image{
    let mut new_image: Image = image.clone();
    let radius = calculate_radius(p0, p1);
    
    for x in -1*radius .. radius{
        let x_circunference:i32 = p0.0 - x;
        let temp_y:i32 = (radius.pow(2) + x.pow(2)).sqrt();
        //reminder: an higher y implies in a lower pixel position
        let y_upper:i32 = p0.1 - temp_y;
        let y_lower:i32 = p0.1 + temp_y;

        //do not draw if the circunference surpass the images boundaries    
        if !(x_circunference > image.len()|| x_circunference < 0 || y_upper < 0 || y_lower > image[0].len()){
            //draws the upper and lower half of the circunference
            let new_point_1: Point = (x_circunference as u32, y_upper as u32);
            new_image[new_point_1.0 as usize][new_point_1.1 as usize] = color;

            let new_point_2: Point = (x_circunference as u32, y_lower as u32);
            new_image[new_point_2.0 as usize][new_point_2.1 as usize] = color;            
        }
    }
    new_image
}

pub fn parametric_circunference(image: Image, p0: Point, p1: Point, color: Rgba) -> Image{
    let mut new_image: Image = image.clone();
    let radius = calculate_radius(p0, p1);
    let mut a:f32 = 0.0;
    let step = 0.01;
    while (a < 6.28){
        let temp_x:f32 = radius*a.cos();
        let temp_y:f32 = radius*a.sin();
        let x_circunference:i32 = p0.0 + temp_x;
        let y_circunference:i32 = p0.1 + temp_y;
        if !(x_circunference > image.len()|| x_circunference < 0 || y_circunference < 0 || y_circunference > image[0].len()){
            let new_point_1: Point = (x_circunference as u32, y_circunference as u32);
            new_image[new_point_1.0 as usize][new_point_1.1 as usize] = color;
        }
        a += step;
    }
}

pub fn bresenham_circunference_algorithm(image: Image, p0: Point, p1: Point, color: Rgba) -> Image{
    let mut new_image: Image = image.clone();
    let radius = calculate_radius(p0, p1);
    //let mut h
    new_image
}