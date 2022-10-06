use crate::common::*;

// use crate::transformations;
// use std::thread;

pub fn draw_line(image: Image, p0: Point, p1: Point, color: Rgba) -> Image {
    let width = image.len();
    let height = image[0].len();
    let mut new_image: Image = image.clone();    
    let mut increment: i32;
    
    
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
        let mut temp_x0 = p1.0;
        while temp_x1 != temp_x0{
            let new_point: Point = (p1.0, (p1.0 as f32 * m)as u32);
            new_image[new_point.0 as usize][new_point.1 as usize] = color;
            //temp_x1 += increment;
        }
    }else{
        let m = ((p1.0 - p0.0)/(p1.1 - p0.1)) as f32;
        if p1.1 > p0.1 {
            increment = -1;
        }else{
            increment = 1;
        }
        let mut temp_y1 = p1.0;
        let mut temp_y0 = p1.0;
        while temp_y1 != temp_y0{
            let new_point: Point = (p1.0, (p1.0 as f32 * m)as u32);
            new_image[new_point.0 as usize][new_point.1 as usize] = color;
            //temp_y1 += increment;
        }
    }
    
    
    new_image
}