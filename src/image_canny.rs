#![allow(unused_imports)]
#![allow(dead_code)]
use opencv::{
    core::{Size, BORDER_DEFAULT},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::gaussian_blur,
    prelude::*,
};
use crate::opencv_util::imshow_many;

// Canny 边缘检测
pub fn image_canny() {
    let src = imread("young.jpg", IMREAD_COLOR).unwrap();

    imshow_many("Canny", &[&src], false)
}

fn image_gaussian_filter(mat: &Mat) -> Mat {
    let mut gaussian = Mat::default();
    gaussian_blur(&mat, &mut gaussian, Size::new(3, 3), 1., 0., BORDER_DEFAULT).unwrap();
    gaussian
}
