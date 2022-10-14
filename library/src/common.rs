pub type Rgba = [u8; 4];

pub type Point = (u32, u32);
pub type Color = Rgba;
pub type Image = Vec<Vec<Rgba>>;
// pub type DCTCoefficients = Vec<Vec<f32>>;
pub type Gray = u8;

pub fn rgb2gray(r: u8, g: u8, b: u8) -> Gray {
    ((r as u16 + g as u16 + b as u16) / 3) as Gray
}
