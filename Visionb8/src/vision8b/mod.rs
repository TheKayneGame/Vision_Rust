extern crate image;
pub type Vec2d<T> = Vec<Vec<T>>; //2D vector
pub mod img_rgb_mat;
pub mod img_bw_mat;
pub mod img_label_mat;
pub mod img_hsv_mat;
pub mod license_plate;
pub mod dice_count;

pub use img_rgb_mat::ImgMat;
pub use img_bw_mat::ImgBWMat;
pub use img_label_mat::ImgLabelMat;
pub use image::Pixel;
pub use image::GenericImageView;
pub use image::Rgba;