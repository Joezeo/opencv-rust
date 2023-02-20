#![allow(unused_imports)]
#![allow(dead_code)]
use crate::opencv_util::imshow_many;
use opencv::{
    core::{Size, BORDER_DEFAULT},
    imgcodecs::{imread, IMREAD_COLOR, IMREAD_GRAYSCALE},
    imgproc::{canny, gaussian_blur},
    prelude::*,
};

// Canny 边缘检测
pub fn image_canny() {
    let src = imread("young.jpg", IMREAD_GRAYSCALE).unwrap();

    let mut c1 = Mat::default();
    canny(&src, &mut c1, 80., 150., 3, false).unwrap();
    let mut c2 = Mat::default();
    canny(&src, &mut c2, 50., 100., 3, false).unwrap();

    imshow_many("Canny", &[&src, &c1, &c2], true)
}
