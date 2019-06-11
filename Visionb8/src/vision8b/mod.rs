extern crate image;
pub type Vec2d<T> = Vec<Vec<T>>; //2D vector
pub mod img_mat;
pub mod img_bw_mat;

pub use img_mat::ImgMat;
pub use img_bw_mat::ImgBWMat;




