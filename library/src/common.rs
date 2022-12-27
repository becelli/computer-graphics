pub type Rgba = [u8; 4];
pub type Point = (i32, i32);
pub type Image = Vec<Vec<Rgba>>;
pub type Edge = (Point, Point);
//3d point in homogeneous coordinates
pub type HomogeneousPoint = (f64, f64, f64, f64);
// pub type Border = (i32, i32, i32, i32);
pub type HomogeneousEdge = (HomogeneousPoint, HomogeneousPoint);
pub struct Border {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}

pub enum Neighborhood {
    Four,
    Eight,
}
