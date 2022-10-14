use crate::common::*;

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
            /*bresenham algo for first octet
            let mut yp:i32 = p0.1 as i32;
            if delta_y >= 0{
                increment = 1;
            }else{
                increment = -1;
            }
            //first or eighth octet
            let mut d:i32 = (2*(p1.1-p0.1) - (p1.0-p0.0)) as i32;
            for i in p0.0 .. p1.0{
                if d>0{
                    let new_point: Point = (i, yp as u32);
                    new_image[new_point.0 as usize][new_point.1 as usize] = color;
                    d += (p1.1-p0.1) as i32;
                }else{
                    yp+=increment;
                    let new_point: Point = (i, yp as u32);
                    new_image[new_point.0 as usize][new_point.1 as usize] = color;
                    d += ((p1.1-p0.1) + (p1.0-p0.0)) as i32;
                }
            }
            */
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